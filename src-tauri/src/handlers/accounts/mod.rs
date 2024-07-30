use crate::errors::AccountError;
use crate::types::{
    handlers::{ListResult, PopularResult, SearchResult},
    Mode, DB,
};
pub mod legacy;
pub mod secure;
pub mod sso;
pub mod supersecure;

async fn add_call(db: DB<'_>, id: &str) -> Result<(), AccountError> {
    let sql = "UPDATE ONLY type::thing('account', $account) SET calls += 1;";
    // Expects the id already splitted...
    db.query(sql).bind(("account", id)).await?;
    Ok(())
}

#[tauri::command]
#[specta::specta]
pub async fn get_mode(db: DB<'_>, id: &str) -> Result<Mode, AccountError> {
    let sql = "(SELECT account_type as mode FROM ONLY type::thing('account', $account)).mode;";
    let id = id.split(':').last().unwrap();
    let mode: Option<Mode> = db.query(sql).bind(("account", id)).await?.take(0)?;
    mode.ok_or(AccountError::AccountNotFound(id.to_string()))
}

#[tauri::command]
#[specta::specta]
pub async fn in_sso_use(db: DB<'_>, id: &str) -> Result<bool, AccountError> {
    let sql = "RETURN array::len(
        SELECT in, (<-account.archived)[0] as archived
        FROM use_sso_of
        WHERE out = type::thing('account', $account)
        AND !archived);";
    let id = id.split(':').last().unwrap();
    let n = db
        .query(sql)
        .bind(("account", id))
        .await?
        .take::<Option<u32>>(0)?
        .ok_or(AccountError::AccountNotFound(id.to_string()))?;
    Ok(n > 0)
}

#[tauri::command]
#[specta::specta]
pub async fn delete_account(db: DB<'_>, id: &str) -> Result<(), AccountError> {
    let sql = "fn::delete_account(type::thing('account', $account));";
    let id = id.split(':').last().unwrap();
    db.query(sql)
        .bind(("account", id))
        .await?
        .take::<Option<()>>(0)?;
    Ok(())
}

#[tauri::command]
#[specta::specta]
pub async fn get_all_accounts(db: DB<'_>) -> Result<Vec<ListResult>, AccountError> {
    let sql = "SELECT
        type::string(id) as id,
        institution,
        account_type,
        ((->is_secure->secure_account.identity)[0] or
            (->is_supersecure->supersecure_account.identity)[0] or
            (->use_sso_of->account.institution)[0] or
            'legacy'
        ) as identity,
        (SELECT color, name FROM (->is_sorted_in->bucket))[0] as bucket,
        (SELECT device, name FROM (->is_secured_by->twofactor))[0] as twofactor
    FROM account WHERE !archived;";
    let accounts: Vec<ListResult> = db.query(sql).await?.take(0)?;
    Ok(accounts)
}

#[tauri::command]
#[specta::specta]
pub async fn get_popular(db: DB<'_>) -> Result<Vec<PopularResult>, AccountError> {
    let sql = "
        SELECT
            type::string(id) as id,
            institution,
            account_type,
            calls,
            ((->is_secure->secure_account.identity)[0] or
                (->is_supersecure->supersecure_account.identity)[0] or
                (->use_sso_of->account.institution)[0] or
                'legacy'
            ) as identity,
            (SELECT color, name FROM (->is_sorted_in->bucket))[0] as bucket
        FROM account
        WHERE archived = false AND calls > 0
        ORDER BY calls DESC
        LIMIT 5;
        ";
    let accounts: Vec<PopularResult> = db.query(sql).await?.take(0)?;
    Ok(accounts)
}
