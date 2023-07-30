use crate::entities::{bucket, user};
use once_cell::sync::Lazy;
use regex::Regex;
use sea_orm::*;
use std::sync::Mutex;
use thiserror::Error;

static COLOR_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"/^#[0-9A-F]{6}$/i").unwrap());

#[derive(Debug, Error)]
pub enum BucketError {
    #[error("Database error from SeaORM: {0:?}")]
    Db(#[from] DbErr),
    #[error("Invalid Color: {0:?}, must be 6-digit hex code, e.g. \"#ff0000\"")]
    InvalidColor(String),
}

#[tauri::command]
#[specta::specta]
pub async fn create_bucket(
    db: tauri::State<'_, DatabaseConnection>,
    user: tauri::State<'_, Mutex<user::Model>>,
    bucket_name: &str,
    bucket_color: &str,
) -> Result<i32, BucketError> {
    let user_id = user.lock().unwrap().id;
    drop(user);
    if !COLOR_RE.is_match(bucket_color) {
        return Err(BucketError::InvalidColor(bucket_color.to_owned()));
    }
    let active_bucket = bucket::ActiveModel {
        user_id: Set(user_id),
        name: Set(bucket_name.to_owned()),
        color: Set(bucket_color.to_owned()),
        ..Default::default()
    };
    let bucket = active_bucket.insert(db.inner()).await?;
    Ok(bucket.id)
}

#[tauri::command]
#[specta::specta]
pub async fn recolor_bucket(
    db: tauri::State<'_, DatabaseConnection>,
    bucket_id: i32,
    bucket_color: &str,
) -> Result<i32, BucketError> {
    if !COLOR_RE.is_match(bucket_color) {
        return Err(BucketError::InvalidColor(bucket_color.to_owned()));
    }
    let active_bucket = bucket::ActiveModel {
        id: Set(bucket_id),
        color: Set(bucket_color.to_owned()),
        ..Default::default()
    };
    let bucket = active_bucket.update(db.inner()).await?;
    Ok(bucket.id)
}

#[tauri::command]
#[specta::specta]
pub async fn rename_bucket(
    db: tauri::State<'_, DatabaseConnection>,
    bucket_id: i32,
    bucket_name: &str,
) -> Result<i32, BucketError> {
    let active_bucket = bucket::ActiveModel {
        id: Set(bucket_id),
        name: Set(bucket_name.to_owned()),
        ..Default::default()
    };
    let bucket = active_bucket.update(db.inner()).await?;
    Ok(bucket.id)
}

#[tauri::command]
#[specta::specta]
pub async fn delete_bucket(
    db: tauri::State<'_, DatabaseConnection>,
    bucket_id: i32,
) -> Result<u64, BucketError> {
    let active_bucket = bucket::ActiveModel {
        id: Set(bucket_id),
        ..Default::default()
    };
    // Note: By deleting a bucket, its accounts will be set to "unsorted" = bucket_id is Null
    let res = active_bucket.delete(db.inner()).await?;
    Ok(res.rows_affected)
}

#[tauri::command]
#[specta::specta]
pub async fn get_user_buckets(
    db: tauri::State<'_, DatabaseConnection>,
    user: tauri::State<'_, Mutex<user::Model>>,
) -> Result<Vec<bucket::Model>, BucketError> {
    let user_id = user.lock().unwrap().id;
    drop(user);
    let buckets = bucket::Entity::find()
        .filter(bucket::Column::UserId.eq(user_id))
        .all(db.inner())
        .await?;
    Ok(buckets)
}
