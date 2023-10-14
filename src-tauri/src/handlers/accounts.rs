use crate::algorithm::gen_pw;
use crate::common::get_user;
use crate::errors::AccountError;
use crate::types::{Industry, Record, DB};
use serde::{Deserialize, Serialize};
use specta::Type;
use surrealdb::sql::Thing;

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct Institution {
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct AccountOverview {
    pub id: String,
    pub identity: String,
    pub institution_name: String,
    pub bucket_name: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct AccountData {
    pub created_at: String,
    pub identity: String,
    pub recovery: Option<String>,
    pub institution: Institution,
}

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct SecureAccount {
    pub industry: Industry,
    pub account: AccountData,
}

#[derive(Debug, Serialize)]
struct NewInstitution {
    name: String,
    website: Option<String>,
    user: Thing,
}

#[derive(Debug, Serialize)]
struct NewAccount {
    identity: String,
    recovery: Option<String>,
    bucket: Option<Thing>,
    user: Thing,
    institution: Thing,
}

#[derive(Debug, Serialize)]
struct NewSecureAccount {
    industry: Industry,
    account: Thing,
}

// =====================================================================================================================
// ===All===============================================================================================================
// =====================================================================================================================

#[tauri::command]
#[specta::specta]
pub async fn search(db: DB<'_>, search_term: &str) -> Result<Vec<AccountOverview>, AccountError> {
    Ok(db
        .query(
            "
            SELECT id, identity, institution.name as institution_name, bucket.name as bucket_name
            FROM account
            WHERE institution.name ~ $query",
        )
        .bind(("query", search_term))
        .await?
        .take::<Vec<AccountOverview>>(0)?)
}

#[tauri::command]
#[specta::specta]
pub async fn search_bucket(
    db: DB<'_>,
    search_term: &str,
    bucket_id: &str,
) -> Result<Vec<AccountOverview>, AccountError> {
    Ok(db
        .query(
            "
            SELECT id, identity, institution.name as institution_name, bucket.name as bucket_name
            FROM account
            WHERE institution.name ~ $query AND bucket.id = $bucket_id",
        )
        .bind(("query", search_term, "bucket_id", bucket_id))
        .await?
        .take::<Vec<AccountOverview>>(0)?
        .into())
}

// =====================================================================================================================
// ===Secure============================================================================================================
// =====================================================================================================================

#[tauri::command]
#[specta::specta]
pub async fn secure_live_input(
    institution: &str,
    identity: &str,
    industry: Industry,
) -> Result<String, ()> {
    let secret = "supersecretsecret"; // TODO
    let pw = gen_pw(institution, &industry, secret, identity);
    Ok(pw)
}

#[tauri::command]
#[specta::specta]
pub async fn create_secure(
    db: DB<'_>,
    institution_name: &str,
    institution_website: Option<String>,
    identity: &str,
    recovery: Option<String>,
    industry: Industry,
    bucketid: Option<&str>,
) -> Result<String, AccountError> {
    let auth = get_user(db.clone()).await?;

    // Check if bucket exists
    let bucket: Option<Thing> = match bucketid {
        Some(bucketid) => Some(
            db.query("SELECT id FROM buckets WHERE id = $id;")
                .bind(("id", bucketid))
                .await?
                .take::<Option<Thing>>(0)?
                .ok_or(AccountError::CorruptedBucket(bucketid.to_string()))?,
        ),
        None => None,
    };

    // Check if institution exists
    let mut institution: Option<Thing> = db
        .query("SELECT id FROM institutions WHERE name = $name;")
        .bind(("name", institution_name))
        .await?
        .take(0)?;

    if institution.is_none() {
        let new_institution_object = NewInstitution {
            name: institution_name.to_string(),
            website: institution_website,
            user: auth.clone(),
        };
        let new_institution: Vec<Record> = db
            .create("institution")
            .content(new_institution_object)
            .await?;
        institution = new_institution[0].id.clone().into();
    }

    let new_account: Vec<Record> = db
        .create("account")
        .content(NewAccount {
            identity: identity.to_string(),
            recovery,
            bucket: bucket,
            user: auth,
            institution: institution.ok_or(AccountError::CorruptedInstitution(
                institution_name.to_string(),
            ))?,
        })
        .await?;

    let account_id = new_account[0].id.clone();

    let secure_account: Vec<Record> = db
        .create("secure_account")
        .content(NewSecureAccount {
            industry: industry,
            account: account_id,
        })
        .await?;

    Ok(secure_account[0].id.id.to_string())
}

#[tauri::command]
#[specta::specta]
pub async fn get_secure(db: DB<'_>, id: &str) -> Result<(SecureAccount, String), AccountError> {
    let mut result = db
        .query("SELECT industry, account.created_at, account.identity, account.recovery, account.institution.name
            FROM type::thing('secure_account', $id);")
        .bind(("id", id))
        .await?;
    let account = result
        .take::<Option<SecureAccount>>(0)?
        .ok_or(AccountError::SecureAccountNotFound(id.to_string()))?;
    let secret = "supersecrure"; // TODO
    let password = gen_pw(
        &account.account.institution.name,
        &account.industry,
        secret,
        &account.account.identity,
    );
    Ok((account, password))
}

// TODO: update

#[derive(Debug, Deserialize)]
struct DeleteAccount {
    id: Thing,
    account_id: Thing,
    industry_id: Thing,
}

#[tauri::command]
#[specta::specta]
pub async fn delete_secure(db: DB<'_>, id: &str) -> Result<(), AccountError> {
    let mut result = db
        .query(
            "SELECT id, account.id, account.institution.id
            FROM type::thing('secure_account', $id);",
        )
        .bind(("id", id))
        .await?;
    let account = result
        .take::<Option<DeleteAccount>>(0)?
        .ok_or(AccountError::SecureAccountNotFound(id.to_string()))?;
    db.delete::<Option<Record>>(("account", account.account_id))
        .await?;

    // TODO: check if the institution is still used by other accounts & sso-accounts

    db.delete::<Option<Record>>(("secure_account", id)).await?;
    Ok(())
}
