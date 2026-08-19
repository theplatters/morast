use std::{cmp::Ordering, ops::SubAssign};

use bevy::ecs::component::Component;
use serde::{Deserialize, Serialize};

use crate::phases::Phase;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
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
            unreachable!("All other cases should be comparable")
        })
    }
}
