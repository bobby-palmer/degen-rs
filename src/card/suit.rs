use std::str::FromStr;

use thiserror::Error;

#[derive(PartialEq, Eq, Debug)]
pub enum Suit {
    Diamond,
    Heart,
    Spade,
    Club,
}

#[derive(Error, Debug)]
#[error("{0} is not a valid suit")]
pub struct SuitParseError(String);

impl FromStr for Suit {
    type Err = SuitParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let suit = match s {
            "d" => Self::Diamond,
            "h" => Self::Heart,
            "s" => Self::Spade,
            "c" => Self::Club,
            other => Err(SuitParseError(other.to_string()))?
        };

        Ok(suit)
    }
}
