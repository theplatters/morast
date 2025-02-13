use crate::game::{card::card_id::CardID, player::PlayerID};

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
