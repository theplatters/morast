use bevy::ecs::component::Component;
use serde::{Deserialize, Serialize};

#[derive(
    Component, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Serialize, Deserialize,
)]
pub enum SpellSpeed {
    #[default]
    Slow = 1, // Can only be cast during main phase, when stack is empty
    Fast = 2,    // Can be cast anytime you have priority
    Instant = 3, // Can be cast anytime, even during opponent's turn
}
