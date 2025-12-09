use macroquad::math::I16Vec2;

use crate::game::{card::card_id::CardID, player::PlayerID};

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum Event {
    SpellPlayed {
        owner: PlayerID,
        card_id: CardID,
    },
    CreaturePlayed {
        owner: PlayerID,
        card_id: CardID,
        position: I16Vec2,
    },

    TrapPlaced {
        owner: PlayerID,
        card_id: CardID,
        position: I16Vec2,
    },
    TurnEnd,
    EffectAdded {
        effect: crate::game::board::effect::Effect,
    },
    GoldAdded {
        player_id: PlayerID,
        amount: u16,
    },
    CardsDrawn {
        player_id: PlayerID,
        count: u16,
    },
}
