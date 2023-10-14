use crate::errors::TwoFactorError;
use crate::types::{Record, DB};
use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Debug, Serialize, Deserialize, Type)]
pub struct SsoInstitution {
    pub id: String,
    pub name: String,
    pub alias: Vec<String>,
    pub website: Option<String>,
}

#[tauri::command]
#[specta::specta]
pub async fn make_institution_sso(db: DB<'_>, institution_id: &str) -> Result<(), TwoFactorError> {
    db.update::<Option<Record>>(("institution", institution_id))
        .content(("is_sso", true))
        .await?
        .ok_or(TwoFactorError::NotFound(institution_id.to_string()))?;

    Ok(())
}

#[tauri::command]
#[specta::specta]
pub async fn unmake_institution_sso(
    db: DB<'_>,
    institution_id: &str,
) -> Result<(), TwoFactorError> {
    db.update::<Option<Record>>(("institution", institution_id))
        .content(("is_sso", false))
        .await?
        .ok_or(TwoFactorError::NotFound(institution_id.to_string()))?;

    Ok(())
}

#[tauri::command]
#[specta::specta]
pub async fn get_sso_institutions(db: DB<'_>) -> Result<Vec<SsoInstitution>, TwoFactorError> {
    let institutions: Vec<SsoInstitution> = db
        .query("SELECT * FROM institution WHERE is_sso = true")
        .await?
        .take(0)?;

    Ok(institutions)
}
