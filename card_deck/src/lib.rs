use::rand::Rng;

pub enum Suit {
    Heart,
    Diamond,
    Spade,
    Club
}

impl Suit {
    pub fn random() -> Suit {

    }

    pub fn translate(idx: u8) -> Suit {
        match idx {
            1 => Suit::Heart,
            2 => Suit::Diamond,
            3 => Suit::Spade,
            4 => Suit::Club,
            _ => unreachable!(),
        }
    }
}

pub enum Rank {
    Ace,
    King,
    Queen,
    Jack,
    Number(u8)
}

impl Rank {
    pub fn Random() -> Rank {

    }

    pub fn translate(idx: u8) -> Rank {
        match idx {
            1 => Rank::Ace,
            2..=10 => Rank::Number(idx),
            11 => Rank::Jack,
            12 => Rank::Queen,
            13 => Rank::King,
            _ => unreachable!(),

        }
    }
}

pub struct Card {
    pub suit: Suit,
    pub rank: Rank
}

pub fn winner_card(card: Card) -> bool {
    
    match card {
        Card{ suit, rank } => if suit == 
    }
}