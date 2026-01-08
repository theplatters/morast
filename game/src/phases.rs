use crate::{engine::janet_handler::types::janetenum::JanetEnum, game::error::GameError};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Phase {
    Start, // Beginning of a turn
    Main,  // During the turn
    End,   // End of a turn
}

impl From<i32> for Phase {
    fn from(value: i32) -> Self {
        match value {
            0 => Phase::Start,
            1 => Phase::Main,
            2 => Phase::End,
            _ => Phase::End,
        }
    }
}

impl TryFrom<JanetEnum> for Phase {
    type Error = GameError;

    fn try_from(value: JanetEnum) -> Result<Self, Self::Error> {
        if !value.is_int() {
            return Err(GameError::Cast(
                "Phase is not given in the correct format (int)".into(),
            ));
        };

        Ok(value.as_int().unwrap().into())
    }
}
