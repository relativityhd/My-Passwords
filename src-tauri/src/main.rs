#![allow(dead_code)]
// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod algorithm;
mod common;
mod errors;
mod handlers;
mod types;

use handlers::*;
use log::debug;
use log::LevelFilter;
use once_cell::sync::Lazy;
use specta::collect_types;
use std::sync::Arc;
use std::sync::Mutex;
use surrealdb::engine::remote::ws::Client;
use surrealdb::Surreal;
use tauri::Manager;
use tauri_plugin_log::fern::colors::ColoredLevelConfig;
use tauri_plugin_log::LogTarget;
use tauri_specta::ts;
use types::LocalCreds;

#[cfg(debug_assertions)]
const LOG_TARGETS: [LogTarget; 3] = [LogTarget::Stdout, LogTarget::Webview, LogTarget::LogDir];

#[cfg(not(debug_assertions))]
const LOG_TARGETS: [LogTarget; 2] = [LogTarget::Stdout, LogTarget::LogDir];

#[cfg(debug_assertions)]
const LOG_LEVEL: LevelFilter = LevelFilter::Debug;

#[cfg(not(debug_assertions))]
const LOG_LEVEL: LevelFilter = LevelFilter::Debug;

fn setup_log_dir(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    let log_dir = app
        .handle()
        .path_resolver()
        .app_log_dir()
        .expect("Valid log dir");
    std::fs::create_dir_all(log_dir)?;
    Ok(())
}

#[tokio::main]
async fn main() {
    if cfg!(debug_assertions) {
        debug!("Running in dev mode, generating bindings...");
        ts::export(
            collect_types![
                database::check_connection,
                database::connect,
                database::is_connected,
                database::version_info,
                auth::signin,
                auth::signup,
                auth::signout,
                auth::is_authenticated,
                auth::has_lc,
                auth::store_lc,
                auth::get_username,
                buckets::create_bucket,
                buckets::get_buckets,
                buckets::delete_bucket,
                search::search,
                search::search_bucket,
                accounts::get_mode,
                accounts::in_sso_use,
                accounts::delete_account,
                accounts::get_all_accounts,
                accounts::get_popular,
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
                accounts::legacy::get_legacy_overview,
                accounts::legacy::load_from_json
            ],
            "../src/lib/bindings.ts",
        )
        .expect("Type export to just work...");
    }

    let db: Lazy<Surreal<Client>> = Lazy::new(Surreal::init);

    let pin = Mutex::new(None::<LocalCreds>);

    tauri::Builder::default()
        .setup(setup_log_dir)
        .plugin(
            tauri_plugin_log::Builder::default()
                .targets(LOG_TARGETS)
                .with_colors(ColoredLevelConfig::default())
                .level_for("tungstenite", LevelFilter::Warn)
                .level_for("tokio_tungstenite", LevelFilter::Warn)
                .level_for("rustls", LevelFilter::Warn)
                .level(LOG_LEVEL)
                .build(),
        )
        .manage(db)
        .manage(pin)
        .invoke_handler(tauri::generate_handler![
            database::check_connection,
            database::connect,
            database::is_connected,
            database::version_info,
            auth::signin,
            auth::signup,
            auth::signout,
            auth::is_authenticated,
            auth::has_lc,
            auth::store_lc,
            auth::get_username,
            buckets::create_bucket,
            buckets::get_buckets,
            buckets::delete_bucket,
            search::search,
            search::search_bucket,
            accounts::get_mode,
            accounts::in_sso_use,
            accounts::delete_account,
            accounts::get_all_accounts,
            accounts::get_popular,
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
            accounts::legacy::get_legacy_overview,
            accounts::legacy::load_from_json
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
