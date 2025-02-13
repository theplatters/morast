use std::io::Empty;

use super::{card_on_board::CardOnBoard, effect::Effect};

#[derive(Debug, PartialEq, Eq)]
pub enum TileState {
    Empty,
    Card(CardOnBoard),
}

#[derive(Debug)]
pub struct Tile {
    ontile: TileState,
    effects: Vec<Effect>,
}

impl Tile {
    pub fn new() -> Self {
        Self {
            ontile: TileState::Empty,
            effects: Vec::new(),
        }
    }

    pub fn place(&mut self, card: CardOnBoard) {
        self.ontile = TileState::Card(card);
    }

    pub fn is_occupied(&self) -> bool {
        self.ontile != TileState::Empty
    }
}
