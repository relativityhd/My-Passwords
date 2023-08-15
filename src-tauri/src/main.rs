#![allow(dead_code)]
// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod algorithm;
mod database;
mod entities;
mod handlers;
mod migrator;
mod types;

use entities::{prelude::*, *};
use sea_orm::*;
use specta::collect_types;
use std::sync::Mutex;
use tauri_specta::ts;

#[tokio::main]
async fn main() {
    ts::export(
        collect_types![
            handlers::accounts::add_acc,
            handlers::accounts::retrieve_account,
            handlers::accounts::get_user_accounts,
            handlers::buckets::create_bucket,
            handlers::buckets::recolor_bucket,
            handlers::buckets::rename_bucket,
            handlers::buckets::delete_bucket,
            handlers::buckets::get_user_buckets,
        ],
        "../src/lib/bindings.ts",
    )
    .expect("Type export to just work...");

    let db = database::establish_connection()
        .await
        .expect("a reachable database");

    let userid = 1;

    let user_res = User::insert(user::ActiveModel {
        id: Set(userid),
        username: Set("dev".to_owned()),
        pass: Set("dev".to_owned()),
        ..Default::default()
    })
    .exec(&db)
    .await;
    match user_res {
        Ok(_) => println!("User inserted"),
        Err(e) => println!("Error inserting user: {:?}", e),
    }
    let main_user = User::find_by_id(userid)
        .one(&db)
        .await
        .expect("Failed to find user")
        .unwrap();

    let bucket_res = Bucket::insert(bucket::ActiveModel {
        id: Set(1),
        name: Set("Main".to_owned()),
        color: Set("#000000".to_owned()),
        user_id: Set(main_user.id),
        ..Default::default()
    })
    .exec(&db)
    .await;
    match bucket_res {
        Ok(_) => println!("Bucket inserted"),
        Err(e) => println!("Error inserting bucket: {:?}", e),
    }

    tauri::Builder::default()
        .manage(db)
        .manage(Mutex::new(main_user))
        .invoke_handler(tauri::generate_handler![
            handlers::accounts::add_acc,
            handlers::accounts::retrieve_account,
            handlers::accounts::get_user_accounts,
            handlers::buckets::create_bucket,
            handlers::buckets::recolor_bucket,
            handlers::buckets::rename_bucket,
            handlers::buckets::delete_bucket,
            handlers::buckets::get_user_buckets,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
