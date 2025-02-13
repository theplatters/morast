use super::{card::card_holder::CardHolder, player::PlayerID};

#[derive(Debug)]
pub struct Hand {
    pub player: PlayerID,
    pub cards: CardHolder,
}

impl Hand {
    pub fn new(player: PlayerID) -> Self {
        Self {
            player,
            cards: CardHolder::new(),
        }
    }
}

impl crate::engine::janet_handler::types::janetenum::ToVoidPointer for Hand {}
