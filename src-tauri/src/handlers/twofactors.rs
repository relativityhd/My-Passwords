use crate::common::get_user;
use crate::errors::TwoFactorError;
use crate::types::{Record, DB};
use serde::{Deserialize, Serialize};
use specta::Type;
use surrealdb::sql::Thing;

#[derive(Debug, Serialize)]
struct NewTwoFactor {
    name: String,
    device: String,
    user: Thing,
}

#[tauri::command]
#[specta::specta]
pub async fn create_twofactor(
    db: DB<'_>,
    name: &str,
    device: &str,
) -> Result<String, TwoFactorError> {
    let auth = get_user(db.clone()).await?;

    let twofactor: Vec<Record> = db
        .create("twofactor")
        .content(NewTwoFactor {
            name: name.to_string(),
            device: device.to_string(),
            user: auth,
        })
        .await?;

    Ok(twofactor[0].id.id.to_string())
}

#[derive(Debug, Serialize, Deserialize, Type)]
pub struct TwoFactor {
    pub id: String,
    pub name: String,
    pub color: String,
}

#[tauri::command]
#[specta::specta]
pub async fn get_twofactors(db: DB<'_>) -> Result<Vec<TwoFactor>, TwoFactorError> {
    let twofactors = db.select("twofactor").await?;
    Ok(twofactors)
}

#[tauri::command]
#[specta::specta]
pub async fn edit_twofactor(
    db: DB<'_>,
    twofactor_id: &str,
    name: &str,
    device: &str,
) -> Result<String, TwoFactorError> {
    let twofactor = db
        .update::<Option<Record>>(("twofactor", twofactor_id))
        .content(("name", name, "device", device))
        .await?
        .ok_or(TwoFactorError::NotFound(twofactor_id.to_string()))?;
    Ok(twofactor.id.id.to_string())
}

#[tauri::command]
#[specta::specta]
pub async fn delete_twofactor(db: DB<'_>, twofactor_id: &str) -> Result<String, TwoFactorError> {
    let twofactor = db
        .delete::<Option<Record>>(("twofactor", twofactor_id))
        .await?
        .ok_or(TwoFactorError::NotFound(twofactor_id.to_string()))?;

    // TODO: unlink stuff
    Ok(twofactor.id.id.to_string())
}
