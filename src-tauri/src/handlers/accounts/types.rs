use crate::types::InvalidModeError;
use sea_orm::*;
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AccountError {
    #[error("Database error from SeaORM: {0:?}")]
    Db(#[from] DbErr),
    #[error("No account found with id {0}")]
    NotFound(i32),
    #[error("The account doesn't belong to the current user")]
    NotAuthorized,
    #[error("Database is probably corrupted: No Bucket found with id {0}")]
    CorruptedBucket(i32),
    #[error("Database is probably corrupted: No Institution found with id {0}")]
    CorruptedInstitution(i32),
    #[error("Database is probably corrupted: {0:?}")]
    CorruptedMode(#[from] InvalidModeError),
}

impl Serialize for AccountError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}
