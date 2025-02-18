use std::io::Empty;

use super::{card_on_board::CardOnBoard, effect::Effect};

#[derive(Debug, PartialEq, Eq)]
pub enum TileState {
    Empty,
    Card(CardOnBoard),
}

#[derive(Debug)]
pub struct Tile {
    pub ontile: TileState,
    effects: Vec<Effect>,
    attack_on_tile: [u64; 2],
}

impl Tile {
    pub fn new() -> Self {
        Self {
            ontile: TileState::Empty,
            effects: Vec::new(),
            attack_on_tile: [0, 0],
        }
    }

    pub fn place(&mut self, card: CardOnBoard) {
        self.ontile = TileState::Card(card);
    }

    pub fn is_occupied(&self) -> bool {
        self.ontile != TileState::Empty
    }
}
