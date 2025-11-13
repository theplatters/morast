use crate::game::{
    card::{card_id::CardID, in_play_id::InPlayID},
    player::PlayerID,
};

use std::hash::{Hash, Hasher};
#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Copy, Clone)]
pub struct CardOnBoard {
    pub card_id: CardID,
    pub player_id: PlayerID,
    movement_points: u16,
}

impl CardOnBoard {
    pub fn new(card_id: CardID, player_id: PlayerID, movement_points: u16) -> Self {
        Self {
            card_id,
            player_id,
            movement_points,
        }
    }
}
