use crate::game::{card::card_id::CardID, player::PlayerID};

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Copy, Clone)]
pub struct CardOnBoard {
    pub id: i64,
    pub card_id: CardID,
    pub player_id: PlayerID,
}

impl CardOnBoard {
    pub fn new(id: i64, card_id: CardID, player_id: PlayerID) -> Self {
        Self {
            id,
            card_id,
            player_id,
        }
    }
}
