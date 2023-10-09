use std::path::PathBuf;

use serde::Serialize;
use surrealdb::engine::remote::ws::Client;
use surrealdb::opt::auth::{Jwt, Scope};
use surrealdb::sql::Thing;
use surrealdb::Surreal;
use thiserror::Error;
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("Database error: {0:?}")]
    Db(#[from] surrealdb::Error),
    #[error("IO error: {0:?}")]
    Io(#[from] std::io::Error),
    #[error("App data directory not found")]
    AppDataNotFound,
    #[error("Invalid UTF-8 in cookie file: {0:?}")]
    InvalidUtf8(#[from] std::string::FromUtf8Error),
    #[error("Not signed in")]
    NotSignedIn,
}

impl Serialize for AuthError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

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
    db: tauri::State<'_, Surreal<Client>>,
) -> Result<(), AuthError> {
    let auth: Thing = db
        .query("RETURN $auth;")
        .await?
        .take::<Option<Thing>>(0)?
        .ok_or(AuthError::NotSignedIn)?;
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
    println!("User signed out");
    Ok(())
}

#[tauri::command]
#[specta::specta]
pub async fn is_authenticated(
    app_handle: tauri::AppHandle,
    db: tauri::State<'_, Surreal<Client>>,
) -> Result<bool, AuthError> {
    //let mut res = db.query("RETURN $auth;").await?.take(0)?;
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
    db: tauri::State<'_, Surreal<Client>>,
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
    db: tauri::State<'_, Surreal<Client>>,
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
