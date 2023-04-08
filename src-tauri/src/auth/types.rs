use tauri::State;
use argon2::Argon2;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use chacha20poly1305::XChaCha20Poly1305;

pub type DBConnectionPool = Pool<ConnectionManager<MysqlConnection>>;
pub type ConnectionPool<'a> = State<'a, DBConnectionPool>;
pub type Cipher = XChaCha20Poly1305;
pub type Hasher<'a> = Argon2<'a>;
