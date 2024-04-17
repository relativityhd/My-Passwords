use std::sync::PoisonError;

use serde::Serialize;
use thiserror::Error;
use tokio::sync::oneshot::error;

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("Database error: {0:?}")]
    Db(#[from] surrealdb::Error),
    #[error("IO error: {0:?}")]
    Io(#[from] std::io::Error),
    #[error("PIN Convertion error: {0:?}")]
    PinConvertion(#[from] std::array::TryFromSliceError),
    #[error("The PIN mutex was poisoned")]
    PinPoisonError(String),
    #[error("App data directory not found")]
    AppDataNotFound,
    #[error("Invalid UTF-8 in cookie file: {0:?}")]
    InvalidUtf8(#[from] std::string::FromUtf8Error),
    #[error("Serialization error: {0:?}")]
    Serialization(#[from] serde_json::Error),
    #[error("Encryption error: {0:?}")]
    Encryption(#[from] orion::errors::UnknownCryptoError),
    #[error("No password found in the database")]
    NoPassword,
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

impl<T> From<PoisonError<T>> for AuthError {
    fn from(e: PoisonError<T>) -> Self {
        AuthError::PinPoisonError(e.to_string()).into()
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
    #[error("Database is probably corrupted: No ID was returned from the database.")]
    NoID,
}

#[derive(Debug, Error)]
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
    #[error("The PIN mutex was poisoned")]
    PinPoisonError(String),
    #[error("The PIN was not found")]
    PinNotFound,
    #[error("Database is probably corrupted: No ID was returned from the database.")]
    NoID,
}

impl Serialize for AccountError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

impl<T> From<PoisonError<T>> for AccountError {
    fn from(e: PoisonError<T>) -> Self {
        AccountError::PinPoisonError(e.to_string()).into()
    }
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

#[derive(Debug, Serialize, Error)]
pub enum GenerationError {
    #[error("Unable to generate password.")]
    UnableToGenerate,
}
