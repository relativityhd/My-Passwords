use serde::Deserialize;
use surrealdb::sql::Thing;
pub mod industry;
pub mod mode;
use surrealdb::engine::remote::ws::Client;
use surrealdb::Surreal;

pub use industry::Industry;
pub use mode::{InvalidModeError, Mode};

#[derive(Debug, Deserialize)]
pub struct Record {
    pub id: Thing,
}

pub type DB<'a> = tauri::State<'a, Surreal<Client>>;
