use serde::Deserialize;
use specta::Type;
use surrealdb::sql::Thing;
pub mod industry;
pub mod mode;
use crate::errors::AccountError;
use serde::Serialize;
use std::sync::Arc;
use std::sync::Mutex;
use surrealdb::engine::remote::ws::Client;
use surrealdb::Surreal;

pub mod handlers;
pub use industry::Industry;
pub use mode::Mode;

#[derive(Debug, Deserialize)]
pub struct Record {
    pub id: Thing,
}

#[derive(Serialize, Deserialize, Clone, Type)]
pub(crate) struct LocalCreds {
    #[serde(skip_serializing)]
    pub pin: u32,
    #[serde(skip_serializing)]
    pub secret: String,
}

pub type DB<'a> = tauri::State<'a, Surreal<Client>>;
pub type LC<'a> = tauri::State<'a, Mutex<Option<LocalCreds>>>;

pub async fn extract_lc(lc: &LC<'_>) -> Result<LocalCreds, AccountError> {
    let state = lc.lock()?;
    state.clone().ok_or(AccountError::PinNotFound)
}
