use serde::{Deserialize, Serialize};
use specta::Type;
use std::convert::Infallible;
use std::fmt::Display;
pub use std::str::FromStr;

#[derive(Serialize, Deserialize, Debug, Type)]
pub enum Industry {
    Tech,     // -> @
    Games,    // -> !
    Social,   // -> #
    Finance,  // -> $
    Shopping, // -> *
    Science,  // -> ?
    Other,    // -> &
}

impl FromStr for Industry {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "tech" => Ok(Industry::Tech),
            "games" => Ok(Industry::Games),
            "social" => Ok(Industry::Social),
            "finance" => Ok(Industry::Finance),
            "shopping" => Ok(Industry::Shopping),
            "science" => Ok(Industry::Science),
            _ => Ok(Industry::Other),
        }
    }
}

impl Display for Industry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Industry::Tech => "tech",
            Industry::Games => "games",
            Industry::Social => "social",
            Industry::Finance => "finance",
            Industry::Shopping => "shopping",
            Industry::Science => "science",
            Industry::Other => "other",
        };
        write!(f, "{}", s)
    }
}

impl Industry {
    pub fn parse(&self) -> &'static str {
        match self {
            Industry::Tech => "@",
            Industry::Games => "!",
            Industry::Social => "#",
            Industry::Finance => "$",
            Industry::Shopping => "*",
            Industry::Science => "?",
            Industry::Other => "&",
        }
    }
}
