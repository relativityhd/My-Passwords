use crate::algorithm::gen_pw;
use crate::common::extract_lc;
use crate::errors::AccountError;
use crate::handlers::accounts::add_call;
use crate::types::{
    handlers::{AccountMetadata, SecureOverview, SecureSpecifics},
    Industry, LocalCreds, DB, LC,
};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Deserialize, Serialize)]
struct SecurePasswordData {
    institution: String,
    industry: Industry,
    identity: String,
}

#[tauri::command]
#[specta::specta]
pub async fn secure_live_input(
    lc: LC<'_>,
    institution: &str,
    identity: &str,
    industry: Industry,
) -> Result<String, AccountError> {
    //let state = lc.lock()?;
    //let local_creds: LocalCreds =
    //<Option<LocalCreds> as Clone>::clone(&state).ok_or(AccountError::PinNotFound)?;
    let local_creds = extract_lc(&lc).await?;
    let pw = gen_pw(institution, &industry, &local_creds.secret, identity);
    Ok(pw)
}

#[tauri::command]
#[specta::specta]
pub async fn get_secure_password(db: DB<'_>, lc: LC<'_>, id: &str) -> Result<String, AccountError> {
    let local_creds = extract_lc(&lc).await?;
    let secret = local_creds.secret;
    // Convert id to Record
    let sql = "SELECT institution,
        (->is_secure->secure_account.industry)[0] as industry,
        (->is_secure->secure_account.identity)[0] as identity
        FROM ONLY type::thing('account', $account);";
    let id = id.split(':').last().unwrap();
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
    add_call(db, id).await?;
    Ok(pw)
}

#[tauri::command]
#[specta::specta]
pub async fn get_secure_overview(
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
        (->is_secure->secure_account.industry)[0] as industry,
        (->is_secure->secure_account.identity)[0] as identity,
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
    let pw = gen_pw(
        &data.institution,
        &data.industry,
        &secret.to_string(),
        &data.identity,
    );

    Ok((data, pw))
}

#[tauri::command]
#[specta::specta]
pub async fn create_secure(
    db: DB<'_>,
    metadata: AccountMetadata,
    specifics: SecureSpecifics,
    bucketid: Option<&str>,
    twofactorid: Option<&str>,
) -> Result<String, AccountError> {
    let bucket = bucketid.map(|b| Thing::from(("bucket", b.split(':').last().unwrap())));
    let twofactor = twofactorid.map(|t| Thing::from(("twofactor", t.split(':').last().unwrap())));
    let sql = "
        fn::create_secure_account(
            $identity,
            $industry,
            $institution,
            $recovery,
            $website,
            $alias,
            $bucket,
            $twofactor
        );
    ";
    let record = db
        .query(sql)
        .bind(("identity", specifics.identity))
        .bind(("industry", specifics.industry))
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
pub async fn edit_secure(
    db: DB<'_>,
    id: &str,
    metadata: AccountMetadata,
    specifics: SecureSpecifics,
    bucketid: Option<&str>,
    twofactorid: Option<&str>,
) -> Result<String, AccountError> {
    let bucket = bucketid.map(|b| Thing::from(("bucket", b.split(':').last().unwrap())));
    let twofactor = twofactorid.map(|t| Thing::from(("twofactor", t.split(':').last().unwrap())));
    dbg!(&bucket, &twofactor);
    let sql = "
        fn::edit_secure_account(
            type::thing('account', $account),
            $identity,
            $industry,
            $institution,
            $recovery,
            $website,
            $alias,
            $bucket,
            $twofactor
        );
    ";
    let id = id.split(':').last().unwrap();
    let record = db
        .query(sql)
        .bind(("account", id))
        .bind(("identity", specifics.identity))
        .bind(("industry", specifics.industry))
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
