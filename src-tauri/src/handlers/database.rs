use crate::errors::DatabaseError;
use crate::types::DB;
use log::debug;
use once_cell::sync::Lazy;
use std::path::PathBuf;
use surrealdb::engine::remote::ws::{Ws, Wss};
use surrealdb::error::Api;
use surrealdb::Error;
use surrealdb::Surreal;
use tauri::http::status;
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[cfg(debug_assertions)]
const DBNAME: &str = "dev";

#[cfg(not(debug_assertions))]
const DBNAME: &str = "prod";

const VERSION: &str = env!("CARGO_PKG_VERSION");

async fn store_db_url(app_data_dir: Option<PathBuf>, url: &str) -> Result<(), DatabaseError> {
    let dir = app_data_dir.ok_or(DatabaseError::AppDataNotFound)?;
    debug!("Storing db url in {:?}", &dir);
    if !dir.exists() {
        tokio::fs::create_dir_all(&dir).await?;
    }
    let mut file = File::create(dir.join("dburl.txt")).await?;
    file.write_all(url.as_bytes()).await?;
    Ok(())
}

#[derive(Debug)]
enum Scheme {
    Ws,
    Wss,
}

fn parse_url(url: &str) -> Result<(Scheme, &str), DatabaseError> {
    let mut parts = url.split("://");
    let scheme = parts.next().ok_or(DatabaseError::InvalidUrl)?;
    let scheme = match scheme {
        "ws" => Scheme::Ws,
        "wss" => Scheme::Wss,
        _ => return Err(DatabaseError::InvalidUrl),
    };
    let host = parts.next().ok_or(DatabaseError::InvalidUrl)?;
    debug!("Parsed url: {:?} {}", scheme, host);
    Ok((scheme, host))
}

async fn validate_url(url: &str) -> Result<(), DatabaseError> {
    let (scheme, host) = parse_url(url)?;
    let db = match scheme {
        Scheme::Ws => Surreal::new::<Ws>(host).await?,
        Scheme::Wss => Surreal::new::<Wss>(host).await?,
    };
    db.use_ns("accounts").use_db(DBNAME).await?;

    debug!("Checking db health");
    db.health().await?;

    let version = env!("CARGO_PKG_VERSION");
    let found = db
        .query("$version")
        .await?
        .take::<Option<String>>(0)?
        .ok_or(DatabaseError::NoVersion)?;
    debug!("Found version: {}, expected: {}", found, version);
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
    validate_url(url).await?;
    store_db_url(app_handle.path_resolver().app_data_dir(), url).await?;
    let (scheme, host) = parse_url(url)?;
    let db = match scheme {
        Scheme::Ws => Surreal::new::<Ws>(host).await?,
        Scheme::Wss => Surreal::new::<Wss>(host).await?,
    };

    db.use_ns("accounts").use_db(DBNAME).await?;
    Ok(())
}

#[tauri::command]
#[specta::specta]
pub async fn is_connected(app_handle: tauri::AppHandle, db: DB<'_>) -> Result<bool, DatabaseError> {
    debug!("Checking if db is connected");
    // Optimistic check
    if (db.health().await.is_ok()) {
        debug!("Db is connected");
        return Ok(true);
    }
    // Now check if we can load the db url from file
    let fpath = app_handle
        .path_resolver()
        .app_data_dir()
        .ok_or(DatabaseError::AppDataNotFound)?
        .join("dburl.txt");
    if !fpath.exists() {
        // In this case the URL is completly unknown
        return Ok(false);
    }
    debug!("Reading db url from {:?}", &fpath);
    let mut file = File::open(fpath).await?;
    let mut contents = vec![];
    file.read_to_end(&mut contents).await?;
    let url = String::from_utf8(contents)?;
    validate_url(&url).await?;
    let (scheme, host) = parse_url(&url)?;
    match scheme {
        Scheme::Ws => db.connect::<Ws>(host).await?,
        Scheme::Wss => db.connect::<Wss>(host).await?,
    };
    db.use_ns("accounts").use_db(DBNAME).await?;
    debug!("Db is connected");
    Ok(db.health().await.is_ok())
}

#[tauri::command]
#[specta::specta]
pub async fn version_info() -> Result<String, ()> {
    Ok(format!("v{VERSION}-{DBNAME}"))
}
