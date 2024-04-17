use serde::Deserialize;
use specta::Type;
use surrealdb::sql::Thing;
pub mod industry;
pub mod mode;
use serde::Serialize;
use std::sync::Arc;
use surrealdb::engine::remote::ws::Client;
use surrealdb::Surreal;
use tokio::sync::Mutex;

pub mod handlers;
pub use industry::Industry;
pub use mode::Mode;

#[derive(Debug, Deserialize)]
pub struct Record {
    pub id: Thing,
}

#[derive(Serialize, Deserialize, Clone, Type)]
pub(crate) struct LocalCreds {
    pub pin: u32,
    pub secret: String,
}

pub type DB<'a> = tauri::State<'a, Surreal<Client>>;
pub type LC<'a> = tauri::State<'a, Arc<Mutex<Option<LocalCreds>>>>;
