use super::types::AccountError;
use crate::algorithm::gen_pw;
use crate::types::Industry;
use chrono::Utc;
use entities;
use entities::sea_orm_active_enums::Industry as IndustryEnum;
use entities::{prelude::*, *};
use sea_orm::*;
use serde::Serialize;
use specta::Type;
use std::sync::Mutex;

//type DB = tauri::State<Mutex<DatabaseConnection>>;
//type User = tauri::State<Mutex<user::Model>>;

#[derive(FromQueryResult)]
struct DbEnrichedSecretAccount {
    id: i32,
    user_id: i32,
    created_at: String,
    account_name: String,
    industry: IndustryEnum,
    two_factor_auth: bool,
    recovery: Option<String>,
    bucket_name: String,
    bucket_color: String,
    institution_name: String,
}

#[derive(Serialize, Debug, Type)]
pub struct UnlockedSecretAccount {
    pub id: i32,
    pub created_at: String,
    pub account_name: String,
    pub industry: Industry,
    pub two_factor_auth: bool,
    pub recovery: Option<String>,
    pub bucket_name: String,
    pub bucket_color: String,
    pub institution_name: String,
    pub password: String,
}

impl UnlockedSecretAccount {
    fn from_account_data(account_data: DbEnrichedSecretAccount, secret: &str) -> Self {
        let industry = account_data.industry.into();
        let password = gen_pw(
            &account_data.institution_name,
            &industry,
            secret,
            &account_data.account_name,
        );
        Self {
            id: account_data.id,
            created_at: account_data.created_at,
            account_name: account_data.account_name,
            industry: industry,
            two_factor_auth: account_data.two_factor_auth,
            recovery: account_data.recovery,
            bucket_name: account_data.bucket_name,
            bucket_color: account_data.bucket_color,
            institution_name: account_data.institution_name,
            password,
        }
    }
}

#[tauri::command]
#[specta::specta]
pub async fn retrieve_secure_account(
    db: tauri::State<'_, DatabaseConnection>,
    user: tauri::State<'_, Mutex<user::Model>>,
    account_id: i32,
) -> Result<UnlockedSecretAccount, AccountError> {
    let user_id = user.lock().unwrap().id;
    drop(user);

    let account_data = Account::find_by_id(account_id)
        .column_as(account::Column::Id, "id")
        .column_as(account::Column::UserId, "user_id")
        .column_as(account::Column::CreatedAt, "created_at")
        .column_as(account::Column::AccountName, "account_name")
        .column_as(secure_account::Column::Industry, "industry")
        .column_as(account::Column::Recovery, "recovery")
        .column_as(account::Column::TwoFactorAuth, "two_factor_auth")
        .column_as(bucket::Column::Name, "bucket_name")
        .column_as(bucket::Column::Color, "bucket_color")
        .column_as(institution::Column::Name, "institution_name")
        .join(JoinType::LeftJoin, account::Relation::Bucket.def())
        .join(JoinType::LeftJoin, account::Relation::Institution.def())
        .join(JoinType::RightJoin, account::Relation::SecureAccount.def())
        .into_model::<DbEnrichedSecretAccount>()
        .one(db.inner())
        .await?
        .ok_or(AccountError::NotFound(account_id))?;

    // Check if user is authorized to access this account
    if account_data.user_id != user_id {
        return Err(AccountError::NotAuthorized);
    }
    let secret = "secret_val"; // TODO: get secret from user and limit it to max 10 chars
    let unlocked_account = UnlockedSecretAccount::from_account_data(account_data, secret);
    Ok(unlocked_account)
}

#[tauri::command]
#[specta::specta]
pub async fn add_secure_account(
    db: tauri::State<'_, DatabaseConnection>,
    user: tauri::State<'_, Mutex<user::Model>>,
    institution_name: &str,
    account_name: &str,
    industry: Industry,
    bucket_id: Option<i32>,
) -> Result<i32, AccountError> {
    let created = Utc::now().naive_utc();
    let user_id = user.lock().unwrap().id;
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
        created_at: Set(created),
        account_name: Set(account_name.to_owned()),
        user_id: Set(user_id),
        bucket_id: Set(bucket_id),
        institution_id: Set(institution.unwrap().id),
        ..Default::default()
    };

    let account = new_acc.insert(db.inner()).await?;

    let new_secure_account = secure_account::ActiveModel {
        industry: Set(industry.into()),
        account_id: Set(account.id),
        ..Default::default()
    };

    let secure_account = new_secure_account.insert(db.inner()).await?;

    Ok(secure_account.id)
}

#[tauri::command]
#[specta::specta]
pub async fn delete_and_create_new_secure_account() -> Result<(), AccountError> {
    todo!()
}

#[tauri::command]
#[specta::specta]
pub async fn delete_secure_account() -> Result<(), AccountError> {
    todo!()
}
