#![allow(dead_code)]
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod generator;
mod model;
mod database;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn gen_pw(institution: &str, industry: model::Industry, secret: &str, account_name: &str) -> String {
    generator::gen_pw(institution, &industry, secret, account_name)
}

#[tauri::command]
fn gen_legacy_pw(institution: &str, industry: model::Industry, secret: &str) -> String {
    generator::gen_legacy_pw(institution, &industry, secret)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, gen_pw, gen_legacy_pw])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
