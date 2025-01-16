use std::collections::HashMap;

use super::{board::Board, deck::Deck, hand::Hand};

pub struct Context<'a> {
    decks: HashMap<u16, &'a Deck>,
    hands: HashMap<u16, &'a Hand>,
    board: &'a Board,
}

impl crate::engine::janet_handler::types::janetenum::ToVoidPointer for Context<'_> {}
