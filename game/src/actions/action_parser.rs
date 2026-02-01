use std::{error::Error, fmt::Display};

use janet_bindings::{error::JanetError, types::janetenum::JanetEnum};

use crate::actions::{GameAction, spell_speed::SpellSpeed};

pub struct ActionParser;

#[derive(Debug, Clone)]
pub enum ParseError {
    JanetError(JanetError),
    NotFound(String),
    Cast(String),
    EngineError,
}

impl From<JanetError> for ParseError {
    fn from(value: JanetError) -> Self {
        ParseError::JanetError(value)
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
impl Error for ParseError {}

impl ActionParser {
    pub fn parse_action(item: &JanetEnum) -> Result<GameAction, ParseError> {
        let action_table = item.as_table().unwrap();
        let condition = super::Condition::new(
            action_table
                .get_function("condition")
                .ok_or(ParseError::NotFound("Could not find condition".into()))?,
        );
        let speed = action_table
            .get("speed")
            .map(|speed| speed.try_into())
            .transpose()?
            .unwrap_or_default();

        let action = action_table
            .get_function("action")
            .ok_or(ParseError::NotFound("Could not find action".into()))?
            .into();

        Ok(GameAction::new(condition, speed, action))
    }
}
