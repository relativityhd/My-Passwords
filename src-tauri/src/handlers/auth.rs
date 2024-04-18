use crate::common::get_user;
use crate::errors::AuthError;
use crate::types::{LocalCreds, DB, LC};
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

async fn store_cookie(app_data_dir: Option<PathBuf>, token: Jwt) -> Result<(), AuthError> {
    println!("Setting session cookie");
    let dir = app_data_dir.ok_or(AuthError::AppDataNotFound)?;
    dbg!(&dir);
    if !dir.exists() {
        tokio::fs::create_dir_all(&dir).await?;
    }
    let mut file = File::create(dir.join("session-cookie.jwt")).await?;
    file.write_all(token.as_insecure_token().as_bytes()).await?;
    Ok(())
}

#[tauri::command]
#[specta::specta]
pub async fn signout(
    app_handle: tauri::AppHandle,
    db: DB<'_>,
    lc: LC<'_>,
) -> Result<(), AuthError> {
    let auth = get_user(db.clone()).await?;
    println!("Sign out user: {}", auth);
    let fpath = app_handle
        .path_resolver()
        .app_data_dir()
        .ok_or(AuthError::AppDataNotFound)?
        .join("session-cookie.jwt");
    if fpath.exists() {
        std::fs::remove_file(fpath)?;
    }
    db.invalidate().await?;

    let mut state = lc.lock().await;
    *state = None;
    println!("User signed out");
    Ok(())
}

#[tauri::command]
#[specta::specta]
pub async fn is_authenticated(app_handle: tauri::AppHandle, db: DB<'_>) -> Result<bool, AuthError> {
    let auth: Option<Thing> = db.query("RETURN $auth;").await?.take(0)?;
    if let Some(id) = auth {
        println!("User is authenticated: {}", id);
        return Ok(true);
    } else {
        println!("User is not authenticated, try to authenticate with cookie");
        let fpath = app_handle
            .path_resolver()
            .app_data_dir()
            .ok_or(AuthError::AppDataNotFound)?
            .join("session-cookie.jwt");
        if !fpath.exists() {
            println!("No cookie found");
            return Ok(false);
        }
        let mut file = File::open(fpath).await?;
        let mut contents = vec![];
        file.read_to_end(&mut contents).await?;
        let token = Jwt::from(String::from_utf8(contents)?);
        db.authenticate(token).await?;
        print!("User is now authenticated");
        return Ok(true);
    }
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
    print!("Signing in...");
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
        .await?;
    db.authenticate(token.clone()).await?;
    println!("Logged in!");
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
        .await?;
    db.authenticate(token.clone()).await?;
    println!("Signed up!");
    if remember {
        store_cookie(app_handle.path_resolver().app_data_dir(), token).await?;
    }
    Ok(())
}

#[tauri::command]
#[specta::specta]
pub async fn has_lc(
    app_handle: tauri::AppHandle,
    db: DB<'_>,
    lc: LC<'_>,
) -> Result<bool, AuthError> {
    let auth = get_user(db.clone()).await?;

    let state = lc.lock().await;
    if state.is_some() {
        return Ok(true);
    }
    drop(state); // Free the lock

    let fname = format!("{}.cache", auth.id);
    let fpath = app_handle
        .path_resolver()
        .app_data_dir()
        .ok_or(AuthError::AppDataNotFound)?
        .join(fname);
    if !fpath.exists() {
        println!("No lc found");
        return Ok(false);
    }

    // Use hashed password from user to encrypt lc
    let userpw = db
        .query("(SELECT password FROM ONLY $auth).password;")
        .await?
        .take::<Option<String>>(0)?
        .ok_or(AuthError::NoPassword)?;

    // Generate cipher from userpw
    let key = aead::SecretKey::from_slice(&userpw.as_bytes()[..32])?;

    let mut file = File::open(fpath).await?;
    let mut contents = vec![];
    file.read_to_end(&mut contents).await?;

    let serialized_lc = aead::open(&key, &contents)?;

    // Deserialize lc
    let newlc: LocalCreds = serde_json::from_slice(&serialized_lc)?;

    let mut state = lc.lock().await;
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
) -> Result<(), AuthError> {
    let auth = get_user(db.clone()).await?;

    println!("Setting lc");
    let dir = app_handle
        .path_resolver()
        .app_data_dir()
        .ok_or(AuthError::AppDataNotFound)?;
    dbg!(&dir);
    if !dir.exists() {
        tokio::fs::create_dir_all(&dir).await?;
    }
    let fname = format!("{}.cache", auth.id);
    // Use hashed password from user to encrypt lc
    let userpw = db
        .query("(SELECT password FROM ONLY $auth).password;")
        .await?
        .take::<Option<String>>(0)?
        .ok_or(AuthError::NoPassword)?;

    // Generate cipher from userpw
    let key = aead::SecretKey::from_slice(&userpw.as_bytes()[..32])?;

    let serialized_lc = serde_json::to_string(&newlc)?;
    let encrypted_lc = aead::seal(&key, serialized_lc.as_bytes())?;

    let mut file = File::create(dir.join(fname)).await?;

    // Convert lc to bytes
    file.write_all(&encrypted_lc).await?;

    let mut state = lc.lock().await;
    *state = Some(newlc);
    Ok(())
}

#[tauri::command]
#[specta::specta]
pub async fn get_username(db: DB<'_>) -> Result<String, AuthError> {
    let sql = "(SELECT username FROM ONLY $auth).username;";
    let username: String = db
        .query(sql)
        .await?
        .take::<Option<String>>(0)?
        .ok_or(AuthError::NotSignedIn)?;
    Ok(username)
}
