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
            accounts::secure_live_input,
            accounts::create_secure,
            accounts::get_secure,
            accounts::delete_secure,
            accounts::search,
            accounts::search_bucket,
            buckets::create_bucket,
            buckets::get_buckets,
            buckets::recolor_bucket,
            buckets::rename_bucket,
            buckets::delete_bucket,
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
            accounts::secure_live_input,
            accounts::create_secure,
            accounts::get_secure,
            accounts::delete_secure,
            accounts::search,
            accounts::search_bucket,
            buckets::create_bucket,
            buckets::get_buckets,
            buckets::recolor_bucket,
            buckets::rename_bucket,
            buckets::delete_bucket,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
