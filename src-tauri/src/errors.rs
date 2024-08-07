use std::fmt::format;

use serde::Serialize;
use thiserror::Error;
use tokio::sync::oneshot::error;

/**
 * A serialized error contains a status and an message.
 * The error codes follow the HTTP status codes (4XX and 5XX) (https://developer.mozilla.org/en-US/docs/Web/HTTP/Status).
 *
 * Note that this SerializedError is just an intermediate struct to serialize the error.
 *
 * There are two types of errors:
 * - 500: Unexpected errors - meant to be written to a log file and shown on an error page
 * - XXX: Expected errors - meant to be shown to the user in a dialog box
 *
 */
#[derive(Debug, Serialize)]
pub struct SerializedError {
    pub status: u16,
    pub message: String,
}

// -- Auth related errors --
#[derive(Debug, Error)]
pub enum UnauthenticatedError {
    #[error("Not signed in.")]
    NotSignedIn,
    #[error("Database error: {0:?}")]
    Db(#[from] surrealdb::Error),
}

impl Serialize for UnauthenticatedError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let serializer_error = match self {
            UnauthenticatedError::NotSignedIn => SerializedError {
                status: 401,
                message: self.to_string(),
            },
            _ => SerializedError {
                status: 500,
                message: self.to_string(),
            },
        };
        serializer_error.serialize(serializer)
    }
}

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("Invalid credentials.")]
    InvalidCredentials,
    #[error("User already exists.")]
    UserExists,
    #[error("Cookie error: {0:?}")]
    Cookie(#[from] CookieError),
    #[error("Database error: {0:?}")]
    Db(#[from] surrealdb::Error),
}

impl Serialize for AuthError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let serializer_error = match self {
            AuthError::InvalidCredentials | AuthError::UserExists => SerializedError {
                status: 400,
                message: self.to_string(),
            },
            _ => SerializedError {
                status: 500,
                // Also add dbg message
                message: self.to_string(),
            },
        };
        serializer_error.serialize(serializer)
    }
}

// Note: This error is NOT front-facing, so it does not need to be serialized.
#[derive(Debug, Error)]
pub enum CookieError {
    #[error("App data directory not found")]
    AppDataNotFound,
    #[error("Cookie not found")]
    Io(#[from] std::io::Error),
    #[error("Invalid UTF-8 in cookie file: {0:?}")]
    InvalidUtf8(#[from] std::string::FromUtf8Error),
}

#[derive(Debug, Error)]
pub enum LocalCredsError {
    #[error("No password found for user in the database - database is probably corrupted.")]
    NoPasswordFound,
    #[error("App data directory not found")]
    AppDataNotFound,
    #[error("Unauthenticated error: {0:?}")]
    Unauthenticated(#[from] UnauthenticatedError),
    #[error("IO error: {0:?}")]
    Io(#[from] std::io::Error),
    #[error("PIN Convertion error: {0:?}")]
    PinConvertion(#[from] std::array::TryFromSliceError),
    #[error("Serialization error: {0:?}")]
    Serialization(#[from] serde_json::Error),
    #[error("Encryption error: {0:?}")]
    Encryption(#[from] orion::errors::UnknownCryptoError),
    #[error("Database error: {0:?}")]
    Db(#[from] surrealdb::Error),
}

impl Serialize for LocalCredsError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let serializer_error = SerializedError {
            status: 500,
            message: self.to_string(),
        };
        serializer_error.serialize(serializer)
    }
}

// -- Bucket related errors --

#[derive(Debug, Error)]
pub enum BucketError {
    #[error("Invalid Color: {0:?}, must be 6-digit hex code, e.g. \"#ff0000\"")]
    InvalidColor(String),
    #[error("No Bucket found with id bucket:{0}")]
    NotFound(String),
    #[error("Database is probably corrupted: No ID was returned from the database.")]
    NoID,
    #[error("Database error: {0:?}")]
    Db(#[from] surrealdb::Error),
}

impl Serialize for BucketError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let serializer_error = match self {
            BucketError::InvalidColor(e) => SerializedError {
                status: 400,
                message: self.to_string(),
            },
            BucketError::NotFound(e) => SerializedError {
                status: 404,
                message: self.to_string(),
            },
            _ => SerializedError {
                status: 500,
                message: self.to_string(),
            },
        };
        serializer_error.serialize(serializer)
    }
}

// -- Account / Passwords related errors --

#[derive(Debug, Error)]
pub enum AccountError {
    #[error("Password could not be generated:{0}")]
    PasswordGeneration(#[from] GenerationError),

    #[error("The PIN was not found. Try to restart the App.")]
    PinNotFound,

    #[error("No ID was returned from the database. The database is probably corrupted.")]
    NoID,
    #[error("No account found with id account:{0}. The database is probably corrupted.")]
    AccountNotFound(String),
    #[error("Database error: {0:?}")]
    Db(#[from] surrealdb::Error),
}

impl Serialize for AccountError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let serializer_error = match self {
            AccountError::PasswordGeneration(_) => SerializedError {
                status: 400,
                message: self.to_string(),
            },
            AccountError::PinNotFound | AccountError::AccountNotFound(_) | AccountError::NoID => {
                SerializedError {
                    status: 404,
                    message: self.to_string(),
                }
            }
            _ => SerializedError {
                status: 500,
                message: self.to_string(),
            },
        };
        serializer_error.serialize(serializer)
    }
}

#[derive(Debug, Error)]
#[error("Password generation error")]
pub struct GenerationError;

impl Serialize for GenerationError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        SerializedError {
            status: 400,
            message: self.to_string(),
        }
        .serialize(serializer)
    }
}

// -- Database related errors --

#[derive(Debug, Error)]
pub enum DatabaseError {
    #[error("Invalid URL.")]
    InvalidUrl,
    #[error("Version mismatch. Expected: {expected:?}, found: {found:?}")]
    VersionMismatch { expected: String, found: String },
    #[error("No version found in database. It is probably not meant to be used with this app.")]
    NoVersion,
    #[error("Database error: {0:?}")]
    Db(#[from] surrealdb::Error),
    #[error("IO error: {0:?}")]
    Io(#[from] std::io::Error),
    #[error("Invalid UTF-8 in db url file: {0:?}")]
    InvalidUtf8(#[from] std::string::FromUtf8Error),
    #[error("App data directory not found")]
    AppDataNotFound,
}

impl Serialize for DatabaseError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let serializer_error = match self {
            DatabaseError::InvalidUrl
            | DatabaseError::VersionMismatch { .. }
            | DatabaseError::NoVersion
            | DatabaseError::Db(_) => SerializedError {
                status: 400,
                message: self.to_string(),
            },
            _ => SerializedError {
                status: 500,
                message: self.to_string(),
            },
        };
        serializer_error.serialize(serializer)
    }
}
