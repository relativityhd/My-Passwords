use std::sync::Mutex;
use argon2::Argon2;
use tauri::{
  plugin::{Builder, TauriPlugin},
  Manager, Runtime,
};
use chacha20poly1305::{
  aead::KeyInit,
  XChaCha20Poly1305,
};
mod cookie;
mod error;
mod handlers;
mod session;
mod user;
mod types;

use self::{session::{Session}, types::DBConnectionPool};
use self::cookie::Cookie;

pub fn init<R: Runtime>(connection_pool: DBConnectionPool) -> TauriPlugin<R> {

  // TODO: Change with env variable at compile time: https://doc.rust-lang.org/std/macro.env.html
  let key = b"Very cryptic value of length 32!";
  let cipher = XChaCha20Poly1305::new_from_slice(key)
    .expect("Hardcoded key should be fine since it has exactly 32 UTF8 characters");

  let argon2 = Argon2::default();

  let cookie = Cookie::new(cipher);
  let session: Mutex<Session> = Mutex::new(Session::new(connection_pool, cookie, argon2));

  Builder::new("auth")
    .invoke_handler(tauri::generate_handler![handlers::check, handlers::signup])
    .setup(|app_handle| {
      // setup plugin specific state here
      app_handle.manage(session);
      Ok(())
    })
    .build()
}