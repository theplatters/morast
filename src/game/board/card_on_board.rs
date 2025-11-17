use crate::game::{card::card_id::CardID, player::PlayerID};

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Copy, Clone)]
pub struct CreatureOnBoard {
    pub card_id: CardID,
    pub player_id: PlayerID,
    pub movement_points: u16,
}

impl CreatureOnBoard {
    pub fn new(card_id: CardID, player_id: PlayerID, movement_points: u16) -> Self {
        Self {
            card_id,
            player_id,
            movement_points,
        }
    }
}
