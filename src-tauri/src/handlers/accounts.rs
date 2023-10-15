use crate::algorithm::{gen_pw, gen_super_pw};
use crate::common::get_user;
use crate::errors::AccountError;
use crate::types::{Industry, Record, DB};
use serde::{Deserialize, Serialize};
use specta::Type;
use surrealdb::sql::Thing;

#[derive(Serialize, Deserialize, Debug, Type)]
pub enum AccountType {
    Secure,
    SuperSecure,
    Sso,
}

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct Institution {
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct AccountOverview {
    pub id: String,
    pub institution_name: String,
    pub bucket_name: Option<String>,
    pub account_type: AccountType,
}

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct AccountData {
    pub created_at: String,
    pub recovery: Option<String>,
    pub institution: Institution,
    pub account_type: AccountType,
}

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct SecureAccount {
    pub identity: String,
    pub industry: Industry,
    pub account: AccountData,
}

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct SuperSecureAccount {
    pub identity: String,
    pub industry: Industry,
    pub pin: i32,
    pub min_length: i32,
    pub max_length: i32,
    pub account: AccountData,
}

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct SsoAccount {
    pub sso: Institution,
    pub account: AccountData,
}

#[derive(Debug, Serialize)]
struct NewInstitution {
    name: String,
    website: Option<String>,
    alias: Vec<String>,
    user: Thing,
}

#[derive(Debug, Serialize)]
struct NewAccount {
    recovery: Option<String>,
    account_type: AccountType,
    bucket: Option<Thing>,
    two_factor: Option<Thing>,
    institution: Thing,
    user: Thing,
}

#[derive(Debug, Serialize)]
struct NewSecureAccount {
    identity: String,
    industry: Industry,
    account: Thing,
    user: Thing,
}

#[derive(Debug, Serialize)]
struct NewSuperSecureAccount {
    identity: String,
    pin: i32,
    min_length: i32,
    max_length: i32,
    industry: Industry,
    account: Thing,
    user: Thing,
}

#[derive(Debug, Serialize)]
struct NewSsoAccount {
    account: Thing,
    sso: Thing,
    user: Thing,
}

#[derive(Debug, Deserialize)]
struct DeleteAccount {
    id: Thing,
    account_id: Thing,
    industry_id: Thing,
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
            SELECT id, account_type, institution.name as institution_name, bucket.name as bucket_name
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
            SELECT id, account_type, institution.name as institution_name, bucket.name as bucket_name
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
    institution_alias: Vec<String>,
    identity: &str,
    recovery: Option<String>,
    industry: Industry,
    bucketid: Option<&str>,
    twofactorid: Option<&str>,
) -> Result<String, AccountError> {
    let auth = get_user(db.clone()).await?;

    // Check if bucket exists
    let bucket: Option<Thing> = match bucketid {
        Some(bucketid) => db
            .query("SELECT id FROM buckets WHERE id = $id;")
            .bind(("id", bucketid))
            .await?
            .take::<Option<Thing>>(0)?
            .ok_or(AccountError::CorruptedBucket(bucketid.to_string()))?
            .into(),
        None => None,
    };

    // Check if institution exists
    let mut institution: Option<Thing> = db
        .query("SELECT id FROM institutions WHERE name = $name;")
        .bind(("name", institution_name))
        .await?
        .take(0)?;

    if let Some(institution_id) = institution {
        // Add aliases
        institution = db
            .query("UPDATE type::thing('institution', $id) SET alias += $new_aliases;")
            .bind(("id", &institution_id))
            .bind(("new_aliases", institution_alias))
            .await?
            .take::<Option<Thing>>(0)?
            .ok_or(AccountError::CorruptedInstitution(
                institution_id.id.to_string(),
            ))?
            .into();
    } else {
        // Create if not exists
        let new_institution_object = NewInstitution {
            name: institution_name.to_string(),
            website: institution_website,
            alias: institution_alias,
            user: auth.clone(),
        };
        let new_institution: Vec<Record> = db
            .create("institution")
            .content(new_institution_object)
            .await?;
        institution = new_institution[0].id.clone().into();
    }

    // Check if 2fa exists
    let two_factor: Option<Thing> = match twofactorid {
        Some(twofactorid) => db
            .query("SELECT id FROM two_factor WHERE id = $id;")
            .bind(("id", twofactorid))
            .await?
            .take::<Option<Thing>>(0)?
            .ok_or(AccountError::CorruptedTwoFactor(twofactorid.to_string()))?
            .into(),
        None => None,
    };

    let new_account: Vec<Record> = db
        .create("account")
        .content(NewAccount {
            account_type: AccountType::Secure,
            recovery,
            bucket: bucket,
            two_factor: two_factor,
            user: auth.clone(),
            institution: institution.ok_or(AccountError::CorruptedInstitution(
                institution_name.to_string(),
            ))?,
        })
        .await?;

    let account_id = new_account[0].id.clone();

    let secure_account: Vec<Record> = db
        .create("secure_account")
        .content(NewSecureAccount {
            identity: identity.to_string(),
            industry,
            account: account_id,
            user: auth,
        })
        .await?;

    Ok(secure_account[0].id.id.to_string())
}

#[tauri::command]
#[specta::specta]
pub async fn get_secure(db: DB<'_>, id: &str) -> Result<(SecureAccount, String), AccountError> {
    let mut result = db
        .query("SELECT identity, industry, account.created_at, account.recovery, account.account_type, account.institution.name
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
        &account.identity,
    );
    Ok((account, password))
}

// TODO: update

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
    db.delete::<Option<Record>>(("secure_account", id)).await?;
    db.delete::<Option<Record>>(("account", account.account_id))
        .await?;

    // TODO: check if the institution is still used by other accounts & sso-accounts

    Ok(())
}

// =====================================================================================================================
// ===SuperSecure=======================================================================================================
// =====================================================================================================================

#[tauri::command]
#[specta::specta]
pub async fn supersecure_live_input(
    institution: &str,
    identity: &str,
    industry: Industry,
    pin: i32,
    min_length: i32,
    max_length: i32,
) -> Result<String, ()> {
    let secret = "supersecretsecret"; // TODO
    let pw = gen_super_pw(
        institution,
        &industry,
        secret,
        identity,
        pin,
        min_length,
        max_length,
    );
    Ok(pw)
}

#[tauri::command]
#[specta::specta]
pub async fn create_super_secure(
    db: DB<'_>,
    institution_name: &str,
    institution_website: Option<String>,
    institution_alias: Vec<String>,
    identity: &str,
    recovery: Option<String>,
    industry: Industry,
    pin: i32,
    min_length: i32,
    max_length: i32,
    bucketid: Option<&str>,
    twofactorid: Option<&str>,
) -> Result<String, AccountError> {
    let auth = get_user(db.clone()).await?;

    // Check if bucket exists
    let bucket: Option<Thing> = match bucketid {
        Some(bucketid) => db
            .query("SELECT id FROM buckets WHERE id = $id;")
            .bind(("id", bucketid))
            .await?
            .take::<Option<Thing>>(0)?
            .ok_or(AccountError::CorruptedBucket(bucketid.to_string()))?
            .into(),
        None => None,
    };

    // Check if institution exists
    let mut institution: Option<Thing> = db
        .query("SELECT id FROM institutions WHERE name = $name;")
        .bind(("name", institution_name))
        .await?
        .take(0)?;

    if let Some(institution_id) = institution {
        // Add aliases
        institution = db
            .query("UPDATE type::thing('institution', $id) SET alias += $new_aliases;")
            .bind(("id", &institution_id))
            .bind(("new_aliases", institution_alias))
            .await?
            .take::<Option<Thing>>(0)?
            .ok_or(AccountError::CorruptedInstitution(
                institution_id.id.to_string(),
            ))?
            .into();
    } else {
        // Create if not exists
        let new_institution_object = NewInstitution {
            name: institution_name.to_string(),
            website: institution_website,
            alias: institution_alias,
            user: auth.clone(),
        };
        let new_institution: Vec<Record> = db
            .create("institution")
            .content(new_institution_object)
            .await?;
        institution = new_institution[0].id.clone().into();
    }

    // Check if 2fa exists
    let two_factor: Option<Thing> = match twofactorid {
        Some(twofactorid) => db
            .query("SELECT id FROM two_factor WHERE id = $id;")
            .bind(("id", twofactorid))
            .await?
            .take::<Option<Thing>>(0)?
            .ok_or(AccountError::CorruptedTwoFactor(twofactorid.to_string()))?
            .into(),
        None => None,
    };

    let new_account: Vec<Record> = db
        .create("account")
        .content(NewAccount {
            account_type: AccountType::SuperSecure,
            recovery,
            bucket: bucket,
            two_factor: two_factor,
            user: auth.clone(),
            institution: institution.ok_or(AccountError::CorruptedInstitution(
                institution_name.to_string(),
            ))?,
        })
        .await?;

    let account_id = new_account[0].id.clone();

    let super_secure_account: Vec<Record> = db
        .create("secure_account")
        .content(NewSuperSecureAccount {
            identity: identity.to_string(),
            industry,
            account: account_id,
            pin,
            min_length,
            max_length,
            user: auth,
        })
        .await?;

    Ok(super_secure_account[0].id.id.to_string())
}

#[tauri::command]
#[specta::specta]
pub async fn get_super_secure(
    db: DB<'_>,
    id: &str,
) -> Result<(SuperSecureAccount, String), AccountError> {
    let mut result = db
        .query("SELECT identity, pin, min_length, max_length, industry, account.created_at, account.recovery, account.account_type, account.institution.name
            FROM type::thing('super_secure_account', $id);")
        .bind(("id", id))
        .await?;
    let account = result
        .take::<Option<SuperSecureAccount>>(0)?
        .ok_or(AccountError::SecureAccountNotFound(id.to_string()))?;
    let secret = "supersecrure"; // TODO
    let password = gen_super_pw(
        &account.account.institution.name,
        &account.industry,
        secret,
        &account.identity,
        account.pin,
        account.min_length,
        account.max_length,
    );
    Ok((account, password))
}

// TODO: update

#[tauri::command]
#[specta::specta]
pub async fn delete_super_secure(db: DB<'_>, id: &str) -> Result<(), AccountError> {
    let mut result = db
        .query(
            "SELECT id, account.id, account.institution.id
            FROM type::thing('super_secure_account', $id);",
        )
        .bind(("id", id))
        .await?;
    let account = result
        .take::<Option<DeleteAccount>>(0)?
        .ok_or(AccountError::SuperSecureAccountNotFound(id.to_string()))?;
    db.delete::<Option<Record>>(("super_secure_account", id))
        .await?;
    db.delete::<Option<Record>>(("account", account.account_id))
        .await?;

    // TODO: check if the institution is still used by other accounts & sso-accounts

    Ok(())
}

// =====================================================================================================================
// ===SSOSecure=========================================================================================================
// =====================================================================================================================

#[tauri::command]
#[specta::specta]
pub async fn create_sso(
    db: DB<'_>,
    institution_name: &str,
    institution_website: Option<String>,
    institution_alias: Vec<String>,
    sso_id: &str,
    recovery: Option<String>,
    bucketid: Option<&str>,
    twofactorid: Option<&str>,
) -> Result<String, AccountError> {
    let auth = get_user(db.clone()).await?;

    // Check if sso exists
    let sso: Thing = db
        .query("SELECT id FROM institution WHERE id = $id;")
        .bind(("id", sso_id))
        .await?
        .take::<Option<Thing>>(0)?
        .ok_or(AccountError::CorruptedInstitution(sso_id.to_string()))?;

    // Check if bucket exists
    let bucket: Option<Thing> = match bucketid {
        Some(bucketid) => db
            .query("SELECT id FROM buckets WHERE id = $id;")
            .bind(("id", bucketid))
            .await?
            .take::<Option<Thing>>(0)?
            .ok_or(AccountError::CorruptedBucket(bucketid.to_string()))?
            .into(),
        None => None,
    };

    // Check if institution exists
    let mut institution: Option<Thing> = db
        .query("SELECT id FROM institutions WHERE name = $name;")
        .bind(("name", institution_name))
        .await?
        .take(0)?;

    if let Some(institution_id) = institution {
        // Add aliases
        institution = db
            .query("UPDATE type::thing('institution', $id) SET alias += $new_aliases;")
            .bind(("id", &institution_id))
            .bind(("new_aliases", institution_alias))
            .await?
            .take::<Option<Thing>>(0)?
            .ok_or(AccountError::CorruptedInstitution(
                institution_id.id.to_string(),
            ))?
            .into();
    } else {
        // Create if not exists
        let new_institution_object = NewInstitution {
            name: institution_name.to_string(),
            website: institution_website,
            alias: institution_alias,
            user: auth.clone(),
        };
        let new_institution: Vec<Record> = db
            .create("institution")
            .content(new_institution_object)
            .await?;
        institution = new_institution[0].id.clone().into();
    }

    // Check if 2fa exists
    let two_factor: Option<Thing> = match twofactorid {
        Some(twofactorid) => db
            .query("SELECT id FROM two_factor WHERE id = $id;")
            .bind(("id", twofactorid))
            .await?
            .take::<Option<Thing>>(0)?
            .ok_or(AccountError::CorruptedTwoFactor(twofactorid.to_string()))?
            .into(),
        None => None,
    };

    let new_account: Vec<Record> = db
        .create("account")
        .content(NewAccount {
            account_type: AccountType::Sso,
            recovery,
            bucket: bucket,
            two_factor: two_factor,
            user: auth.clone(),
            institution: institution.ok_or(AccountError::CorruptedInstitution(
                institution_name.to_string(),
            ))?,
        })
        .await?;

    let account_id = new_account[0].id.clone();

    let sso_account: Vec<Record> = db
        .create("sso_account")
        .content(NewSsoAccount {
            account: account_id,
            sso: sso,
            user: auth,
        })
        .await?;

    Ok(sso_account[0].id.id.to_string())
}

#[tauri::command]
#[specta::specta]
pub async fn get_sso(db: DB<'_>, id: &str) -> Result<SsoAccount, AccountError> {
    let mut result = db
        .query("SELECT sso.name, account.created_at, account.recovery, account.account_type, account.institution.name
            FROM type::thing('sso_account', $id);")
        .bind(("id", id))
        .await?;
    let account = result
        .take::<Option<SsoAccount>>(0)?
        .ok_or(AccountError::SsoAccountNotFound(id.to_string()))?;
    Ok(account)
}

// TODO: update

#[tauri::command]
#[specta::specta]
pub async fn delete_sso(db: DB<'_>, id: &str) -> Result<(), AccountError> {
    let mut result = db
        .query(
            "SELECT id, account.id, account.institution.id
            FROM type::thing('sso_account', $id);",
        )
        .bind(("id", id))
        .await?;
    let account = result
        .take::<Option<DeleteAccount>>(0)?
        .ok_or(AccountError::SsoAccountNotFound(id.to_string()))?;
    db.delete::<Option<Record>>(("sso_account", id)).await?;
    db.delete::<Option<Record>>(("account", account.account_id))
        .await?;

    // TODO: check if the institution is still used by other accounts & sso-accounts

    Ok(())
}
