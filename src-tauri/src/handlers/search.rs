use crate::errors::AccountError;
use crate::types::{Mode, DB};
use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Serialize, Deserialize, Debug, Type)]
pub struct SearchResultBucket {
    pub name: String,
    pub color: String,
}

#[derive(Serialize, Deserialize, Debug, Type)]
pub struct SearchResult {
    pub id: String,
    pub account_type: Mode,
    pub institution: String,
    pub identity: String,
    pub bucket: SearchResultBucket,
}

/*
"Failed to convert
`[{ account_type: 'supersecure', bucket: { color: '#00ff00', name: 'home' }, id: account:zf22lensmix8irp10v5w, identity: 'tobiashoelzer@hotmail.com', institution: 'Microsoft' }]`
to `T`: invalid type: map, expected a string"

*/

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
        WHERE institution ~ $query;
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
        WHERE institution ~ $query AND (SELECT id FROM (->is_sorted_in->bucket))[0] = $bucket;
    ";
    let accounts: Vec<SearchResult> = db
        .query(sql)
        .bind(("query", search_term))
        .bind(("bucket", bucket_id))
        .await?
        .take(0)?;
    Ok(accounts)
}
