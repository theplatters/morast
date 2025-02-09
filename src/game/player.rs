use super::{deck::Deck, hand::Hand};

pub struct Player {
    id: u16,
    money: u64,
    deck: Deck,
    hand: Hand,
    discard_pile: Deck,
}
