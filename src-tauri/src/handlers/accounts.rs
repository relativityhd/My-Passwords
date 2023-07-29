use crate::algorithm::gen_pw;
use crate::entities;
use crate::entities::{prelude::*, *};
use crate::types::{Industry, InvalidModeError, Mode};
use chrono::Utc;
use sea_orm::*;
use serde::Serialize;
use specta::Type;
use std::str::FromStr;
use std::sync::Mutex;
use thiserror::Error;
use tokio::join;

//type DB = tauri::State<Mutex<DatabaseConnection>>;
//type User = tauri::State<Mutex<user::Model>>;

#[derive(Serialize, Debug, Type)]
pub struct RetrievedSecretAccount {
    pub id: i32,
    pub created_at: String,
    pub account_name: String,
    pub mode: Mode,
    pub industry: Industry,
    pub two_factor_auth: bool,
    pub recovery: Option<String>,
    pub bucket_name: String,
    pub bucket_color: String,
    pub institution_name: String,
    pub password: String,
}

impl RetrievedSecretAccount {
    pub fn new(
        secret: &str,
        mode: Mode,
        account: <entities::account::Entity as sea_orm::EntityTrait>::Model,
        bucket: <entities::bucket::Entity as sea_orm::EntityTrait>::Model,
        institution: <entities::institution::Entity as sea_orm::EntityTrait>::Model,
    ) -> Self {
        let industry = Industry::from_str(&account.industry).unwrap();
        let password = gen_pw(&institution.name, &industry, secret, &account.account_name);
        Self {
            id: account.id,
            created_at: account.created_at,
            account_name: account.account_name,
            mode: mode,
            industry,
            two_factor_auth: account.two_factor_auth,
            recovery: account.recovery,
            bucket_name: bucket.name,
            bucket_color: bucket.color,
            institution_name: institution.name,
            password,
        }
    }
}

#[derive(Serialize, Debug, Type)]
pub enum RetrievedAccount {
    RetrievedSecretAccount(RetrievedSecretAccount),
}

#[derive(Debug, Error)]
pub enum AccountError {
    #[error("Database error from SeaORM: {0:?}")]
    Db(#[from] DbErr),
    #[error("No account found with id {0}")]
    NotFound(i32),
    #[error("Database is probably corrupted: No Bucket found with id {0}")]
    CorruptedBucket(i32),
    #[error("Database is probably corrupted: No Institution found with id {0}")]
    CorruptedInstitution(i32),
    #[error("Database is probably corrupted: {0:?}")]
    CorruptedMode(#[from] InvalidModeError),
}

impl Serialize for AccountError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

#[tauri::command]
#[specta::specta]
pub async fn retrieve_account(
    db: tauri::State<'_, DatabaseConnection>,
    account_id: i32,
) -> Result<RetrievedAccount, AccountError> {
    let account = Account::find_by_id(account_id)
        .one(db.inner())
        .await?
        .ok_or(AccountError::NotFound(account_id))?;
    // TODO check if acc user == user
    let bucket_future = Bucket::find_by_id(account.bucket_id).one(db.inner());
    let institution_future = Institution::find_by_id(account.institution_id).one(db.inner());
    let res = join!(bucket_future, institution_future);
    let bucket = res
        .0?
        .ok_or(AccountError::CorruptedBucket(account.bucket_id))?;
    let institution = res
        .1?
        .ok_or(AccountError::CorruptedInstitution(account.institution_id))?;
    let secret = "secret_val"; // TODO: get secret from user and limit it to max 10 chars
    let mode = Mode::from_str(&account.mode)?; // TODO: fetch SSO-Instiution from DB in SSO mode
    let retrieved_account =
        RetrievedSecretAccount::new(&secret, mode, account, bucket, institution);
    Ok(RetrievedAccount::RetrievedSecretAccount(retrieved_account))
}

#[tauri::command]
#[specta::specta]
pub async fn add_acc(
    db: tauri::State<'_, DatabaseConnection>,
    user: tauri::State<'_, Mutex<user::Model>>,
    institution_name: &str,
    account_name: &str,
    industry: Industry,
) -> Result<i32, AccountError> {
    let mode = Mode::Secure;
    let created = Utc::now();
    let user_id = user.lock().unwrap().id;
    let bucket_id = 1;
    drop(user);

    let mut institution = Institution::find()
        .filter(institution::Column::Name.eq(institution_name))
        .one(db.inner())
        .await?;

    // Check if institution exists
    if institution.is_none() {
        let new_institution = institution::ActiveModel {
            name: Set(institution_name.to_owned()),
            user_id: Set(user_id),
            ..Default::default()
        };
        institution = Some(new_institution.insert(db.inner()).await?)
    }

    let new_acc = account::ActiveModel {
        created_at: Set(created.to_rfc3339()),
        account_name: Set(account_name.to_owned()),
        mode: Set(mode.to_string()),
        industry: Set(industry.to_string()),
        user_id: Set(user_id),
        bucket_id: Set(bucket_id),
        institution_id: Set(institution.unwrap().id),
        ..Default::default()
    };

    let account = new_acc.insert(db.inner()).await?;

    Ok(account.id)
}

#[tauri::command]
#[specta::specta]
pub async fn add_super_acc() -> Result<(), AccountError> {
    todo!()
}

#[tauri::command]
#[specta::specta]
pub async fn move_acc_to_bucket() -> Result<(), AccountError> {
    todo!()
}

#[tauri::command]
#[specta::specta]
pub async fn delete_and_create_new_acc() -> Result<(), AccountError> {
    todo!()
}

#[tauri::command]
#[specta::specta]
pub async fn delete_acc() -> Result<(), AccountError> {
    todo!()
}

#[tauri::command]
#[specta::specta]
pub async fn search_acc() -> Result<(), AccountError> {
    todo!()
}
