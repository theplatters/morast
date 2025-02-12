use super::{card::card_id::CardID, player::PlayerID};

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
pub struct CardOnBoard {
    card_id: CardID,
    player_id: PlayerID,
}

impl CardOnBoard {
    pub fn new(card_id: CardID, player_id: PlayerID) -> Self {
        Self { card_id, player_id }
    }
}

#[derive(Debug)]
pub struct Tile {
    ontile: Vec<CardOnBoard>,
}

impl Tile {
    pub fn new() -> Self {
        Self { ontile: Vec::new() }
    }

    pub fn place(&mut self, card: CardOnBoard) {
        self.ontile.push(card);
    }
}
