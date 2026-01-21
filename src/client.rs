use std::ops::Deref;
use std::sync::{Arc, RwLock};
use std::vec::IntoIter;

use crate::apis::configuration::Configuration;
use crate::apis::{Api, ApiClient};
use crate::auth_provider::{self, AccessTokenWithExpiry, AuthProvider, AuthProviderError};
use chrono::{DateTime, Utc};
use oauth2::basic::BasicTokenType;
use oauth2::http::{Extensions, HeaderValue};
use oauth2::{basic::BasicClient, EndpointNotSet, EndpointSet, TokenResponse};
use oauth2::{
    AuthType, AuthUrl, AuthorizationCode, ClientId, CsrfToken, EmptyExtraTokenFields,
    PkceCodeChallenge, PkceCodeVerifier, RedirectUrl, RefreshToken, Scope, StandardTokenResponse,
    TokenUrl,
};
use reqwest::{Request, Response, StatusCode};
use reqwest_middleware::{ClientBuilder, Extension, Middleware, Next};
use std::collections::HashMap;
use std::error::Error;
use std::fmt::Display;
use url::{OpaqueOrigin, Url};

type Result<T> = std::result::Result<T, TidalClientError>;

static TIDAL_AUTH_URI: &str = "https://login.tidal.com/authorize";
static TIDAL_TOKEN_URI: &str = "https://auth.tidal.com/v1/oauth2/token";

#[derive(Debug)]
pub struct TidalClientError {
    msg: String,
    cause: String,
}

impl Error for TidalClientError {}

impl Display for TidalClientError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.msg, self.cause)
    }
}

pub struct TidalClient {
    api_client: ApiClient,
    oauth_client: oauth2::basic::BasicClient<
        EndpointSet,
        EndpointNotSet,
        EndpointNotSet,
        EndpointNotSet,
        EndpointSet,
    >,
    oauth_http_client: oauth2::reqwest::Client,
}

pub struct Token {
    pub access_token: String,
    pub refresh_token: String,
    pub expiry: String,
}

impl From<StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>> for Token {
    fn from(value: StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>) -> Self {
        let expires_in = value.expires_in().clone().unwrap_or_default();

        let expiry = Utc::now() + expires_in;

        Token {
            access_token: value.access_token().clone().into_secret(),
            refresh_token: value
                .refresh_token()
                .unwrap_or(&oauth2::RefreshToken::new(String::from("")))
                .clone()
                .into_secret(),
            expiry: expiry.to_rfc3339(),
        }
    }
}

// TidalClient configuration. `auth_token` is optional
// so that the client can be used w
pub struct TidalClientConfig {
    pub auth_token: Option<Token>,
    pub oauth_config: OAuthConfig,
}

#[derive(Debug, Clone)]
pub struct OAuthConfig {
    pub redirect_uri: String,
    pub client_id: String,
    pub client_secret: Option<String>,
}

struct AuthTokenRefreshMiddleware {
    auth_provider: crate::auth_provider::AuthProvider,
    http_client: oauth2::reqwest::Client,
    oauth_client: oauth2::basic::BasicClient<
        EndpointSet,
        EndpointNotSet,
        EndpointNotSet,
        EndpointNotSet,
        EndpointSet,
    >,
}

impl AuthTokenRefreshMiddleware {
    async fn refresh_access_token(&self) -> Result<AccessTokenWithExpiry> {
        let refresh_token = self.auth_provider.get_refresh_token();

        let maybe_token = self
            .oauth_client
            .exchange_refresh_token(&refresh_token)
            .request_async(&self.http_client)
            .await;

        let token = match maybe_token {
            Ok(t) => t,
            Err(e) => {
                return Err(TidalClientError {
                    msg: String::from("failed to refresh token"),
                    cause: e.to_string(),
                })
            }
        };

        let Some(expires_in) = token.expires_in() else {
            return Err(TidalClientError {
                msg: String::from("failed to refresh access token"),
                cause: String::from("expires_in was missing"),
            });
        };

        let expiry = Utc::now() + expires_in;

        Ok(AccessTokenWithExpiry {
            access_token: token.access_token().clone(),
            expiry,
        })
    }
}
#[async_trait::async_trait]
impl Middleware for AuthTokenRefreshMiddleware {
    async fn handle(
        &self,
        mut req: Request,
        extensions: &mut Extensions,
        next: Next<'_>,
    ) -> reqwest_middleware::Result<Response> {
        let token = match self.auth_provider.get_access_token() {
            Ok(t) => t.into_secret(),
            Err(AuthProviderError::TokenExpiredError) => {
                let token = match self.refresh_access_token().await {
                    Ok(t) => t,
                    Err(e) => {
                        return Err(reqwest_middleware::Error::middleware(TidalClientError {
                            msg: String::from("failed to refresh access token"),
                            cause: e.to_string(),
                        }))
                    }
                };

                match self.auth_provider.update_access_token(token.clone()) {
                    Ok(_) => {}
                    Err(e) => {
                        return Err(reqwest_middleware::Error::middleware(TidalClientError {
                            msg: String::from("failed to set new access token"),
                            cause: e.to_string(),
                        }))
                    }
                };

                token.access_token.into_secret()
            }
            Err(e) => {
                return Err(reqwest_middleware::Error::middleware(TidalClientError {
                    msg: String::from("failed to get access token from auth provider"),
                    cause: e.to_string(),
                }))
            }
        };

        let header = match HeaderValue::from_str(format!("Bearer {}", token).as_str()) {
            Ok(h) => h,
            Err(e) => {
                return Err(reqwest_middleware::Error::middleware(TidalClientError {
                    msg: String::from("failed to get access token from auth provider"),
                    cause: e.to_string(),
                }))
            }
        };

        req.headers_mut().insert("Authorization", header);

        next.run(req, extensions).await
    }
}

impl Deref for TidalClient {
    type Target = ApiClient;
    fn deref(&self) -> &Self::Target {
        &self.api_client
    }
}

fn to_scopes(scopes: Vec<&str>) -> Vec<Scope> {
    return scopes
        .into_iter()
        .map(|s| -> Scope { Scope::new(s.to_string()) })
        .collect();
}

impl TidalClient {
    pub fn new(config: TidalClientConfig) -> Result<Self> {
        let oauth_http_client = match oauth2::reqwest::ClientBuilder::new()
            .redirect(oauth2::reqwest::redirect::Policy::none())
            .build()
        {
            Ok(cb) => cb,
            Err(e) => {
                return Err(TidalClientError {
                    msg: String::from("failed to build oauth http client"),
                    cause: e.to_string(),
                })
            }
        };

        let redirect_url = match RedirectUrl::new(config.oauth_config.redirect_uri.to_string()) {
            Ok(r) => r,
            Err(e) => {
                return Err(TidalClientError {
                    msg: String::from("failed to parse redirect uri"),
                    cause: e.to_string(),
                })
            }
        };

        let auth_url = match AuthUrl::new(TIDAL_AUTH_URI.to_string()) {
            Ok(a) => a,
            Err(e) => {
                return Err(TidalClientError {
                    msg: String::from("failed to parse auth uri"),
                    cause: e.to_string(),
                })
            }
        };

        let token_url = match TokenUrl::new(TIDAL_TOKEN_URI.to_string()) {
            Ok(a) => a,
            Err(e) => {
                return Err(TidalClientError {
                    msg: e.to_string(),
                    cause: e.to_string(),
                });
            }
        };

        let oauth_client = oauth2::basic::BasicClient::new(ClientId::new(
            config.oauth_config.client_id.to_string(),
        ))
        .set_redirect_uri(redirect_url)
        .set_auth_uri(auth_url)
        .set_token_uri(token_url);

        let oauth_client = match config.oauth_config.client_secret {
            Some(client_secret) => {
                oauth_client.set_client_secret(oauth2::ClientSecret::new(client_secret))
            }
            None => oauth_client,
        };

        let api_http_client = reqwest::Client::new();

        let api_client = match config.auth_token {
            Some(auth_token) => {
                let expiry = match DateTime::parse_from_rfc3339(&auth_token.expiry) {
                    Ok(e) => e,
                    Err(e) => {
                        return Err(TidalClientError {
                            msg: String::from("failed to parse expiry"),
                            cause: e.to_string(),
                        })
                    }
                }
                .to_utc();

                let auth_provider = AuthProvider {
                    access_token: Arc::new(RwLock::new(AccessTokenWithExpiry {
                        access_token: oauth2::AccessToken::new(auth_token.access_token),
                        expiry,
                    })),
                    refresh_token: oauth2::RefreshToken::new(auth_token.refresh_token),
                };

                let refresh_middleware = AuthTokenRefreshMiddleware {
                    auth_provider,
                    http_client: oauth_http_client.clone(),
                    oauth_client: oauth_client.clone(),
                };

                let middleware_client = ClientBuilder::new(api_http_client)
                    .with(refresh_middleware)
                    .build();

                ApiClient::new(Arc::new(Configuration {
                    client: middleware_client,
                    oauth_access_token: None,
                    ..Default::default()
                }))
            }
            None => {
                let middleware_client = ClientBuilder::new(api_http_client).build();
                ApiClient::new(Arc::new(Configuration {
                    client: middleware_client,
                    ..Default::default()
                }))
            }
        };

        return Ok(TidalClient {
            api_client: api_client,
            oauth_client,
            oauth_http_client,
        });
    }

    /// Generate a PKCE challenge code and verifier.
    pub fn generate_pkce_challenge_and_verifier(&self) -> (PkceCodeChallenge, PkceCodeVerifier) {
        oauth2::PkceCodeChallenge::new_random_sha256()
    }

    /// Get the /authorize URL given a code challenge and scopes, returns
    /// URL as well as stringified CSRF state token
    ///
    /// Caller must generate pkce challenge code and verifier separately
    /// because they should be used in subsequent requests.
    pub fn get_authorize_url_and_state(
        &self,
        code_challenge: PkceCodeChallenge,
        scopes: Vec<&str>,
    ) -> (String, String) {
        let (url, csrf_token) = self
            .oauth_client
            .authorize_url(CsrfToken::new_random)
            .set_pkce_challenge(code_challenge)
            .add_scopes(to_scopes(scopes))
            .url();

        return (url.to_string(), csrf_token.into_secret());
    }

    /// Exchanges client_id and client_secret for an access token.
    pub async fn exchange_client_credentials_for_token(&self, scopes: Vec<&str>) -> Result<Token> {
        match self
            .oauth_client
            .exchange_client_credentials()
            .add_scopes(to_scopes(scopes))
            .request_async(&self.oauth_http_client)
            .await
        {
            Ok(resp) => Ok(resp.into()),
            Err(e) => Err(TidalClientError {
                msg: String::from("failed to exchange client credentials for token"),
                cause: e.to_string(),
            }),
        }
    }

    /// Calls the /token URL and returns an access and refresh token if the
    /// request is successful
    pub async fn exchange_code_for_token(
        &self,
        code_verifier: String,
        code: String,
    ) -> Result<Token> {
        let verifier = PkceCodeVerifier::new(code_verifier);
        let auth_code = AuthorizationCode::new(code);
        let resp = self
            .oauth_client
            .exchange_code(auth_code)
            .set_pkce_verifier(verifier)
            .request_async(&self.oauth_http_client)
            .await;

        let token_resp = match resp {
            Ok(t) => t,
            Err(e) => {
                return Err(TidalClientError {
                    msg: String::from("failed to exchange auth code"),
                    cause: e.to_string(),
                })
            }
        };

        let Some(refresh_token) = token_resp.refresh_token() else {
            return Err(TidalClientError {
                msg: String::from("failed to exchange auth code"),
                cause: String::from("response missing refresh token"),
            });
        };

        let Some(expires_in) = token_resp.expires_in() else {
            return Err(TidalClientError {
                msg: String::from("failed to exchange auth code"),
                cause: String::from("response missing expires_in"),
            });
        };

        let expiry = Utc::now() + expires_in;

        Ok(Token {
            access_token: token_resp.access_token().clone().into_secret(),
            refresh_token: refresh_token.clone().into_secret(),
            expiry: expiry.to_rfc3339(),
        })
    }
}
