use serde::Deserialize;
use surrealdb::sql::Thing;
pub mod industry;
pub mod mode;
use serde::Serialize;
use std::sync::Arc;
use surrealdb::engine::remote::ws::Client;
use surrealdb::Surreal;
use tokio::sync::Mutex;

pub use industry::Industry;
pub use mode::Mode;

#[derive(Debug, Deserialize)]
pub struct Record {
    pub id: Thing,
}

#[derive(Serialize, Clone)]
pub(crate) struct PinState {
    pub val: Option<usize>,
}

pub type DB<'a> = tauri::State<'a, Surreal<Client>>;
pub type PIN<'a> = tauri::State<'a, Arc<Mutex<PinState>>>;
