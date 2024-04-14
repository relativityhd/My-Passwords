use crate::errors::AccountError;
use crate::types::{handlers::SearchResult, DB};

#[tauri::command]
#[specta::specta]
pub async fn search(db: DB<'_>, search_term: &str) -> Result<Vec<SearchResult>, AccountError> {
    let sql = "
        SELECT
            type::string(id) as id,
            institution,
            account_type,
            ((->is_secure->secure_account.identity)[0] or
                (->is_supersecure->supersecure_account.identity)[0] or
                string::concat('SSO::', (->use_sso_of->account.institution)[0])
            ) as identity,
            (SELECT color, name FROM (->is_sorted_in->bucket))[0] as bucket
        FROM account
        WHERE institution ~ $query AND archived = false;
    ";
    let accounts: Vec<SearchResult> = db.query(sql).bind(("query", search_term)).await?.take(0)?;
    Ok(accounts)
}

#[tauri::command]
#[specta::specta]
pub async fn search_bucket(
    db: DB<'_>,
    search_term: &str,
    bucket_id: &str,
) -> Result<Vec<SearchResult>, AccountError> {
    let sql = "
        SELECT
            type::string(id) as id,
            institution,
            account_type,
            ((->is_secure->secure_account.identity)[0] or
                (->is_supersecure->supersecure_account.identity)[0] or
                string::concat('SSO::', (->use_sso_of->account.institution)[0])
            ) as identity,
            (SELECT color, name FROM (->is_sorted_in->bucket))[0] as bucket
        FROM account
        WHERE institution ~ $query AND archived = false AND (SELECT id FROM (->is_sorted_in->bucket))[0] = $bucket;
    ";
    let accounts: Vec<SearchResult> = db
        .query(sql)
        .bind(("query", search_term))
        .bind(("bucket", bucket_id))
        .await?
        .take(0)?;
    Ok(accounts)
}
