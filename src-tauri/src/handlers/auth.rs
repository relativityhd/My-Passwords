use crate::common::get_user;
use crate::errors::{AuthError, CookieError, LocalCredsError, UnauthenticatedError};
use crate::types::{LocalCreds, DB, LC};
use log::debug;
use orion::aead;
use serde::Serialize;
use serde_json;
use std::path::PathBuf;
use surrealdb::opt::auth::{Jwt, Scope};
use surrealdb::sql::{self, Thing};
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[derive(Serialize)]
struct Credentials<'a> {
    email: &'a str,
    username: &'a str,
    password: &'a str,
}

// -- Cookie handling --
async fn store_cookie(app_data_dir: Option<PathBuf>, token: Jwt) -> Result<(), CookieError> {
    let dir = app_data_dir.ok_or(CookieError::AppDataNotFound)?;
    debug!("Storing cookie in {:?}", &dir);
    if !dir.exists() {
        tokio::fs::create_dir_all(&dir).await?;
    }
    let mut file = File::create(dir.join("session-cookie.jwt")).await?;
    file.write_all(token.as_insecure_token().as_bytes()).await?;
    Ok(())
}

async fn get_token_from_cookie(app_data_dir: Option<PathBuf>) -> Result<Option<Jwt>, CookieError> {
    let fpath = app_data_dir
        .ok_or(CookieError::AppDataNotFound)?
        .join("session-cookie.jwt");
    if !fpath.exists() {
        debug!("No cookie found");
        return Ok(None);
    }
    debug!("Reading cookie from {:?}", &fpath);
    let mut file = File::open(fpath).await?;
    let mut contents = vec![];
    file.read_to_end(&mut contents).await?;
    let token = Jwt::from(String::from_utf8(contents)?);
    Ok(Some(token))
}

async fn delete_cookie(app_data_dir: Option<PathBuf>) -> Result<(), CookieError> {
    let fpath = app_data_dir
        .ok_or(CookieError::AppDataNotFound)?
        .join("session-cookie.jwt");
    if fpath.exists() {
        debug!("Deleting cookie at {:?}", &fpath);
        std::fs::remove_file(fpath)?;
    }
    Ok(())
}

// -- Auth commands --
#[tauri::command]
#[specta::specta]
pub async fn is_authenticated(app_handle: tauri::AppHandle, db: DB<'_>) -> Result<bool, AuthError> {
    let auth: Option<Thing> = db.query("RETURN $auth;").await?.take(0)?;
    // Be optimistic and assume that the user is authenticated
    if let Some(id) = auth {
        debug!("User is authenticated: {}", id);
        return Ok(true);
    }

    // If the user is not authenticated, try to authenticate with cookie
    debug!("User is not authenticated, try to authenticate with cookie");
    let app_data_dir = app_handle.path_resolver().app_data_dir();
    let token = get_token_from_cookie(app_data_dir).await?;
    if let Some(token) = token {
        // TODO: Check if token is still valid and invalidate if not
        // Also extract potential invalidation errors from the authenticate method and put it in a separate error type
        db.authenticate(token).await?;
        debug!("User is now authenticated");
        return Ok(true);
    }

    debug!("User is not authenticated");
    // If the user is not authenticated and no cookie is found, return false
    Ok(false)
}

#[tauri::command]
#[specta::specta]
pub async fn signin(
    app_handle: tauri::AppHandle,
    db: DB<'_>,
    identity: &str,
    password: &str,
    remember: bool,
) -> Result<(), AuthError> {
    let token = db
        .signin(Scope {
            namespace: "accounts",
            database: "dev",
            scope: "user",
            params: Credentials {
                username: &identity,
                email: &identity,
                password: &password,
            },
        })
        .await
        .map_err(|e| match e {
            surrealdb::Error::Api(surrealdb::error::Api::Query(_msg)) => {
                debug!("Invalid credentials: {:?}", _msg);
                AuthError::InvalidCredentials
            }
            _ => AuthError::Db(e),
        })?;
    db.authenticate(token.clone()).await?;
    if remember {
        store_cookie(app_handle.path_resolver().app_data_dir(), token).await?;
    }
    Ok(())
}

#[tauri::command]
#[specta::specta]
pub async fn signup(
    app_handle: tauri::AppHandle,
    db: DB<'_>,
    email: &str,
    username: &str,
    password: &str,
    remember: bool,
) -> Result<(), AuthError> {
    let token = db
        .signup(Scope {
            namespace: "accounts",
            database: "dev",
            scope: "user",
            params: Credentials {
                username: &username,
                email: &email,
                password: &password,
            },
        })
        .await
        .map_err(|e| match e {
            surrealdb::Error::Api(surrealdb::error::Api::Query(_msg)) => {
                debug!("Invalid credentials: {:?}", _msg);
                AuthError::InvalidCredentials
            }
            _ => AuthError::Db(e),
        })?;
    db.authenticate(token.clone()).await?;
    if remember {
        store_cookie(app_handle.path_resolver().app_data_dir(), token).await?;
    }
    Ok(())
}

#[tauri::command]
#[specta::specta]
pub async fn signout(
    app_handle: tauri::AppHandle,
    db: DB<'_>,
    lc: LC<'_>,
) -> Result<(), AuthError> {
    let app_data_dir = app_handle.path_resolver().app_data_dir();
    delete_cookie(app_data_dir).await?;
    db.invalidate().await?;
    invalidate_credentials(lc);
    Ok(())
}

// -- Local credentials handling --
fn invalidate_credentials(lc: LC<'_>) {
    // It is okay to unwrap the lock, because 1. if the lock is poisoned, the program is already in a bad state and we WANT to panic, 2. this application is not multi-threaded, hence poisoning is not possible in the first place
    let mut state = lc.lock().unwrap();
    *state = None;
}

#[tauri::command]
#[specta::specta]
pub async fn has_lc(
    app_handle: tauri::AppHandle,
    db: DB<'_>,
    lc: LC<'_>,
) -> Result<bool, LocalCredsError> {
    // Check if lc is already in memory
    if lc.lock().unwrap().is_some() {
        return Ok(true);
    }

    // Check if lc is stored in cache
    let auth = get_user(db.clone()).await?;
    let fname = format!("{}.cache", auth.id);
    let fpath = app_handle
        .path_resolver()
        .app_data_dir()
        .ok_or(LocalCredsError::AppDataNotFound)?
        .join(fname);
    if !fpath.exists() {
        debug!("No lc found");
        return Ok(false);
    }

    debug!("Reading lc from {:?}", &fpath);

    // Use hashed password from user to encrypt lc
    let userpw = db
        .query("(SELECT password FROM ONLY $auth).password;")
        .await?
        .take::<Option<String>>(0)?
        .ok_or(LocalCredsError::NoPasswordFound)?;

    // Generate cipher from userpw
    let key = aead::SecretKey::from_slice(&userpw.as_bytes()[..32])?;

    let mut file = File::open(fpath).await?;
    let mut contents = vec![];
    file.read_to_end(&mut contents).await?;

    let serialized_lc = aead::open(&key, &contents)?;

    // Deserialize lc
    let newlc: LocalCreds = serde_json::from_slice(&serialized_lc)?;

    let mut state = lc.lock().unwrap();
    *state = Some(newlc);
    Ok(true)
}

#[tauri::command]
#[specta::specta]
pub async fn store_lc(
    app_handle: tauri::AppHandle,
    db: DB<'_>,
    lc: LC<'_>,
    newlc: LocalCreds,
) -> Result<(), LocalCredsError> {
    let auth = get_user(db.clone()).await?;

    let dir = app_handle
        .path_resolver()
        .app_data_dir()
        .ok_or(LocalCredsError::AppDataNotFound)?;
    if !dir.exists() {
        tokio::fs::create_dir_all(&dir).await?;
    }
    debug!("Storing lc in {:?}", &dir);
    let fname = format!("{}.cache", auth.id);
    // Use hashed password from user to encrypt lc
    let userpw = db
        .query("(SELECT password FROM ONLY $auth).password;")
        .await?
        .take::<Option<String>>(0)?
        .ok_or(LocalCredsError::NoPasswordFound)?;

    // Generate cipher from userpw
    let key = aead::SecretKey::from_slice(&userpw.as_bytes()[..32])?;

    let serialized_lc = serde_json::to_string(&newlc)?;
    let encrypted_lc = aead::seal(&key, serialized_lc.as_bytes())?;

    let mut file = File::create(dir.join(fname)).await?;

    // Convert lc to bytes
    file.write_all(&encrypted_lc).await?;

    let mut state = lc.lock().unwrap();
    *state = Some(newlc);
    Ok(())
}

#[tauri::command]
#[specta::specta]
pub async fn get_username(db: DB<'_>) -> Result<String, UnauthenticatedError> {
    let sql = "(SELECT username FROM ONLY $auth).username;";
    let username: String = db
        .query(sql)
        .await?
        .take::<Option<String>>(0)?
        .ok_or(UnauthenticatedError::NotSignedIn)?;
    Ok(username)
}
