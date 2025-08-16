pub enum Suit {
    Hearts,
    Diamonds,
    Spades,
    Clubs,
}

impl Suit {
    const COUNT: u32 = 4;
}

impl From<Suit> for u32 {
    fn from(value: Suit) -> Self {
        match value {
            Suit::Hearts => 0,
            Suit::Diamonds=> 1,
            Suit::Spades=> 2,
            Suit::Clubs=> 3,
        }
    }
}

pub enum Value {
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
    Ace,
}

impl Value {
    const COUNT: u32 = 13;
}

impl From<Value> for u32 {
    fn from(value: Value) -> Self {
        match value {
            Value::Two => 0,
            Value::Three => 1,
            Value::Four => 2,
            Value::Five => 3,
            Value::Six => 4,
            Value::Seven => 5,
            Value::Eight => 6,
            Value::Nine => 7,
            Value::Ten => 8,
            Value::Jack => 9,
            Value::Queen => 10,
            Value::King => 11,
            Value::Ace => 12,
        }
    }
}

pub struct Card {
    value: Value,
    suit: Suit,
}

impl From<Card> for u32 {
    fn from(value: Card) -> Self {
        let suit: u32 = value.suit.into();
        let value: u32 = value.value.into();
        suit * Value::COUNT + value
    }
}

pub struct CardSet(u64);
