use chrono::{DateTime, Utc};
use oauth2::{AccessToken, RefreshToken};
use std::error::Error;
use std::fmt::Display;
use std::result::Result as StdResult;
use std::sync::{Arc, Mutex, MutexGuard, RwLock, TryLockError};

#[derive(Clone)]
pub(crate) struct AccessTokenWithExpiry {
    pub(crate) access_token: AccessToken,
    pub(crate) expiry: DateTime<Utc>,
}

pub(crate) struct AuthProvider {
    pub(crate) access_token: Arc<RwLock<AccessTokenWithExpiry>>,
    pub(crate) refresh_token: RefreshToken,
}

#[derive(Debug)]
pub(crate) enum AuthProviderError {
    TokenExpiredError,
    LockError { cause: String },
}

impl Display for AuthProviderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            AuthProviderError::TokenExpiredError => String::from("access token expired"),
            AuthProviderError::LockError { cause } => {
                format!("failed to acquire lock on token: {}", cause)
            }
        };

        write!(f, "auth provider encountered error: {}", msg)
    }
}

impl Error for AuthProviderError {}

type Result<T> = StdResult<T, AuthProviderError>;

impl AuthProvider {
    pub(crate) fn get_refresh_token(&self) -> RefreshToken {
        self.refresh_token.clone()
    }

    pub(crate) fn get_access_token(&self) -> Result<AccessToken> {
        let locked_token = match self.access_token.read() {
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

    pub(crate) fn update_access_token(&self, new: AccessTokenWithExpiry) -> Result<()> {
        let mut locked_token = match self.access_token.write() {
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
}
