use async_trait::async_trait;
use chrono::{DateTime, Utc};
use oauth2::basic::BasicTokenType;
use oauth2::{
    AccessToken, AsyncHttpClient, EmptyExtraTokenFields, ErrorResponse, ExtraTokenFields,
    RefreshToken, RequestTokenError, Scope, StandardTokenResponse, TokenResponse, TokenType,
};
use serde::Serialize;
use std::error::Error;
use std::fmt::{Debug, Display};
use std::result::Result as StdResult;
use std::sync::{Arc, Mutex, MutexGuard, RwLock, TryLockError};

use crate::client::OAuthClient;

type HttpClient = oauth2::reqwest::Client;

#[async_trait]
pub(crate) trait AuthProvider {
    async fn refresh_access_token<'a>(
        &self,
        oauth_client: &'a OAuthClient,
        http_client: &'a HttpClient,
    ) -> Result<AccessTokenWithExpiry>;
    fn get_access_token(&self) -> Result<AccessToken>;
    fn update_access_token(&self, new: AccessTokenWithExpiry) -> Result<()>;
}

#[derive(Clone)]
pub(crate) struct AccessTokenWithExpiry {
    pub(crate) access_token: AccessToken,
    pub(crate) expiry: DateTime<Utc>,
}

#[derive(Clone)]
pub(crate) struct AuthProviderRefreshToken {
    pub(crate) access_token: Arc<RwLock<AccessTokenWithExpiry>>,
    pub(crate) refresh_token: RefreshToken,
}

#[derive(Debug)]
pub(crate) enum AuthProviderError {
    TokenExpiredError,
    LockError { cause: String },
    RefreshError { cause: String },
}

impl Display for AuthProviderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            AuthProviderError::TokenExpiredError => String::from("access token expired"),
            AuthProviderError::LockError { cause } => {
                format!("failed to acquire lock on token: {}", cause)
            }
            AuthProviderError::RefreshError { cause } => {
                format!("failed to refresh access token: {}", cause)
            }
        };

        write!(f, "auth provider encountered error: {}", msg)
    }
}

impl Error for AuthProviderError {}

type Result<T> = StdResult<T, AuthProviderError>;

#[async_trait]
impl AuthProvider for AuthProviderRefreshToken {
    async fn refresh_access_token<'a>(
        &self,
        oauth_client: &'a OAuthClient,
        http_client: &'a HttpClient,
    ) -> Result<AccessTokenWithExpiry> {
        let maybe_token = oauth_client
            .exchange_refresh_token(&self.refresh_token)
            .request_async(http_client)
            .await;

        let token = match maybe_token {
            Ok(t) => t,
            Err(e) => {
                return Err(AuthProviderError::RefreshError {
                    cause: e.to_string(),
                })
            }
        };

        token_response_to_access_token_with_expiry(token)
    }

    fn get_access_token(&self) -> Result<AccessToken> {
        get_access_token(self.access_token.clone())
    }

    fn update_access_token(&self, new: AccessTokenWithExpiry) -> Result<()> {
        update_access_token(self.access_token.clone(), new)
    }
}

fn get_access_token(access_token: Arc<RwLock<AccessTokenWithExpiry>>) -> Result<AccessToken> {
    let locked_token = match access_token.read() {
        Ok(a) => a,
        Err(e) => {
            return Err(AuthProviderError::LockError {
                cause: e.to_string(),
            })
        }
    };

    let now = Utc::now();

    if now > locked_token.expiry {
        return Err(AuthProviderError::TokenExpiredError);
    }

    return Ok(locked_token.access_token.clone());
}

fn update_access_token(
    to_update: Arc<RwLock<AccessTokenWithExpiry>>,
    new: AccessTokenWithExpiry,
) -> Result<()> {
    let mut locked_token = match to_update.write() {
        Ok(a) => a,
        Err(e) => {
            return Err(AuthProviderError::LockError {
                cause: e.to_string(),
            })
        }
    };

    *locked_token = new;

    Ok(())
}

fn token_response_to_access_token_with_expiry(
    token: StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>,
) -> Result<AccessTokenWithExpiry> {
    let Some(expires_in) = token.expires_in() else {
        return Err(AuthProviderError::RefreshError {
            cause: String::from("expires_in was missing"),
        });
    };

    let expiry = Utc::now() + expires_in;

    Ok(AccessTokenWithExpiry {
        access_token: token.access_token().clone(),
        expiry,
    })
}

pub(crate) struct AuthProviderClientCredentials {
    pub(crate) access_token: Arc<RwLock<AccessTokenWithExpiry>>,
    pub(crate) scopes: Vec<Scope>,
}

#[async_trait]
impl AuthProvider for AuthProviderClientCredentials {
    async fn refresh_access_token<'a>(
        &self,
        oauth_client: &'a OAuthClient,
        http_client: &'a HttpClient,
    ) -> Result<AccessTokenWithExpiry> {
        let maybe_token = oauth_client
            .exchange_client_credentials()
            .add_scopes(self.scopes.clone())
            .request_async(http_client)
            .await;

        let token = match maybe_token {
            Ok(t) => t,
            Err(e) => {
                return Err(AuthProviderError::RefreshError {
                    cause: e.to_string(),
                })
            }
        };

        token_response_to_access_token_with_expiry(token)
    }

    fn get_access_token(&self) -> Result<AccessToken> {
        get_access_token(self.access_token.clone())
    }

    fn update_access_token(&self, new: AccessTokenWithExpiry) -> Result<()> {
        update_access_token(self.access_token.clone(), new)
    }
}
