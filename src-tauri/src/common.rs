use crate::errors::AuthError;
use surrealdb::engine::remote::ws::Client;
use surrealdb::sql::Thing;
use surrealdb::Surreal;

pub async fn get_user(db: tauri::State<'_, Surreal<Client>>) -> Result<Thing, AuthError> {
    db.query("RETURN $auth;")
        .await?
        .take::<Option<Thing>>(0)?
        .ok_or(AuthError::NotSignedIn)
}
