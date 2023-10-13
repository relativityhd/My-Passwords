use super::types::AccountError;
use crate::algorithm::gen_pw;
use crate::handlers::auth::AuthError;
use crate::types::Industry;
use serde::{Deserialize, Serialize};
use specta::Type;
use surrealdb::engine::remote::ws::Client;
use surrealdb::sql::Thing;
use surrealdb::Surreal;

#[derive(Debug, Deserialize)]
struct Record {
    id: Thing,
}

// Rename to solve?
#[tauri::command]
#[specta::specta]
pub async fn live_input(
    institution: &str,
    identity: &str,
    industry: Industry,
) -> Result<String, ()> {
    let secret = "supersecretsecret"; // TODO
    let pw = gen_pw(institution, &industry, secret, identity);
    Ok(pw)
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

#[tauri::command]
#[specta::specta]
pub async fn create(
    db: tauri::State<'_, Surreal<Client>>,
    institution_name: &str,
    institution_website: Option<String>,
    identity: &str,
    recovery: Option<String>,
    industry: Industry,
    bucketid: Option<&str>,
) -> Result<String, AccountError> {
    let auth = db
        .query("RETURN $auth;")
        .await?
        .take::<Option<Thing>>(0)?
        .ok_or(AuthError::NotSignedIn)?;

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
            bucket: None, // TODO
            user: auth,
            institution: institution.unwrap(),
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

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct Institution {
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct Account {
    pub created_at: String,
    pub identity: String,
    pub recovery: Option<String>,
    pub institution: Institution,
}

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct SecureAccount {
    pub industry: Industry,
    pub account: Account,
}

#[tauri::command]
#[specta::specta]
pub async fn get(
    db: tauri::State<'_, Surreal<Client>>,
    id: &str,
) -> Result<(SecureAccount, String), AccountError> {
    let mut result = db
        .query("SELECT industry, account.created_at, account.identity, account.recovery, account.institution.name
            FROM type::thing('secure_account', $id);")
        .bind(("id", id))
        .await?;
    let account = result
        .take::<Option<SecureAccount>>(0)?
        .ok_or(AccountError::NotFound(Thing::from(("secure_account", id))))?;
    let secret = "supersecrure"; // TODO
    let password = gen_pw(
        &account.account.institution.name,
        &account.industry,
        secret,
        &account.account.identity,
    );
    Ok((account, password))
}
