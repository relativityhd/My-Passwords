#![allow(dead_code)]
// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod algorithm;
mod handlers;
mod types;

use handlers::*;
use specta::collect_types;
use surrealdb::engine::remote::ws::Ws;
use surrealdb::Surreal;
use tauri_specta::ts;

#[tokio::main]
async fn main() {
    ts::export(
        collect_types![
            auth::signin,
            auth::signup,
            auth::signout,
            auth::is_authenticated,
            accounts::secure::live_input,
            accounts::secure::create,
            /* add_secure_account,
            retrieve_secure_account,
            search_user_accounts,
            list_user_accounts,
            create_bucket,
            recolor_bucket,
            rename_bucket,
            delete_bucket,
            get_user_buckets, */
        ],
        "../src/lib/bindings.ts",
    )
    .expect("Type export to just work...");

    let db = Surreal::new::<Ws>("127.0.0.1:8000")
        .await
        .expect("Failed to connect to database");
    db.use_ns("accounts")
        .use_db("dev")
        .await
        .expect("Failed to use namespace");

    tauri::Builder::default()
        .manage(db)
        .invoke_handler(tauri::generate_handler![
            auth::signin,
            auth::signup,
            auth::signout,
            auth::is_authenticated,
            accounts::secure::live_input,
            accounts::secure::create,
            /* add_secure_account,
            retrieve_secure_account,
            search_user_accounts,
            list_user_accounts,
            create_bucket,
            recolor_bucket,
            rename_bucket,
            delete_bucket,
            get_user_buckets, */
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
