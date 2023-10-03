use serde::Serialize;
use surrealdb::engine::remote::ws::Client;
use surrealdb::opt::auth::Scope;
use surrealdb::sql::Thing;
use surrealdb::Surreal;
use thiserror::Error;

#[derive(Debug, Error, Serialize)]
pub enum AuthError {
    #[error("Database error: {0:?}")]
    Db(#[from] surrealdb::Error),
}

#[derive(Serialize)]
struct Credentials<'a> {
    email: &'a str,
    password: &'a str,
}

#[derive(Serialize)]
struct NewUser<'a> {
    username: &'a str,
    email: &'a str,
    password: &'a str,
}

#[tauri::command]
#[specta::specta]
pub async fn is_authenticated(db: tauri::State<'_, Surreal<Client>>) -> Result<bool, AuthError> {
    //let mut res = db.query("RETURN $auth;").await?.take(0)?;
    let auth: Option<Thing> = db.query("RETURN $auth;").await?.take(0)?;
    if let Some(id) = auth {
        println!("User is authenticated: {}", id);
        return Ok(true);
    } else {
        println!("User is not authenticated");
        return Ok(false);
    }
}

#[tauri::command]
#[specta::specta]
pub async fn signin(
    db: tauri::State<'_, Surreal<Client>>,
    email: &str,
    password: &str,
) -> Result<(), AuthError> {
    let token = db
        .signin(Scope {
            namespace: "accounts",
            database: "dev",
            scope: "user",
            params: Credentials {
                email: &email,
                password: &password,
            },
        })
        .await?;
    db.authenticate(token).await?;
    println!("Logged in!");
    Ok(())
}

#[tauri::command]
#[specta::specta]
pub async fn signup(
    db: tauri::State<'_, Surreal<Client>>,
    email: &str,
    username: &str,
    password: &str,
) -> Result<(), AuthError> {
    let token = db
        .signup(Scope {
            namespace: "accounts",
            database: "dev",
            scope: "user",
            params: NewUser {
                username: &username,
                email: &email,
                password: &password,
            },
        })
        .await?;
    db.authenticate(token).await?;
    println!("Signed up!");
    Ok(())
}
