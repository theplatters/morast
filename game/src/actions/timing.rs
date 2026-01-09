use std::{cmp::Ordering, ops::SubAssign};

use bevy::ecs::component::Component;
use janet_bindings::types::{janetenum::JanetEnum, tuple::Tuple};

use crate::{error::GameError, phases::Phase};

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ActionTiming {
    #[default]
    Immediate, // Goes on stack immediately
    Delayed {
        phase: Phase,
        turns: u32,
    }, // End of current turn
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
        }
    }
}

impl Ord for ActionTiming {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap_or_else(|| {
            // Handle the incomparable cases by defining an arbitrary but consistent ordering
            match (self, other) {
                _ => unreachable!("All other cases should be comparable"),
            }
        })
    }
}

impl TryFrom<JanetEnum> for ActionTiming {
    type Error = GameError;

    fn try_from(value: JanetEnum) -> Result<Self, Self::Error> {
        if !(value.is_tuple() || value.is_string()) {
            return Err(GameError::Cast("Timing is not a string or a tuple".into()));
        }

        if value.is_string() {
            value.into_string().unwrap().try_into()
        } else {
            value.into_tuple().unwrap().try_into()
        }
    }
}

impl TryFrom<Tuple> for ActionTiming {
    type Error = GameError;
    fn try_from(value: Tuple) -> Result<Self, Self::Error> {
        let JanetEnum::String(timing) = value.get(0).map_err(GameError::EngineError)? else {
            return Err(GameError::Cast("Timing is not a string".into()));
        };

        match timing.as_str() {
            "now" => Ok(ActionTiming::Immediate),
            "delayed" => {
                let phase = value.get(1).map_err(GameError::EngineError)?.try_into()?;

                let JanetEnum::UInt(turns_ahead) = value.get(2).map_err(GameError::EngineError)?
                else {
                    return Err(GameError::Cast("Timing is not a string".into()));
                };

                Ok(ActionTiming::Delayed {
                    phase,
                    turns: turns_ahead as u32,
                })
            }
            "trigger" => todo!(),
            _ => Err(GameError::Cast("Timing string not supported".into())),
        }
    }
}

impl TryFrom<String> for ActionTiming {
    type Error = GameError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "now" => Ok(ActionTiming::Immediate),
            _ => Err(GameError::Cast("Timing string not supported".into())),
        }
    }
}
