use macroquad::math::I16Vec2;

use crate::game::{
    actions::action::Action,
    board::effect::Effect,
    card::{card_id::CardID, card_registry::CardRegistry},
    error::Error,
    events::event::Event,
    game_context::GameContext,
    janet_action::JanetAction,
    player::PlayerID,
};

// Core action trait

pub trait GameAction {
    fn execute(
        &self,
        context: &mut GameContext,
        card_registry: &CardRegistry,
    ) -> Result<Option<Event>, Error>;
    fn can_execute(&self, context: &GameContext) -> Result<(), Error>;
}

#[derive(Debug, Clone)]
pub enum ActionEffect {
    // Basic game actions
    PlaceCreature {
        card_index: usize,
        position: I16Vec2,
    },
    CastSpell {
        card_index: usize,
    },
    PlaceTrap {
        card_index: usize,
        position: I16Vec2,
    },

    MoveCreature {
        from: I16Vec2,
        to: I16Vec2,
    },
    EndTurn,

    // Atomic game effects (what cards actually do)
    DealDamage {
        target: Vec<I16Vec2>,
        amount: u16,
    },
    HealCreature {
        target: Vec<I16Vec2>,
        amount: u16,
    },
    DrawCards {
        player_id: PlayerID,
        count: u16,
    },
    AddGold {
        player_id: PlayerID,
        amount: u16,
    },
    ApplyEffect {
        effect: Effect,
        targets: Vec<I16Vec2>,
    },
    SummonCreature {
        creature_id: CardID,
        position: I16Vec2,
        owner: PlayerID,
    },
    DestroyCreature {
        targets: Vec<I16Vec2>,
    },

    WithTargets {
        action: Box<Action>,
        targets: Vec<I16Vec2>,
    },

    // Composite actions
    Sequence(Vec<ActionEffect>),

    Conditional {
        condition: Condition,
        then_action: Box<ActionEffect>,
        else_action: Option<Box<ActionEffect>>,
    },

    // Targeting actions
    Custom {
        action: Box<JanetAction>,
        target: Vec<I16Vec2>,
    },
}

#[derive(Debug, Clone)]
pub enum Condition {
    TileOccupied(I16Vec2),
    CreatureHasHealth { target: I16Vec2, min_health: u16 },
    PlayerHasGold { player_id: PlayerID, min_gold: i64 },
    // ... more conditions
}

#[derive(Debug, Clone)]
pub enum CreatureFilter {
    OwnedBy(PlayerID),
    WithinRange { center: I16Vec2, radius: u8 },
    HasTag(String),
    // ... more filters
}
