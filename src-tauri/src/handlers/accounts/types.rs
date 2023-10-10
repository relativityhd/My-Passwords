use crate::handlers::auth::AuthError;
use serde::Serialize;
use surrealdb::sql::{Id, Thing};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AccountError {
    #[error("Database error: {0:?}")]
    Db(#[from] surrealdb::Error),
    #[error("No account found with id {0}")]
    NotFound(Thing),
    #[error("The account doesn't belong to the current user")]
    NotAuthorized,
    #[error("Database is probably corrupted: No Bucket found with id {0}")]
    CorruptedBucket(Thing),
    #[error("Database is probably corrupted: No Institution found with id {0}")]
    CorruptedInstitution(Thing),
    #[error("Authentification error: {0:?}")]
    Auth(#[from] AuthError),
}

impl Serialize for AccountError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}
