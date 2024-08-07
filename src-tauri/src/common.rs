use crate::errors::{AccountError, UnauthenticatedError};
use crate::types::{LocalCreds, DB, LC};
use surrealdb::engine::remote::ws::Client;
use surrealdb::sql::Thing;
use surrealdb::Surreal;

pub async fn get_user(db: DB<'_>) -> Result<Thing, UnauthenticatedError> {
    db.query("RETURN $auth;")
        .await?
        .take::<Option<Thing>>(0)?
        .ok_or(UnauthenticatedError::NotSignedIn)
}

pub async fn extract_lc(lc: &LC<'_>) -> Result<LocalCreds, AccountError> {
    let state = lc.lock().unwrap();
    state.clone().ok_or(AccountError::PinNotFound)
}
