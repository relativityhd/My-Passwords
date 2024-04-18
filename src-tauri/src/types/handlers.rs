use serde::{Deserialize, Serialize};
use specta::Type;

use crate::types::{Industry, Mode};

#[derive(Deserialize, Serialize, Type)]
pub struct SsoOverview {
    pub ssoaccount_id: String,
    pub ssoaccount_institution: String,
    pub institution: String,
    pub mode: Mode,
    pub created: String,
    pub recovery: Option<String>,
    pub website: Option<String>,
    pub alias: Vec<String>,
    pub bucket: Option<Bucket>,
    pub twofactor: Option<TwoFactor>,
}

#[derive(Deserialize, Serialize, Type)]
pub struct SuperSecureOverview {
    pub institution: String,
    pub industry: Industry,
    pub identity: String,
    pub specials: String,
    pub seed: u32,
    pub min: u32,
    pub max: u32,
    pub mode: Mode,
    pub created: String,
    pub recovery: Option<String>,
    pub website: Option<String>,
    pub alias: Vec<String>,
    pub bucket: Option<Bucket>,
    pub twofactor: Option<TwoFactor>,
}

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

#[derive(Deserialize, Serialize, Type)]
pub struct LegacyOverview {
    pub institution: String,
    pub industry: Industry,
    pub mode: Mode,
    pub created: String,
    pub recovery: Option<String>,
    pub website: Option<String>,
    pub alias: Vec<String>,
    pub bucket: Option<Bucket>,
    pub twofactor: Option<TwoFactor>,
}

#[derive(Serialize, Deserialize, Type)]
pub struct AccountMetadata {
    pub institution: String,
    pub recovery: Option<String>,
    pub website: Option<String>,
    pub alias: Vec<String>,
}

#[derive(Serialize, Deserialize, Type)]
pub struct SuperSecureSpecifics {
    pub industry: Industry,
    pub identity: String,
    pub specials: String,
    pub seed: u32,
    pub min: u32,
    pub max: u32,
}

#[derive(Serialize, Deserialize, Type)]
pub struct SecureSpecifics {
    pub industry: Industry,
    pub identity: String,
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
pub struct PopularResult {
    pub id: String,
    pub account_type: Mode,
    pub institution: String,
    pub identity: String,
    pub bucket: Option<ResultBucket>,
    pub calls: u32,
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

#[derive(Serialize, Deserialize, Type)]
pub struct SsoListResult {
    pub id: String,
    pub institution: String,
    pub bucket: Option<ResultBucket>,
}
