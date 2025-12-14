use std::{cmp::Ordering, ops::SubAssign};

use crate::{
    engine::janet_handler::types::{janetenum::JanetEnum, tuple::Tuple},
    game::{error::Error, events::event::GameEvent, phases::Phase},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ActionTiming {
    #[default]
    Immediate, // Goes on stack immediately
    Delayed {
        phase: Phase,
        turns: u32,
    }, // End of current turn
    AtTrigger {
        trigger: GameEvent,
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

impl TryFrom<JanetEnum> for ActionTiming {
    type Error = Error;

    fn try_from(value: JanetEnum) -> Result<Self, Self::Error> {
        if !(value.is_tuple() || value.is_string()) {
            return Err(Error::Cast("Timing is not a string or a tuple".into()));
        }

        if value.is_string() {
            value.into_string().unwrap().try_into()
        } else {
            value.into_tuple().unwrap().try_into()
        }
    }
}

impl TryFrom<Tuple> for ActionTiming {
    type Error = Error;
    fn try_from(value: Tuple) -> Result<Self, Self::Error> {
        let JanetEnum::String(timing) = value.get(0).map_err(Error::EngineError)? else {
            return Err(Error::Cast("Timing is not a string".into()));
        };

        match timing.as_str() {
            "now" => Ok(ActionTiming::Immediate),
            "delayed" => {
                let phase = value.get(1).map_err(Error::EngineError)?.try_into()?;

                let JanetEnum::UInt(turns_ahead) = value.get(2).map_err(Error::EngineError)? else {
                    return Err(Error::Cast("Timing is not a string".into()));
                };

                Ok(ActionTiming::Delayed {
                    phase,
                    turns: turns_ahead as u32,
                })
            }
            "trigger" => todo!(),
            _ => Err(Error::Cast("Timing string not supported".into())),
        }
    }
}

impl TryFrom<String> for ActionTiming {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "now" => Ok(ActionTiming::Immediate),
            _ => Err(Error::Cast("Timing string not supported".into())),
        }
    }
}
