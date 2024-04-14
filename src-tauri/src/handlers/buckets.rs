use crate::errors::BucketError;
use crate::types::{handlers::Bucket, Record, DB};
use once_cell::sync::Lazy;
use regex::Regex;
use surrealdb::sql::Thing;

static COLOR_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"^#[0-9A-Fa-f]{6}$").unwrap());

#[tauri::command]
#[specta::specta]
pub async fn create_bucket(
    db: DB<'_>,
    bucket_name: &str,
    bucket_color: &str,
) -> Result<String, BucketError> {
    if !COLOR_RE.is_match(bucket_color) {
        return Err(BucketError::InvalidColor(bucket_color.to_string()));
    }

    let sql = "fn::create_bucket($name, $color);";
    let bucket = db
        .query(sql)
        .bind(("name", bucket_name))
        .bind(("color", bucket_color))
        .await?
        .take::<Option<Thing>>(0)?
        .ok_or(BucketError::NoID)?;

    Ok(bucket.id.to_string())
}

#[tauri::command]
#[specta::specta]
pub async fn get_buckets(db: DB<'_>) -> Result<Vec<Bucket>, BucketError> {
    let sql = "SELECT type::string(id) as id, name, color, array::len(<-is_sorted_in<-account) as n FROM bucket;";
    let buckets: Vec<Bucket> = db.query(sql).await?.take(0)?;
    Ok(buckets)
}

// TODO: Edit bucket

#[tauri::command]
#[specta::specta]
pub async fn check_bucket_is_empty(db: DB<'_>, bucket_id: &str) -> Result<bool, BucketError> {
    let bucket_id = bucket_id.split(':').last().unwrap();
    let sql =
        "RETURN array::len(SELECT * FROM is_sorted_in WHERE out = type::thing('bucket', $bucket));";
    let n: Option<u32> = db.query(sql).bind(("bucket", bucket_id)).await?.take(0)?;
    let n = n.ok_or(BucketError::NotFound(bucket_id.to_string()))?;
    Ok(n == 0)
}

#[tauri::command]
#[specta::specta]
pub async fn delete_bucket(db: DB<'_>, bucket_id: &str) -> Result<(), BucketError> {
    let bucket_id = bucket_id.split(':').last().unwrap();
    let sql = "fn::delete_bucket(type::thing('bucket', $bucket));";
    db.query(sql)
        .bind(("bucket", bucket_id))
        .await?
        .take::<Option<Record>>(0)?;
    Ok(())
}
