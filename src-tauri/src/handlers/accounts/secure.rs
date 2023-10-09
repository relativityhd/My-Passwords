use crate::algorithm::gen_pw;
use crate::types::Industry;
use serde::{Deserialize, Serialize};
use surrealdb::engine::remote::ws::Client;
use surrealdb::sql::{thing, Id, Thing};
use surrealdb::Surreal;

use super::types::AccountError;
use crate::handlers::auth::AuthError;

#[derive(Debug, Serialize)]
struct NewInstitution {
    name: String,
    website: Option<String>,
    user: Id,
}

#[derive(Debug, Serialize)]
struct NewAccount {
    identity: String,
    recovery: Option<String>,
    bucket: Option<Id>,
    user: Id,
    institution: Id,
}

#[derive(Debug, Serialize)]
struct NewSecureAccount {
    industry: i32,
    account: NewAccount,
}

#[derive(Debug, Deserialize)]
struct Record {
    #[allow(dead_code)]
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
    dbg!(&institution);

    if institution.is_none() {
        dbg!("Creating new institution");
        let new_institution: Vec<Record> = db
            .create("institutions")
            .content(NewInstitution {
                name: institution_name.to_string(),
                website: institution_website,
                user: auth.id.clone(),
            })
            .await?;
        institution = new_institution[0].id.clone().into();
    }
    dbg!(&institution);

    let account: Vec<Record> = db
        .create("secure_account")
        .content(NewSecureAccount {
            industry: industry.into(),
            account: NewAccount {
                identity: identity.to_string(),
                recovery,
                bucket: None, // TODO
                user: auth.id,
                institution: institution.unwrap().id,
            },
        })
        .await?;

    Ok(account[0].id.to_string())
}
