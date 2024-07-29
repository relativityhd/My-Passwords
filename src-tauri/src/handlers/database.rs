use crate::errors::DatabaseError;
use crate::types::DB;
use once_cell::sync::Lazy;
use std::path::PathBuf;
use surrealdb::engine::remote::ws::Ws;
use surrealdb::error::Api;
use surrealdb::Error;
use surrealdb::Surreal;
use tauri::http::status;
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

async fn store_db_url(app_data_dir: Option<PathBuf>, url: &str) -> Result<(), DatabaseError> {
    print!("Storing db url: {}", url);
    let dir = app_data_dir.ok_or(DatabaseError::AppDataNotFound)?;
    dbg!(&dir);
    if !dir.exists() {
        tokio::fs::create_dir_all(&dir).await?;
    }
    let mut file = File::create(dir.join("dburl.txt")).await?;
    file.write_all(url.as_bytes()).await?;
    Ok(())
}

async fn validate_url(url: &str) -> Result<(), DatabaseError> {
    let db = Surreal::new::<Ws>(url).await?;
    let dbname = match cfg!(dev_mode) {
        true => "dev",
        false => "prod",
    };
    db.use_ns("accounts").use_db(dbname).await?;

    db.health().await?;

    let version = env!("CARGO_PKG_VERSION");
    let found = db
        .query("$version")
        .await?
        .take::<Option<String>>(0)?
        .ok_or(DatabaseError::NoVersion)?;
    if version != found {
        return Err(DatabaseError::VersionMismatch {
            expected: version.to_string(),
            found,
        });
    }
    Ok(())
}

#[tauri::command]
#[specta::specta]
pub async fn load_db(app_handle: tauri::AppHandle, db: DB<'_>) -> Result<bool, DatabaseError> {
    println!("Loading db url");
    let fpath = app_handle
        .path_resolver()
        .app_data_dir()
        .ok_or(DatabaseError::AppDataNotFound)?
        .join("dburl.txt");
    if !fpath.exists() {
        return Ok(false);
    }
    let mut file = File::open(fpath).await?;
    let mut contents = vec![];
    file.read_to_end(&mut contents).await?;
    let url = String::from_utf8(contents)?;

    validate_url(&url).await?;

    db.connect::<Ws>(url).await?;
    let dbname = match cfg!(dev_mode) {
        true => "dev",
        false => "prod",
    };
    db.use_ns("accounts").use_db(dbname).await?;
    Ok(true)
}

#[tauri::command]
#[specta::specta]
pub async fn check_connection(url: &str) -> Result<(), DatabaseError> {
    validate_url(url).await
}

#[tauri::command]
#[specta::specta]
pub async fn connect(
    app_handle: tauri::AppHandle,
    db: DB<'_>,
    url: &str,
) -> Result<(), DatabaseError> {
    validate_url(&url).await?;
    store_db_url(app_handle.path_resolver().app_data_dir(), url).await?;

    db.connect::<Ws>(url).await?;
    let dbname = match cfg!(dev_mode) {
        true => "dev",
        false => "prod",
    };
    db.use_ns("accounts").use_db(dbname).await?;
    Ok(())
}
