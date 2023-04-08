// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
use diesel::prelude::MysqlConnection;
use diesel::r2d2::{ConnectionManager, Pool};

use super::model;
use super::generator;

type ConnectionPool<'a> = tauri::State<'a, Pool<ConnectionManager<MysqlConnection>>>;

/**
 * Functions:
 * - Login / Create Account
 * - Preview/Generate Password (new, legacy, super-secure legacy)
 * - Load Password-Meta
 * - Search Password-Meta
 * - Save Password-meta
 * - Delete Password-Meta
 * - Download Dump
 * - Upload Dump
 */

#[tauri::command]
pub fn greet(connection_pool: ConnectionPool, name: &str) -> String {
  let connection = connection_pool.get().expect("Can't get connection from database connection pool!");
  format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
pub fn gen_pw(institution: &str, industry: model::Industry, secret: &str, account_name: &str) -> String {
    generator::gen_pw(institution, &industry, secret, account_name)
}

#[tauri::command]
pub fn gen_legacy_pw(institution: &str, industry: model::Industry, secret: &str) -> String {
    generator::gen_legacy_pw(institution, &industry, secret)
}
