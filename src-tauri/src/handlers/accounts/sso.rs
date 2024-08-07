use crate::errors::AccountError;
use crate::types::{
    handlers::{AccountMetadata, SsoListResult, SsoOverview},
    DB,
};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[tauri::command]
#[specta::specta]
pub async fn get_sso_overview(db: DB<'_>, id: &str) -> Result<SsoOverview, AccountError> {
    let sql = "SELECT
    institution,
    recovery,
    website,
    alias,
    account_type as mode,
    type::string(created) as created,
    (->use_sso_of->account.institution)[0] as ssoaccount_institution,
    type::string((->use_sso_of->account.id)[0]) as ssoaccount_id,
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
        .take::<Option<SsoOverview>>(0)?
        .ok_or(AccountError::AccountNotFound(id.to_string()))?;
    Ok(data)
}

#[tauri::command]
#[specta::specta]
pub async fn create_sso(
    db: DB<'_>,
    ssoaccount_id: &str,
    metadata: AccountMetadata,
    bucketid: Option<&str>,
    twofactorid: Option<&str>,
) -> Result<String, AccountError> {
    let bucket = bucketid.map(|b| Thing::from(("bucket", b.split(':').last().unwrap())));
    let twofactor = twofactorid.map(|t| Thing::from(("twofactor", t.split(':').last().unwrap())));
    let sql = "fn::create_sso_account(
        type::thing('account', $ssoaccount_id),
        $institution,
        $recovery,
        $website,
        $alias,
        $bucket,
        $twofactor
    );";
    let ssoaccount_id = ssoaccount_id.split(':').last().unwrap();
    let record = db
        .query(sql)
        .bind(("ssoaccount_id", ssoaccount_id))
        .bind(("institution", metadata.institution))
        .bind(("recovery", metadata.recovery))
        .bind(("website", metadata.website))
        .bind(("alias", metadata.alias))
        .bind(("bucket", bucket))
        .bind(("twofactor", twofactor))
        .await?
        .take::<Option<Thing>>(0)?
        .ok_or(AccountError::NoID)?;
    Ok(record.id.to_string())
}

#[tauri::command]
#[specta::specta]
pub async fn edit_sso(
    db: DB<'_>,
    id: &str,
    ssoaccount_id: &str,
    metadata: AccountMetadata,
    bucketid: Option<&str>,
    twofactorid: Option<&str>,
) -> Result<String, AccountError> {
    let bucket = bucketid.map(|b| Thing::from(("bucket", b.split(':').last().unwrap())));
    let twofactor = twofactorid.map(|t| Thing::from(("twofactor", t.split(':').last().unwrap())));
    let sql = "fn::edit_sso_account(
        type::thing('account', $id),
        type::thing('account', $ssoaccount_id),
        $institution,
        $recovery,
        $website,
        $alias,
        $bucket,
        $twofactor
    );";
    let id = id.split(':').last().unwrap();
    let ssoaccount_id = ssoaccount_id.split(':').last().unwrap();
    let record = db
        .query(sql)
        .bind(("id", id))
        .bind(("ssoaccount_id", ssoaccount_id))
        .bind(("institution", metadata.institution))
        .bind(("recovery", metadata.recovery))
        .bind(("website", metadata.website))
        .bind(("alias", metadata.alias))
        .bind(("bucket", bucket))
        .bind(("twofactor", twofactor))
        .await?
        .take::<Option<Thing>>(0)?
        .ok_or(AccountError::NoID)?;
    Ok(record.id.to_string())
}

#[tauri::command]
#[specta::specta]
pub async fn list_nosso_accounts(db: DB<'_>) -> Result<Vec<SsoListResult>, AccountError> {
    let sql = "
        SELECT type::string(id) as id,
        institution,
        (SELECT color, name FROM (->is_sorted_in->bucket))[0] as bucket
        FROM account WHERE account_type != 'Sso';";
    let accounts: Vec<SsoListResult> = db.query(sql).await?.take(0)?;
    Ok(accounts)
}
