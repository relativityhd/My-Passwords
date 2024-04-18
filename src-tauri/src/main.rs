#![allow(dead_code)]
// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod algorithm;
mod common;
mod errors;
mod handlers;
mod types;

use handlers::*;
use specta::collect_types;
use std::sync::Arc;
use surrealdb::engine::remote::ws::Ws;
use surrealdb::Surreal;
use tauri_specta::ts;
use tokio::sync::Mutex;
use types::LocalCreds;

#[tokio::main]
async fn main() {
    ts::export(
        collect_types![
            auth::signin,
            auth::signup,
            auth::signout,
            auth::is_authenticated,
            auth::has_lc,
            auth::store_lc,
            buckets::create_bucket,
            buckets::get_buckets,
            buckets::delete_bucket,
            search::search,
            search::search_bucket,
            accounts::get_mode,
            accounts::in_sso_use,
            accounts::delete_account,
            accounts::get_all_accounts,
            accounts::secure::secure_live_input,
            accounts::secure::get_secure_password,
            accounts::secure::get_secure_overview,
            accounts::secure::create_secure,
            accounts::secure::edit_secure,
            accounts::supersecure::supersecure_live_input,
            accounts::supersecure::get_supersecure_password,
            accounts::supersecure::get_supersecure_overview,
            accounts::supersecure::create_supersecure,
            accounts::supersecure::edit_supersecure,
            accounts::sso::get_sso_overview,
            accounts::sso::create_sso,
            accounts::sso::edit_sso,
            accounts::sso::list_nosso_accounts,
            accounts::legacy::get_legacy_password,
            accounts::legacy::get_legacy_overview
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

    let pin = Arc::new(Mutex::new(None::<LocalCreds>));

    tauri::Builder::default()
        .manage(db)
        .manage(pin)
        .invoke_handler(tauri::generate_handler![
            auth::signin,
            auth::signup,
            auth::signout,
            auth::is_authenticated,
            auth::has_lc,
            auth::store_lc,
            buckets::create_bucket,
            buckets::get_buckets,
            buckets::delete_bucket,
            search::search,
            search::search_bucket,
            accounts::get_mode,
            accounts::in_sso_use,
            accounts::delete_account,
            accounts::get_all_accounts,
            accounts::secure::secure_live_input,
            accounts::secure::get_secure_password,
            accounts::secure::get_secure_overview,
            accounts::secure::create_secure,
            accounts::secure::edit_secure,
            accounts::supersecure::supersecure_live_input,
            accounts::supersecure::get_supersecure_password,
            accounts::supersecure::get_supersecure_overview,
            accounts::supersecure::create_supersecure,
            accounts::supersecure::edit_supersecure,
            accounts::sso::get_sso_overview,
            accounts::sso::create_sso,
            accounts::sso::edit_sso,
            accounts::sso::list_nosso_accounts,
            accounts::legacy::get_legacy_password,
            accounts::legacy::get_legacy_overview
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
