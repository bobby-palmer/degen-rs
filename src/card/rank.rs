use std::str::FromStr;

use thiserror::Error;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
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

    /// Parses a card rank like the following:
    /// ```
    /// use degen::card::rank::Rank;
    /// assert_eq!("A".parse::<Rank>().unwrap(), Rank::Ace);
    /// assert_eq!("K".parse::<Rank>().unwrap(), Rank::King);
    /// assert_eq!("Q".parse::<Rank>().unwrap(), Rank::Queen);
    /// assert_eq!("J".parse::<Rank>().unwrap(), Rank::Jack);
    /// assert_eq!("10".parse::<Rank>().unwrap(), Rank::Ten);
    /// assert_eq!("9".parse::<Rank>().unwrap(), Rank::Nine);
    /// assert_eq!("8".parse::<Rank>().unwrap(), Rank::Eight);
    /// assert_eq!("7".parse::<Rank>().unwrap(), Rank::Seven);
    /// assert_eq!("6".parse::<Rank>().unwrap(), Rank::Six);
    /// assert_eq!("5".parse::<Rank>().unwrap(), Rank::Five);
    /// assert_eq!("4".parse::<Rank>().unwrap(), Rank::Four);
    /// assert_eq!("3".parse::<Rank>().unwrap(), Rank::Three);
    /// assert_eq!("2".parse::<Rank>().unwrap(), Rank::Two);
    /// ```
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
