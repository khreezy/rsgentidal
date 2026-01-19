use std::ops::Deref;
use std::sync::Arc;

use crate::apis::configuration::Configuration;
use crate::apis::{Api, ApiClient};
use chrono::Utc;
use oauth2::http::{Extensions, HeaderValue};
use oauth2::{
    basic::BasicClient, EndpointNotSet, EndpointSet, StandardRevocableToken::RefreshToken,
    TokenResponse,
};
use oauth2::{
    AuthType, AuthUrl, AuthorizationCode, ClientId, CsrfToken, PkceCodeChallenge, PkceCodeVerifier,
    RedirectUrl, Scope, TokenUrl,
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
    http_client: oauth2::reqwest::Client,
    oauth_client: oauth2::basic::BasicClient<
        EndpointSet,
        EndpointNotSet,
        EndpointNotSet,
        EndpointNotSet,
        EndpointSet,
    >,
    refresh_token: String,
}

#[async_trait::async_trait]
impl Middleware for AuthTokenRefreshMiddleware {
    async fn handle(
        &self,
        req: Request,
        extensions: &mut Extensions,
        next: Next<'_>,
    ) -> reqwest_middleware::Result<Response> {
        let maybe_req_copy = req.try_clone();

        let res = next.clone().run(req, extensions).await;

        let mut req_copy = match maybe_req_copy {
            Some(r) => r,
            None => return res,
        };

        match res {
            Ok(r) => {
                if r.status().clone() == oauth2::http::StatusCode::UNAUTHORIZED {
                    // Technically, there is a sub_status in the body that indicates our token is
                    // expired, but  if we consume the body to check we can no longer return the body
                    // if that is not the case. So instead, optimistically assume if you provided a refresh token
                    // that we can go through that flow if we get a 403.
                    let maybe_token = self
                        .oauth_client
                        .exchange_refresh_token(&oauth2::RefreshToken::new(
                            self.refresh_token.clone(),
                        ))
                        .request_async(&self.http_client)
                        .await;

                    let token = match maybe_token {
                        Ok(t) => t,
                        Err(_) => return Ok(r),
                    };

                    let header = match HeaderValue::from_str(
                        format!("Bearer {:?}", token.access_token()).as_str(),
                    ) {
                        Ok(h) => h,
                        Err(_) => return Ok(r),
                    };

                    req_copy.headers_mut().insert("Authorization", header);

                    next.run(req_copy, extensions).await
                } else {
                    Ok(r)
                }
            }
            Err(e) => Err(e),
        }
    }
}

impl Deref for TidalClient {
    type Target = ApiClient;
    fn deref(&self) -> &Self::Target {
        &self.api_client
    }
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

        let api_http_client = reqwest::Client::new();

        let api_client = match config.auth_token {
            Some(auth_token) => {
                let refresh_middleware = AuthTokenRefreshMiddleware {
                    http_client: oauth_http_client.clone(),
                    oauth_client: oauth_client.clone(),
                    refresh_token: auth_token.refresh_token,
                };

                let middleware_client = ClientBuilder::new(api_http_client)
                    .with(refresh_middleware)
                    .build();

                ApiClient::new(Arc::new(Configuration {
                    client: middleware_client,
                    oauth_access_token: Some(auth_token.access_token),
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
        let scopes = scopes
            .into_iter()
            .map(|scope| -> Scope { Scope::new(scope.to_string()) });

        let (url, csrf_token) = self
            .oauth_client
            .authorize_url(CsrfToken::new_random)
            .set_pkce_challenge(code_challenge)
            .add_scopes(scopes)
            .url();

        return (url.to_string(), csrf_token.into_secret());
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
