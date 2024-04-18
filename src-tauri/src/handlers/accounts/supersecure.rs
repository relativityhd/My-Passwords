use crate::algorithm::gen_super_pw;
use crate::errors::AccountError;
use crate::handlers::accounts::add_call;
use crate::types::{
    extract_lc,
    handlers::{AccountMetadata, SuperSecureOverview, SuperSecureSpecifics},
    Industry, LocalCreds, DB, LC,
};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Deserialize, Serialize)]
struct SuperSecurePasswordData {
    institution: String,
    industry: Industry,
    identity: String,
    specials: String,
    seed: usize,
    min: usize,
    max: usize,
}

#[tauri::command]
#[specta::specta]
pub async fn supersecure_live_input(
    lc: LC<'_>,
    institution: &str,
    identity: &str,
    industry: Industry,
    specials: String,
    seed: u32,
    min: u32,
    max: u32,
) -> Result<String, AccountError> {
    let local_creds: LocalCreds = extract_lc(&lc).await?;

    let pw = gen_super_pw(
        institution,
        &industry,
        &local_creds.secret,
        identity,
        local_creds.pin as usize,
        min as usize,
        max as usize,
        &specials,
        seed as usize,
    )?;
    Ok(pw)
}

#[tauri::command]
#[specta::specta]
pub async fn get_supersecure_password(
    db: DB<'_>,
    lc: LC<'_>,
    id: &str,
) -> Result<String, AccountError> {
    let local_creds: LocalCreds = extract_lc(&lc).await?;

    // Convert id to Record
    let sql = "SELECT institution,
        (->is_supersecure->supersecure_account.industry)[0] as industry,
        (->is_supersecure->supersecure_account.identity)[0] as identity,
        (->is_supersecure->supersecure_account.specials)[0] as specials,
        (->is_supersecure->supersecure_account.seed)[0] as seed,
        (->is_supersecure->supersecure_account.min_length)[0] as min,
        (->is_supersecure->supersecure_account.max_length)[0] as max
        FROM ONLY type::thing('account', $account);";
    let id = id.split(':').last().unwrap();
    let data = db
        .query(sql)
        .bind(("account", id))
        .await?
        .take::<Option<SuperSecurePasswordData>>(0)?
        .ok_or(AccountError::AccountNotFound(id.to_string()))?;
    let pw = gen_super_pw(
        &data.institution,
        &data.industry,
        &local_creds.secret,
        &data.identity,
        local_creds.pin as usize,
        data.min as usize,
        data.max as usize,
        &data.specials,
        data.seed as usize,
    )?;
    add_call(db, id).await?;
    Ok(pw)
}

#[tauri::command]
#[specta::specta]
pub async fn get_supersecure_overview(
    db: DB<'_>,
    lc: LC<'_>,
    id: &str,
) -> Result<(SuperSecureOverview, String), AccountError> {
    let local_creds: LocalCreds = extract_lc(&lc).await?;

    let sql = "SELECT
        institution,
        recovery,
        website,
        alias,
        account_type as mode,
        type::string(created) as created,
        (->is_supersecure->supersecure_account.industry)[0] as industry,
        (->is_supersecure->supersecure_account.identity)[0] as identity,
        (->is_supersecure->supersecure_account.specials)[0] as specials,
        (->is_supersecure->supersecure_account.seed)[0] as seed,
        (->is_supersecure->supersecure_account.min_length)[0] as min,
        (->is_supersecure->supersecure_account.max_length)[0] as max,
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
        .take::<Option<SuperSecureOverview>>(0)?
        .ok_or(AccountError::AccountNotFound(id.to_string()))?;
    let pw = gen_super_pw(
        &data.institution,
        &data.industry,
        &local_creds.secret,
        &data.identity,
        local_creds.pin as usize,
        data.min as usize,
        data.max as usize,
        &data.specials,
        data.seed as usize,
    )?;

    Ok((data, pw))
}

#[tauri::command]
#[specta::specta]
pub async fn create_supersecure(
    db: DB<'_>,
    metadata: AccountMetadata,
    specifics: SuperSecureSpecifics,
    bucketid: Option<&str>,
    twofactorid: Option<&str>,
) -> Result<String, AccountError> {
    let bucket = bucketid.map(|b| Thing::from(("bucket", b.split(':').last().unwrap())));
    let twofactor = twofactorid.map(|t| Thing::from(("twofactor", t.split(':').last().unwrap())));
    let sql = "
        fn::create_supersecure_account(
            $identity,
            $industry,
            $specials,
            $seed,
            $min_length,
            $max_length,
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
        .bind(("specials", specifics.specials))
        .bind(("seed", specifics.seed))
        .bind(("min_length", specifics.min))
        .bind(("max_length", specifics.max))
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
pub async fn edit_supersecure(
    db: DB<'_>,
    id: &str,
    metadata: AccountMetadata,
    specifics: SuperSecureSpecifics,
    bucketid: Option<&str>,
    twofactorid: Option<&str>,
) -> Result<String, AccountError> {
    let bucket = bucketid.map(|b| Thing::from(("bucket", b.split(':').last().unwrap())));
    let twofactor = twofactorid.map(|t| Thing::from(("twofactor", t.split(':').last().unwrap())));
    dbg!(&bucket, &twofactor);
    let sql = "
        fn::edit_supersecure_account(
            type::thing('account', $account),
            $identity,
            $industry,
            $specials,
            $seed,
            $min_length,
            $max_length,
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
        .bind(("specials", specifics.specials))
        .bind(("seed", specifics.seed))
        .bind(("min_length", specifics.min))
        .bind(("max_length", specifics.max))
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
