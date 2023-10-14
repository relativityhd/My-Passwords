use crate::common::get_user;
use crate::errors::BucketError;
use crate::types::{Record, DB};
use once_cell::sync::Lazy;
use regex::Regex;
use serde::{Deserialize, Serialize};
use specta::Type;
use surrealdb::sql::Thing;

static COLOR_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"/^#[0-9A-F]{6}$/i").unwrap());

#[derive(Debug, Serialize)]
struct NewBucket {
    name: String,
    color: String,
    user: Thing,
}

#[tauri::command]
#[specta::specta]
pub async fn create_bucket(
    db: DB<'_>,
    bucket_name: &str,
    bucket_color: &str,
) -> Result<String, BucketError> {
    let auth = get_user(db.clone()).await?;

    if !COLOR_RE.is_match(bucket_color) {
        return Err(BucketError::InvalidColor(bucket_color.to_string()));
    }

    let bucket: Vec<Record> = db
        .create("bucket")
        .content(NewBucket {
            name: bucket_name.to_string(),
            color: bucket_color.to_string(),
            user: auth,
        })
        .await?;

    Ok(bucket[0].id.id.to_string())
}

#[derive(Debug, Serialize, Deserialize, Type)]
pub struct Bucket {
    pub id: String,
    pub name: String,
    pub color: String,
}

#[tauri::command]
#[specta::specta]
pub async fn get_buckets(db: DB<'_>) -> Result<Vec<Bucket>, BucketError> {
    let buckets = db.select("bucket").await?;
    Ok(buckets)
}

#[tauri::command]
#[specta::specta]
pub async fn recolor_bucket(
    db: DB<'_>,
    bucket_id: &str,
    bucket_color: &str,
) -> Result<String, BucketError> {
    if !COLOR_RE.is_match(bucket_color) {
        return Err(BucketError::InvalidColor(bucket_color.to_string()));
    }
    let bucket = db
        .update::<Option<Record>>(("bucket", bucket_id))
        .content(("color", bucket_color.to_string()))
        .await?
        .ok_or(BucketError::NotFound(bucket_id.to_string()))?;
    Ok(bucket.id.id.to_string())
}

#[tauri::command]
#[specta::specta]
pub async fn rename_bucket(
    db: DB<'_>,
    bucket_id: &str,
    bucket_name: &str,
) -> Result<String, BucketError> {
    let bucket = db
        .update::<Option<Record>>(("bucket", bucket_id))
        .content(("name", bucket_name))
        .await?
        .ok_or(BucketError::NotFound(bucket_id.to_string()))?;
    Ok(bucket.id.id.to_string())
}

#[tauri::command]
#[specta::specta]
pub async fn delete_bucket(db: DB<'_>, bucket_id: &str) -> Result<String, BucketError> {
    // TODO: Check if bucket is empty

    let bucket = db
        .delete::<Option<Record>>(("bucket", bucket_id))
        .await?
        .ok_or(BucketError::NotFound(bucket_id.to_string()))?;
    Ok(bucket.id.id.to_string())
}
