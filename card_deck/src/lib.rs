#[derive(Debug, PartialEq)]
pub enum Suit {
    Heart, 
    Diamond, 
    Spade,
    Club
}

#[derive(Debug, PartialEq)]
pub enum Rank {
    Ace,
    Number(u8),
    Jack,
    Queen,
    King,
}

impl Suit {
    pub fn random() -> Suit {
        Suit::translate(rand::random_range(1..=4))
    }
    
    pub fn translate(value: u8) -> Suit {
        match value {
            1_u8 => Suit::Heart,
            2_u8 => Suit::Diamond,
            3_u8 => Suit::Spade,
            4_u8 => Suit::Club,
            _ => panic!("Invalid value for Suit"),
        }
    }
}

impl Rank {
    pub fn random() -> Rank {
        match rand::random_range(1..=13) {
            1 => Rank::Ace,
            2..=10 => Rank::Number(rand::random_range(2..=10)),
            11 => Rank::Jack,
            12 => Rank::Queen,
            13 => Rank::King,
            _ => unreachable!(),
        }
    }

    pub fn translate(value: u8) -> Rank {
        match value {
            1 => Rank::Ace,
            2..=10 => Rank::Number(value),
            11 => Rank::Jack,
            12 => Rank::Queen,
            13 => Rank::King,
            _ => panic!("Invalid value for Rank"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

pub fn winner_card(card: &Card) -> bool {
    if let (&Suit::Spade, &Rank::Ace) = (&card.suit, &card.rank) {
        true
    } else {
        false
    }
}