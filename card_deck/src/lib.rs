use rand::Rng;

#[derive(Debug, PartialEq, Eq)]
pub enum Suit {
    Heart,
    Diamond,
    Spade,
    Club,
}

impl Suit {
    pub fn random() -> Suit {
        Self::translate(rand::thread_rng().gen_range(1..=4))
    }

    pub fn translate(value: u8) -> Suit {
        match value {
            1 => Suit::Heart,
            2 => Suit::Diamond,
            3 => Suit::Spade,
            4 => Suit::Club,
            _ => panic!("Invalid suit value"),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Rank {
    Ace,
    Number(u8),
    Jack,
    Queen,
    King,
}

impl Rank {
    pub fn random() -> Rank {
        Self::translate(rand::thread_rng().gen_range(1..=13))
    }

    pub fn translate(value: u8) -> Rank {
        match value {
            1 => Rank::Ace,
            2..=10 => Rank::Number(value),
            11 => Rank::Jack,
            12 => Rank::Queen,
            13 => Rank::King,
            _ => panic!("Invalid rank value"),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

pub fn winner_card(card: &Card) -> bool {
    card.suit == Suit::Spade && card.rank == Rank::Ace
}