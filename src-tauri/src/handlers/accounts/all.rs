use super::types::AccountError;
use entities::*;
use sea_orm::*;
use serde::Serialize;
use specta::Type;
use std::sync::Mutex;

#[derive(Serialize, Debug, Type, FromQueryResult)]
pub struct Account {
    pub id: i32,
    pub created_at: String,
    pub account_name: String,
    pub bucket_name: String,
    pub bucket_color: String,
    pub institution_name: String,
}

#[tauri::command]
#[specta::specta]
pub async fn search_user_accounts() -> Result<(), AccountError> {
    todo!()
}

#[tauri::command]
#[specta::specta]
pub async fn list_user_accounts(
    db: tauri::State<'_, DatabaseConnection>,
    user: tauri::State<'_, Mutex<user::Model>>,
) -> Result<Vec<Account>, AccountError> {
    let user_id = user.lock().unwrap().id;
    drop(user);

    let accounts = account::Entity::find()
        .column_as(account::Column::Id, "id")
        .column_as(account::Column::CreatedAt, "created_at")
        .column_as(account::Column::AccountName, "account_name")
        .column_as(bucket::Column::Name, "bucket_name")
        .column_as(bucket::Column::Color, "bucket_color")
        .column_as(institution::Column::Name, "institution_name")
        .filter(account::Column::UserId.eq(user_id))
        .join(JoinType::LeftJoin, account::Relation::Bucket.def())
        .into_model::<Account>()
        .all(db.inner())
        .await?;

    /* let accounts = Account::find_by_statement(Statement::from_sql_and_values(
        DbBackend::Postgres,
        r#"
        SELECT
            a.id as id,
            a.created_at as created_at,
            a.account_name as account_name,
            b.name as bucket_name,
            b.color as bucket_color,
            i.name as institution_name
        FROM account a
            LEFT JOIN bucket b ON a.bucket_id = b.id
            LEFT JOIN institution i ON a.institution_id = i.id
        WHERE a.user_id = $1;
        "#,
        [user_id.into()],
    ))
    .all(db.inner())
    .await?; */

    Ok(accounts)
}

#[tauri::command]
#[specta::specta]
pub async fn move_account_to_bucket() -> Result<(), AccountError> {
    todo!()
}
