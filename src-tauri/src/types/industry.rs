use serde_repr::*;
use specta::Type;
use std::convert::Infallible;
use std::fmt::Display;
pub use std::str::FromStr;

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug, Type)]
#[repr(u8)]
pub enum Industry {
    Tech,     // -> @
    Games,    // -> !
    Social,   // -> #
    Finance,  // -> $
    Shopping, // -> *
    Science,  // -> ?
    Other,    // -> &
}

impl From<i32> for Industry {
    fn from(value: i32) -> Self {
        match value {
            1 => Industry::Tech,
            2 => Industry::Games,
            3 => Industry::Social,
            4 => Industry::Finance,
            5 => Industry::Shopping,
            6 => Industry::Science,
            _ => Industry::Other,
        }
    }
}

impl Into<i32> for Industry {
    fn into(self) -> i32 {
        match self {
            Industry::Tech => 1,
            Industry::Games => 2,
            Industry::Social => 3,
            Industry::Finance => 4,
            Industry::Shopping => 5,
            Industry::Science => 6,
            Industry::Other => 0,
        }
    }
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
