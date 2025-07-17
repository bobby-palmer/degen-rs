use std::str::FromStr;

use thiserror::Error;

#[derive(PartialEq, Eq, Debug)]
pub enum Rank {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace
}

#[derive(Error, Debug)]
#[error("{0} is not a valid card rank")]
pub struct RankParseError(String);

impl FromStr for Rank {
    type Err = RankParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rank = match s {
            "2" => Self::Two,
            "3" => Self::Three,
            "4" => Self::Four,
            "5" => Self::Five,
            "6" => Self::Six,
            "7" => Self::Seven,
            "8" => Self::Eight,
            "9" => Self::Nine,
            "10" => Self::Ten,
            "J" => Self::Jack,
            "Q" => Self::Queen,
            "K" => Self::King,
            "A" => Self::Ace,
            other => Err(RankParseError(other.to_string()))?
        };

        Ok(rank)
    }
}
