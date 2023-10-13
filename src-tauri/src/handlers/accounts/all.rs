use super::types::AccountError;
use crate::algorithm::gen_pw;
use crate::handlers::auth::AuthError;
use crate::types::Industry;
use serde::{Deserialize, Serialize};
use specta::Type;
use surrealdb::engine::remote::ws::Client;
use surrealdb::sql::Thing;
use surrealdb::Surreal;

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct Institution {
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct Bucket {
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct Account {
    pub id: String,
    pub identity: String,
    pub institution: Institution,
    pub bucket: Option<Bucket>,
}

#[tauri::command]
#[specta::specta]
async fn search(
    db: tauri::State<'_, Surreal<Client>>,
    search_term: &str,
) -> Result<Vec<Account>, AccountError> {
    Ok(db
        .query("SELECT id, identity, institution.name bucket.name FROM account WHERE institution.name ~ $query")
        .bind(("query", search_term))
        .await?
        .take::<Vec<Account>>(0)?)
}
