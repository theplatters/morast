use crate::game::{card::card_id::CardID, player::PlayerID};
use std::hash::{Hash, Hasher};
#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Copy, Clone)]
pub struct CardOnBoard {
    pub id: i32,
    pub card_id: CardID,
    pub player_id: PlayerID,
}

impl CardOnBoard {
    pub fn new(id: i32, card_id: CardID, player_id: PlayerID) -> Self {
        Self {
            id,
            card_id,
            player_id,
        }
    }
}

impl Hash for CardOnBoard {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}
