use super::card::Card;

pub struct Hand {
    cards: Vec<Card>,
}

impl crate::engine::janet_handler::types::janetenum::ToVoidPointer for Hand {}
