use crate::algorithm::gen_pw;
use crate::errors::AccountError;
use crate::types::{Industry, DB, PIN};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct SecurePasswordData {
    institution: String,
    industry: Industry,
    identity: String,
}

#[tauri::command]
#[specta::specta]
pub async fn secure_live_input(
    pin: PIN<'_>,
    institution: &str,
    identity: &str,
    industry: Industry,
) -> Result<String, AccountError> {
    let secret = pin.lock().await.val.ok_or(AccountError::PinNotFound)?;
    let pw = gen_pw(institution, &industry, &secret.to_string(), identity);
    Ok(pw)
}

#[tauri::command]
#[specta::specta]
pub async fn get_secure_password(
    db: DB<'_>,
    pin: PIN<'_>,
    id: &str,
) -> Result<String, AccountError> {
    let secret = pin.lock().await.val.ok_or(AccountError::PinNotFound)?;
    // Convert id to Record
    let sql = "SELECT institution,
        (->is_secure->secure_account.industry)[0] as industry,
        (->is_secure->secure_account.identity)[0] as identity
        FROM ONLY type::thing($account);";
    let data = db
        .query(sql)
        .bind(("account", id))
        .await?
        .take::<Option<SecurePasswordData>>(0)?
        .ok_or(AccountError::AccountNotFound(id.to_string()))?;
    let pw = gen_pw(
        &data.institution,
        &data.industry,
        &secret.to_string(),
        &data.identity,
    );
    Ok(pw)
}
