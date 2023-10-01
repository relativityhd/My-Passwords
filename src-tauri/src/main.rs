#![allow(dead_code)]
// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod algorithm;
mod database;
mod handlers;
mod types;

use entities::{prelude::*, *};
use handlers::*;
use sea_orm::*;
use specta::collect_types;
use std::sync::Mutex;
use tauri_specta::ts;

#[tokio::main]
async fn main() {
    ts::export(
        collect_types![
            add_secure_account,
            retrieve_secure_account,
            search_user_accounts,
            list_user_accounts,
            create_bucket,
            recolor_bucket,
            rename_bucket,
            delete_bucket,
            get_user_buckets,
        ],
        "../src/lib/bindings.ts",
    )
    .expect("Type export to just work...");

    let db = database::establish_connection()
        .await
        .expect("a reachable database");

    let userid = 1;

    let user_res = User::insert(user::ActiveModel {
        id: ActiveValue::Set(userid),
        username: Set("dev".to_owned()),
        pass: Set("dev".to_owned()),
        ..Default::default()
    })
    .exec(&db)
    .await;
    match user_res {
        Ok(_) => println!("User inserted"),
        Err(e) => match e.sql_err() {
            Some(SqlErr::UniqueConstraintViolation(_)) => {
                println!("User already exists, won't insert again")
            }
            _ => println!("Error inserting user: {:?}", e),
        },
    }

    // Double check if inserted correctly
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
        Err(e) => match e.sql_err() {
            Some(SqlErr::UniqueConstraintViolation(_)) => {
                println!("Bucket already exists, won't insert again")
            }
            _ => println!("Error inserting bucket: {:?}", e),
        },
    }

    tauri::Builder::default()
        .manage(db)
        .manage(Mutex::new(main_user))
        .invoke_handler(tauri::generate_handler![
            add_secure_account,
            retrieve_secure_account,
            search_user_accounts,
            list_user_accounts,
            create_bucket,
            recolor_bucket,
            rename_bucket,
            delete_bucket,
            get_user_buckets,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
