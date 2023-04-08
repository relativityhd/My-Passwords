use std::{str, io, path::PathBuf, fmt::Debug};
use chacha20poly1305::{aead, XNonce};
use r2d2;
use diesel;
use serde_json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AuthError {
  #[error("Unknown username")]
  UnknownUsername,
  #[error("Invalid password")]
  InvalidPassword,
  #[error("Can't get connection from database connection pool!")]
  FailedConnection(#[from] r2d2::Error),
  #[error("Can't query database at {1}")]
  FailedQuery(#[source] diesel::result::Error, &'static str),
  #[error("Can't parse and hash password!")]
  Unhashable(#[from] argon2::password_hash::Error),
  #[error("An error with the cookie occured!")]
  Cookie(#[from] CookieError),
}


#[derive(Error, Debug)]
pub enum CookieError {
  #[error("Can't read cookie! Error while {1} at {}", .2.as_path().display())]
  IO(#[source] io::Error, &'static str, PathBuf), // (IO-Error, action, path)
  #[error("Can't encrypt cookie!\nNonce: {}\nCookie: {2}", str::from_utf8(.1).unwrap_or("##UTF-8 Unrepresentable##"))]
  Encrypt(#[source] aead::Error, XNonce, String),
  #[error("Can't decrypt cookie!\nNonce: {}\nCookie: {2}", str::from_utf8(.1).unwrap_or("##UTF-8 Unrepresentable##"))]
  Decrypt(#[source] aead::Error, XNonce, String),
  #[error("Cookie is unrepresentable in UTF-8!\nCookie: {1:?}")]
  Unrepresentable(#[source] std::str::Utf8Error, Vec<u8>),
  #[error("Can't parse/serialize Cookie!\nCookie: {1}")]
  Unparsable(#[source] serde_json::Error, String),
  #[error("The cookie was not populated yet!")]
  Unpopulated
}
