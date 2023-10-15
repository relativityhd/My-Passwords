use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("Database error: {0:?}")]
    Db(#[from] surrealdb::Error),
    #[error("IO error: {0:?}")]
    Io(#[from] std::io::Error),
    #[error("App data directory not found")]
    AppDataNotFound,
    #[error("Invalid UTF-8 in cookie file: {0:?}")]
    InvalidUtf8(#[from] std::string::FromUtf8Error),
    #[error("Not signed in")]
    NotSignedIn,
}

impl Serialize for AuthError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

#[derive(Debug, Serialize, Error)]
pub enum BucketError {
    #[error("Database error: {0:?}")]
    Db(#[from] surrealdb::Error),
    #[error("Authentification error: {0:?}")]
    Auth(#[from] AuthError),
    #[error("Invalid Color: {0:?}, must be 6-digit hex code, e.g. \"#ff0000\"")]
    InvalidColor(String),
    #[error("No Bucket found with id bucket:{0}")]
    NotFound(String),
}

#[derive(Debug, Serialize, Error)]
pub enum AccountError {
    #[error("Database error: {0:?}")]
    Db(#[from] surrealdb::Error),
    #[error("Authentification error: {0:?}")]
    Auth(#[from] AuthError),
    #[error("No account found with id account:{0}")]
    AccountNotFound(String),
    #[error("No account found with id secure_account:{0}")]
    SecureAccountNotFound(String),
    #[error("No account found with id super_secure_account:{0}")]
    SuperSecureAccountNotFound(String),
    #[error("No account found with id sso_account:{0}")]
    SsoAccountNotFound(String),
    #[error("Database is probably corrupted: No Bucket found with id bucket:{0}")]
    CorruptedBucket(String),
    #[error("Database is probably corrupted: No Institution found with id institution:{0}")]
    CorruptedInstitution(String),
    #[error("Database is probably corrupted: No TwoFactor Authentification found with id two_factor:{0}")]
    CorruptedTwoFactor(String),
}

#[derive(Debug, Serialize, Error)]
pub enum TwoFactorError {
    #[error("Database error: {0:?}")]
    Db(#[from] surrealdb::Error),
    #[error("Authentification error: {0:?}")]
    Auth(#[from] AuthError),
    #[error("No TwoFactor Authentification found with id twofactor:{0}")]
    NotFound(String),
}

#[derive(Debug, Serialize, Error)]
pub enum SsoError {
    #[error("Database error: {0:?}")]
    Db(#[from] surrealdb::Error),
    #[error("Authentification error: {0:?}")]
    Auth(#[from] AuthError),
    #[error("No SSO Institution found with id institution:{0}")]
    NotFound(String),
}
