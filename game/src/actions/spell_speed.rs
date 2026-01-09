use bevy::ecs::component::Component;
use janet_bindings::types::janetenum::JanetEnum;

use crate::error::GameError;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum SpellSpeed {
    #[default]
    Slow = 1, // Can only be cast during main phase, when stack is empty
    Fast = 2,    // Can be cast anytime you have priority
    Instant = 3, // Can be cast anytime, even during opponent's turn
}

impl TryFrom<JanetEnum> for SpellSpeed {
    type Error = GameError;

    fn try_from(value: JanetEnum) -> Result<Self, Self::Error> {
        match value {
            JanetEnum::Int(0) => Ok(SpellSpeed::Slow),
            JanetEnum::Int(1) => Ok(SpellSpeed::Fast),
            JanetEnum::Int(2) => Ok(SpellSpeed::Instant),
            JanetEnum::Int(num) => Err(GameError::Cast(format!(
                "Invalid SpellSpeed number: {}",
                num
            ))),

            JanetEnum::UInt(0) => Ok(SpellSpeed::Slow),
            JanetEnum::UInt(1) => Ok(SpellSpeed::Fast),
            JanetEnum::UInt(2) => Ok(SpellSpeed::Instant),
            JanetEnum::UInt(num) => Err(GameError::Cast(format!(
                "Invalid SpellSpeed number: {}",
                num
            ))),

            JanetEnum::String(s) => match s.as_str() {
                "slow " => Ok(SpellSpeed::Slow),
                "fast" => Ok(SpellSpeed::Fast),
                "instant" => Ok(SpellSpeed::Instant),
                _ => Err(GameError::Cast(format!("Invalid SpellSpeed string: {}", s))),
            },
            _ => Err(GameError::Cast(format!("Invalid SpellSpeed type "))),
        }
    }
}
