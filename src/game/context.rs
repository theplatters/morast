use std::collections::HashMap;

use super::{board::Board, deck::Deck, hand::Hand};

pub struct Context<'a> {
    decks: HashMap<u16, Deck>,
    hands: HashMap<u16, Hand>,
    board: &'a Board,
}
