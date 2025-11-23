use macroquad::math::I16Vec2;

use crate::game::{
    board::effect::Effect,
    card::{card_id::CardID, in_play_id::InPlayID},
    error::Error,
    events::event_scheduler::GameScheduler,
    game_context::GameContext,
    player::PlayerID,
};
//
// Core action trait
pub trait GameAction {
    fn execute(
        &self,
        context: &mut GameContext,
        scheduler: &mut GameScheduler,
    ) -> Result<(), Error>;
    fn can_execute(&self, context: &GameContext) -> Result<(), Error>;
}

#[derive(Debug, Clone)]
pub enum Action {
    // Basic game actions
    PlaceCreature {
        card_index: usize,
        position: I16Vec2,
        player_id: PlayerID,
    },

    CastSpell {
        card_index: usize,
        targets: Vec<I16Vec2>,
        player_id: PlayerID,
    },
    MoveCreature {
        from: I16Vec2,
        to: I16Vec2,
        player_id: PlayerID,
    },
    // Atomic game effects (what cards actually do)
    DealDamage {
        target: I16Vec2,
        amount: u16,
        source: InPlayID,
    },
    HealCreature {
        target: I16Vec2,
        amount: u16,
        source: InPlayID,
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
        tiles: Vec<I16Vec2>,
        duration: u32,
    },
    SummonCreature {
        creature_id: CardID,
        position: I16Vec2,
        owner: PlayerID,
    },
    DestroyCreature {
        target: I16Vec2,
    },

    // Composite actions
    Sequence(Vec<Action>),

    Conditional {
        condition: Condition,
        then_action: Box<Action>,
        else_action: Option<Box<Action>>,
    },

    // Targeting actions
    ForEachInArea {
        center: I16Vec2,
        radius: u8,
        action: Box<Action>,
    },
    ForEachCreature {
        filter: CreatureFilter,
        action: Box<Action>,
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

impl GameAction for Action {
    fn execute(
        &self,
        context: &mut GameContext,
        scheduler: &mut GameScheduler,
    ) -> Result<(), Error> {
        match self {
            Action::PlaceCreature {
                card_index,
                position,
                player_id,
            } => todo!(),
            Action::CastSpell {
                card_index,
                targets,
                player_id,
            } => todo!(),
            Action::MoveCreature {
                from,
                to,
                player_id,
            } => todo!(),
            Action::DealDamage {
                target,
                amount,
                source,
            } => todo!(),
            Action::HealCreature {
                target,
                amount,
                source,
            } => todo!(),
            Action::DrawCards { player_id, count } => todo!(),
            Action::AddGold { player_id, amount } => todo!(),
            Action::ApplyEffect {
                effect,
                tiles,
                duration,
            } => todo!(),
            Action::SummonCreature {
                creature_id,
                position,
                owner,
            } => todo!(),
            Action::DestroyCreature { target } => todo!(),
            Action::Sequence(actions) => todo!(),
            Action::Conditional {
                condition,
                then_action,
                else_action,
            } => todo!(),
            Action::ForEachInArea {
                center,
                radius,
                action,
            } => todo!(),
            Action::ForEachCreature { filter, action } => todo!(),
        }
    }

    fn can_execute(&self, context: &GameContext) -> Result<(), Error> {
        match self {
            Action::PlaceCreature {
                card_index,
                position,
                player_id,
            } => todo!(),
            Action::CastSpell {
                card_index,
                targets,
                player_id,
            } => todo!(),
            Action::MoveCreature {
                from,
                to,
                player_id,
            } => todo!(),
            Action::DealDamage {
                target,
                amount,
                source,
            } => todo!(),
            Action::HealCreature {
                target,
                amount,
                source,
            } => todo!(),
            Action::DrawCards { player_id, count } => todo!(),
            Action::AddGold { player_id, amount } => todo!(),
            Action::ApplyEffect {
                effect,
                tiles,
                duration,
            } => todo!(),
            Action::SummonCreature {
                creature_id,
                position,
                owner,
            } => todo!(),
            Action::DestroyCreature { target } => todo!(),
            Action::Sequence(actions) => todo!(),
            Action::Conditional {
                condition,
                then_action,
                else_action,
            } => todo!(),
            Action::ForEachInArea {
                center,
                radius,
                action,
            } => todo!(),
            Action::ForEachCreature { filter, action } => todo!(),
        }
    }
}
