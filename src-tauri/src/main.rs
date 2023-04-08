#![allow(dead_code)]
#![allow(unused_doc_comments)]
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use dotenvy::dotenv;

mod generator;
mod model;
mod schema;
mod database;
mod handlers;
mod auth;

fn main() {
    dotenv().ok();
    let pool = database::create_connection_pool();
    tauri::Builder::default()
        .plugin(auth::init(pool.clone()))
        .manage(pool)
        .invoke_handler(tauri::generate_handler![
            handlers::greet,
            handlers::gen_pw,
            handlers::gen_legacy_pw])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
