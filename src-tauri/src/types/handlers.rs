use serde::{Deserialize, Serialize};
use specta::Type;

use crate::types::{Industry, Mode};

#[derive(Deserialize, Serialize, Type)]
pub struct SecureOverview {
    pub institution: String,
    pub industry: Industry,
    pub identity: String,
    pub mode: Mode,
    pub created: String,
    pub recovery: Option<String>,
    pub website: Option<String>,
    pub alias: Vec<String>,
    pub bucket: Option<Bucket>,
    pub twofactor: Option<TwoFactor>,
}

#[derive(Serialize, Deserialize, Type)]
pub struct Bucket {
    pub id: String,
    pub name: String,
    pub color: String,
    pub n: u32,
}

#[derive(Serialize, Deserialize, Type)]
pub struct TwoFactor {
    pub id: String,
    pub name: String,
    pub device: String,
}

#[derive(Serialize, Deserialize, Type)]
pub struct ResultBucket {
    pub name: String,
    pub color: String,
}

#[derive(Serialize, Deserialize, Type)]
pub struct ResultTwofactor {
    pub name: String,
    pub device: String,
}

#[derive(Serialize, Deserialize, Type)]
pub struct SearchResult {
    pub id: String,
    pub account_type: Mode,
    pub institution: String,
    pub identity: String,
    pub bucket: Option<ResultBucket>,
}

#[derive(Serialize, Deserialize, Type)]
pub struct ListResult {
    pub id: String,
    pub account_type: Mode,
    pub institution: String,
    pub identity: String,
    pub bucket: Option<ResultBucket>,
    pub twofactor: Option<ResultTwofactor>,
}
