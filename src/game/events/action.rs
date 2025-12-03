use std::{cmp::Ordering, ops::SubAssign};

use crate::{
    engine::janet_handler::types::janetenum::JanetEnum,
    game::{
        error::Error,
        events::{
            action_builder::ActionBuilder,
            action_effect::{ActionEffect, GameAction},
            event::Event,
        },
        phases::Phase,
        player::PlayerID,
    },
};

#[derive(Debug, Clone)]
pub struct Action {
    pub action: ActionEffect,
    pub timing: ActionTiming,
    pub speed: SpellSpeed,
    pub priority: u32,
    pub player: PlayerID,
    pub can_be_countered: bool,
}
impl PartialEq for Action {
    fn eq(&self, other: &Self) -> bool {
        self.timing == other.timing && self.speed == other.speed && self.priority == other.priority
    }
}

impl Eq for Action {}

impl PartialOrd for Action {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Action {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (other.timing, other.speed, other.priority).cmp(&(self.timing, self.speed, self.priority))
    }
}

impl GameAction for Action {
    fn execute(
        &self,
        context: &mut crate::game::game_context::GameContext,
        card_registry: &crate::game::card::card_registry::CardRegistry,
    ) -> Result<Option<Event>, crate::game::error::Error> {
        self.action.execute(context, card_registry)
    }

    fn can_execute(
        &self,
        context: &crate::game::game_context::GameContext,
    ) -> Result<(), crate::game::error::Error> {
        self.action.can_execute(context)
    }
}

impl Action {
    pub fn builder() -> ActionBuilder {
        ActionBuilder::new()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum SpellSpeed {
    #[default]
    Slow = 1, // Can only be cast during main phase, when stack is empty
    Fast = 2,    // Can be cast anytime you have priority
    Instant = 3, // Can be cast anytime, even during opponent's turn
}

impl TryFrom<JanetEnum> for SpellSpeed {
    type Error = Error;

    fn try_from(value: JanetEnum) -> Result<Self, Self::Error> {
        match value {
            JanetEnum::Int(0) => Ok(SpellSpeed::Slow),
            JanetEnum::Int(1) => Ok(SpellSpeed::Fast),
            JanetEnum::Int(2) => Ok(SpellSpeed::Instant),
            JanetEnum::Int(num) => Err(Error::Cast(format!("Invalid SpellSpeed number: {}", num))),

            JanetEnum::UInt(0) => Ok(SpellSpeed::Slow),
            JanetEnum::UInt(1) => Ok(SpellSpeed::Fast),
            JanetEnum::UInt(2) => Ok(SpellSpeed::Instant),
            JanetEnum::UInt(num) => Err(Error::Cast(format!("Invalid SpellSpeed number: {}", num))),

            JanetEnum::String(s) => match s.as_str() {
                "slow " => Ok(SpellSpeed::Slow),
                "fast" => Ok(SpellSpeed::Fast),
                "instant" => Ok(SpellSpeed::Instant),
                _ => Err(Error::Cast(format!("Invalid SpellSpeed string: {}", s))),
            },
            _ => Err(Error::Cast(format!("Invalid SpellSpeed type "))),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ActionTiming {
    #[default]
    Immediate, // Goes on stack immediately
    Delayed {
        phase: Phase,
        turns: u32,
    }, // End of current turn
    AtTrigger {
        trigger: Event,
    }, // Start of next turn
}

impl SubAssign<u32> for ActionTiming {
    fn sub_assign(&mut self, rhs: u32) {
        if let ActionTiming::Delayed { turns, .. } = self {
            *turns = turns.saturating_sub(rhs);
        }
        // Immediate and AtTrigger variants are unchanged
    }
}

impl PartialOrd for ActionTiming {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            // Immediate actions always come first
            (ActionTiming::Immediate, ActionTiming::Immediate) => Some(Ordering::Equal),
            (ActionTiming::Immediate, _) => Some(Ordering::Less),
            (_, ActionTiming::Immediate) => Some(Ordering::Greater),

            // Compare delayed actions by turns, then by phase
            (
                ActionTiming::Delayed {
                    phase: p1,
                    turns: t1,
                },
                ActionTiming::Delayed {
                    phase: p2,
                    turns: t2,
                },
            ) => match t1.cmp(t2) {
                Ordering::Equal => p1.partial_cmp(p2),
                other => Some(other),
            },

            // AtTrigger actions cannot be compared with Delayed actions
            // (they depend on external events, not time)
            (ActionTiming::Delayed { .. }, ActionTiming::AtTrigger { .. }) => None,
            (ActionTiming::AtTrigger { .. }, ActionTiming::Delayed { .. }) => None,

            // AtTrigger actions are equal to each other for ordering purposes
            // (actual execution order would depend on trigger resolution)
            (ActionTiming::AtTrigger { .. }, ActionTiming::AtTrigger { .. }) => {
                Some(Ordering::Equal)
            }
        }
    }
}

impl Ord for ActionTiming {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap_or_else(|| {
            // Handle the incomparable cases by defining an arbitrary but consistent ordering
            match (self, other) {
                (ActionTiming::Delayed { .. }, ActionTiming::AtTrigger { .. }) => Ordering::Less,
                (ActionTiming::AtTrigger { .. }, ActionTiming::Delayed { .. }) => Ordering::Greater,
                _ => unreachable!("All other cases should be comparable"),
            }
        })
    }
}
