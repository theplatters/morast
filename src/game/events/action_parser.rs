use crate::{
    engine::{
        error::EngineError,
        janet_handler::types::{janetenum::JanetEnum, table::Table, tuple::Tuple},
    },
    game::{
        error::Error,
        events::{
            action::{Action, ActionTiming},
            action_effect::{ActionEffect, TargetingType},
        },
    },
};

// Separate action parser for better organization
pub struct ActionParser;

impl ActionParser {
    pub fn parse(action: &JanetEnum) -> Result<Option<Action>, Error> {
        let JanetEnum::Table(elements) = action else {
            return Err(Error::Cast("Action value is not a table".into()));
        };

        let Some(action_type_table) = elements.get_table("action") else {
            return Err(Error::Cast("Action type is not a table".into()));
        };

        let Some(timing_tuple) = elements.get_tuple("timing") else {
            return Err(Error::Cast("Timing is not a tuple".into()));
        };

        let action_type = Self::parse_action(&action_type_table)?;
        let timing = Self::parse_timing(&timing_tuple)?;

        todo!()
    }

    pub fn parse_action(action: &Table) -> Result<ActionEffect, Error> {
        todo!()
    }

    fn parse_custom_action() -> Result<Action, Error> {
        todo!()
    }

    fn parse_targeting_type(el: JanetEnum) -> Result<TargetingType, Error> {
        match el {
            JanetEnum::String(s) => Self::parse_targeting_string(s),
            JanetEnum::Tuple(tup) => Self::parse_targeting_tuple(tup),
            _ => Err(Error::EngineError(EngineError::Type(
                "not the correct type found".into(),
            ))),
        }
    }

    fn parse_timing(timing_tup: &Tuple) -> Result<ActionTiming, Error> {
        let JanetEnum::String(timing) = timing_tup.get(0).map_err(Error::EngineError)? else {
            return Err(Error::Cast("Timing is not a string".into()));
        };

        match timing.as_str() {
            "now" => Ok(ActionTiming::Immediate),
            "delayed" => {
                let JanetEnum::String(phase) = timing_tup.get(1).map_err(Error::EngineError)?
                else {
                    return Err(Error::Cast("Timing is not a string".into()));
                };

                let JanetEnum::UInt(turns_ahead) = timing_tup.get(2).map_err(Error::EngineError)?
                else {
                    return Err(Error::Cast("Timing is not a string".into()));
                };

                Ok(ActionTiming::Delayed {
                    phase: crate::game::phases::Phase::Start,
                    turns: turns_ahead as u32,
                })
            }
            "trigger" => todo!(),
            _ => Err(Error::Cast("Timing string not supported".into())),
        }
    }

    fn parse_targeting_string(s: String) -> Result<TargetingType, Error> {
        todo!()
    }

    fn parse_targeting_tuple(tup: Tuple) -> Result<TargetingType, Error> {
        todo!()
    }
}
