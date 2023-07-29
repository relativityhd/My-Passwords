use serde::{Deserialize, Serialize};
use specta::Type;
use std::fmt::Display;
use std::str::FromStr;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum InvalidModeError {
    #[error("Invalid mode: {0:?}")]
    InvalidMode(String),
}

#[derive(Serialize, Deserialize, Debug, Type)]
pub enum Mode {
    Secure,
    SuperSecure,
    LegacySecure,
    SSO,
}

impl FromStr for Mode {
    type Err = InvalidModeError;

    fn from_str(s: &str) -> Result<Self, InvalidModeError> {
        match s {
            "secure" => Ok(Mode::Secure),
            "super_secure" => Ok(Mode::SuperSecure),
            "legacy_secure" => Ok(Mode::LegacySecure),
            "sso" => Ok(Mode::SSO),
            _ => Err(InvalidModeError::InvalidMode(s.to_string())),
        }
    }
}

impl Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Mode::Secure => "secure",
            Mode::SuperSecure => "super_secure",
            Mode::LegacySecure => "legacy_secure",
            Mode::SSO => "sso",
        };
        write!(f, "{}", s)
    }
}
