use macroquad::math::I16Vec2;

use crate::game::{
    board::effect::Effect,
    card::card_id::CardID,
    events::{
        action::{ActionTiming, SpellSpeed},
        action_effect::{ActionEffect, TargetingType},
    },
    player::PlayerID,
};

pub struct ActionPrototype {
    pub action: ActionEffectPrototype,
    pub timing: ActionTiming,
    pub speed: SpellSpeed,
    pub player: PlayerID,
    pub can_be_countered: bool,
}

#[derive(Debug, Clone)]
pub enum ActionEffectPrototype {
    // Basic game actions
    PlaceCreature {
        card_index: usize,
        position: I16Vec2,
        player_id: PlayerID,
    },
    CastSpell {
        card_index: usize,
        player_id: PlayerID,
    },
    PlaceTrap {
        card_index: usize,
        position: I16Vec2,
        player_id: PlayerID,
    },

    MoveCreature {
        from: I16Vec2,
        to: I16Vec2,
        player_id: PlayerID,
    },
    EndTurn,

    // Atomic game effects (what cards actually do)
    DealDamage {
        target: TargetingType,
        amount: u16,
    },
    HealCreature {
        target: TargetingType,
        amount: u16,
    },
    DrawCards {
        player_id: PlayerID,
        count: u16,
    },
    AddGold {
        player_id: PlayerID,
        amount: i64,
    },
    ApplyEffect {
        effect: Effect,
        tiles: TargetingType,
        duration: u32,
    },
    SummonCreature {
        creature_id: CardID,
    },

    DestroyCreature {
        target: TargetingType,
    },

    // Composite actions
    Sequence(Vec<ActionEffect>),

    // Targeting actions
    Custom {
        action: Box<JanetAction>,
        target: TargetingType,
    },
}
