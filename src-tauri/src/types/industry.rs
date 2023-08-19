use entities::sea_orm_active_enums::Industry as IndustryEnum;
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

impl Into<IndustryEnum> for Industry {
    fn into(self) -> IndustryEnum {
        match self {
            Industry::Tech => IndustryEnum::Tech,
            Industry::Games => IndustryEnum::Games,
            Industry::Social => IndustryEnum::Social,
            Industry::Finance => IndustryEnum::Finance,
            Industry::Shopping => IndustryEnum::Shopping,
            Industry::Science => IndustryEnum::Science,
            Industry::Other => IndustryEnum::Other,
        }
    }
}

impl From<IndustryEnum> for Industry {
    fn from(value: IndustryEnum) -> Self {
        match value {
            IndustryEnum::Tech => Industry::Tech,
            IndustryEnum::Games => Industry::Games,
            IndustryEnum::Social => Industry::Social,
            IndustryEnum::Finance => Industry::Finance,
            IndustryEnum::Shopping => Industry::Shopping,
            IndustryEnum::Science => Industry::Science,
            IndustryEnum::Other => Industry::Other,
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
