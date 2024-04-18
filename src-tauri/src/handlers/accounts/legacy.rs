use crate::algorithm::gen_legacy_pw;
use crate::errors::AccountError;
use crate::types::{
    extract_lc,
    handlers::{AccountMetadata, SecureOverview, SecureSpecifics},
    Industry, LocalCreds, DB, LC,
};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Deserialize, Serialize)]
struct LegacyPasswordData {
    institution: String,
    industry: Industry,
}

#[tauri::command]
#[specta::specta]
pub async fn get_legacy_password(db: DB<'_>, lc: LC<'_>, id: &str) -> Result<String, AccountError> {
    let local_creds = extract_lc(&lc).await?;
    let secret = local_creds.secret;
    // Convert id to Record
    let sql = "SELECT institution,
        (->is_legacy->legacy_account.industry)[0] as industry
        FROM ONLY type::thing('account', $account);";
    let id = id.split(':').last().unwrap();
    let data = db
        .query(sql)
        .bind(("account", id))
        .await?
        .take::<Option<LegacyPasswordData>>(0)?
        .ok_or(AccountError::AccountNotFound(id.to_string()))?;
    let pw = gen_legacy_pw(&data.institution, &data.industry, &secret.to_string());
    Ok(pw)
}

#[tauri::command]
#[specta::specta]
pub async fn get_legacy_overview(
    db: DB<'_>,
    lc: LC<'_>,
    id: &str,
) -> Result<(SecureOverview, String), AccountError> {
    let local_creds = extract_lc(&lc).await?;
    let secret = local_creds.secret;

    let sql = "SELECT
        institution,
        recovery,
        website,
        alias,
        account_type as mode,
        type::string(created) as created,
        (->is_legacy->legacy_account.industry)[0] as industry,
        (SELECT
            type::string(id) as id,
            color,
            name,
            array::len(<-is_sorted_in<-account) as n
            FROM (->is_sorted_in->bucket))[0] as bucket,
        (SELECT type::string(id) as id, name, device FROM (->is_secured_by->twofactor))[0] as twofactor
        FROM ONLY type::thing('account', $account);";
    let id = id.split(':').last().unwrap();
    let data = db
        .query(sql)
        .bind(("account", id))
        .await?
        .take::<Option<SecureOverview>>(0)?
        .ok_or(AccountError::AccountNotFound(id.to_string()))?;
    let pw = gen_legacy_pw(&data.institution, &data.industry, &secret.to_string());

    Ok((data, pw))
}
