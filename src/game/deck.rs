use crate::engine;

use super::card::Card;

pub struct Deck {
    cards: Vec<Card>,
}

impl engine::janet_handler::types::janetenum::ToVoidPointer for Deck {}
