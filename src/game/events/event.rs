use crate::game::{card::card_id::CardID, player::PlayerID};

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum Event {
    SpellPlayed {
        owner: PlayerID,
        card_id: CardID,
    },
    CreaturePlayed {
        card_id: CardID,
        owner: PlayerID,
    },
    TurnEnd,
    TrapPlaced {
        owner: PlayerID,
        card_id: CardID,
    },
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
