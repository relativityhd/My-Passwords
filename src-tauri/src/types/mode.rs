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
    Sso,
}

impl FromStr for Mode {
    type Err = InvalidModeError;

    fn from_str(s: &str) -> Result<Self, InvalidModeError> {
        match s {
            "Secure" => Ok(Mode::Secure),
            "SuperSecure" => Ok(Mode::SuperSecure),
            "LegacySecure" => Ok(Mode::LegacySecure),
            "Sso" => Ok(Mode::Sso),
            _ => Err(InvalidModeError::InvalidMode(s.to_string())),
        }
    }
}

impl Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Mode::Secure => "Secure",
            Mode::SuperSecure => "SuperSecure",
            Mode::LegacySecure => "LegacySecure",
            Mode::Sso => "Sso",
        };
        write!(f, "{}", s)
    }
}
