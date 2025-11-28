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
        phases::Phase,
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

        let Some(timing_tuple) = elements.get("timing") else {
            return Err(Error::Cast("Timing is not a tuple".into()));
        };

        let timing = Self::parse_timing(&timing_tuple)?;
        print!("Action Timing {:?}", timing);
        let action_type = Self::parse_action_effect(&action_type_table)?;

        todo!("action parsing not fully implemented")
    }

    pub fn parse_action_effect(action: &Table) -> Result<ActionEffect, Error> {
        todo!("action effect parsing not fully implemented")
    }

    fn parse_custom_action() -> Result<Action, Error> {
        todo!()
    }

    fn parse_targeting_type(el: JanetEnum) -> Result<TargetingType, Error> {
        match el {
            JanetEnum::String(s) => Self::parse_targeting_from_string(s),
            JanetEnum::Tuple(tup) => Self::parse_targeting_tuple(tup),
            _ => Err(Error::EngineError(EngineError::Type(
                "not the correct type found".into(),
            ))),
        }
    }

    fn parse_timing(timing_janet: &JanetEnum) -> Result<ActionTiming, Error> {
        if !(timing_janet.is_tuple() || timing_janet.is_string()) {
            return Err(Error::Cast("Timing is not a string or a tuple".into()));
        }

        if timing_janet.is_string() {
            Self::parse_timing_from_string(timing_janet.as_string().unwrap())
        } else {
            Self::parse_timing_from_tuple(timing_janet.as_tuple().unwrap())
        }
    }

    fn parse_timing_from_tuple(timing_tup: &Tuple) -> Result<ActionTiming, Error> {
        let JanetEnum::String(timing) = timing_tup.get(0).map_err(Error::EngineError)? else {
            return Err(Error::Cast("Timing is not a string".into()));
        };

        match timing.as_str() {
            "now" => Ok(ActionTiming::Immediate),
            "delayed" => {
                let phase = Self::parse_phase(&timing_tup.get(1).map_err(Error::EngineError)?)?;

                let JanetEnum::UInt(turns_ahead) = timing_tup.get(2).map_err(Error::EngineError)?
                else {
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

    fn parse_phase(phase_janet: &JanetEnum) -> Result<Phase, Error> {
        if !phase_janet.is_int() {
            return Err(Error::Cast(
                "Phase is not given in the correct format (int)".into(),
            ));
        };

        Ok(phase_janet.as_int().unwrap().into())
    }

    fn parse_timing_from_string(timing_str: &str) -> Result<ActionTiming, Error> {
        match timing_str {
            "now" => Ok(ActionTiming::Immediate),
            _ => Err(Error::Cast("Timing string not supported".into())),
        }
    }

    fn parse_targeting_from_string(s: String) -> Result<TargetingType, Error> {
        todo!()
    }

    fn parse_targeting_tuple(tup: Tuple) -> Result<TargetingType, Error> {
        todo!()
    }
}
