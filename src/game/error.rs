use std::fmt::{write, Display};

use crate::{engine::error::EngineError, game::actions::action_builder::ActionBuilderError};

use super::board::place_error::BoardError;

#[derive(Debug)]
pub enum GameError {
    PlayerNotFound,
    PlaceError(BoardError),
    IncorrectPlayer,
    InvalidMove,
    CardNotFound,
    NotFound(String),
    Cast(String),
    InsufficientGold,
    EngineError(EngineError),
    InvalidHandPosition(usize),
    Incomplete(&'static str),
    InvalidCardType,
    WrongState,
    ActionError(&'static str),
    ActionBuilderError(ActionBuilderError),
    InputCancelled,
    NoInputReceived,
}

impl Display for GameError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GameError::PlayerNotFound => write!(f, "Player not found"),
            GameError::PlaceError(board_error) => write!(f, "Board Error {}", board_error),
            GameError::IncorrectPlayer => write!(f, "Incorrect Player"),
            GameError::InvalidMove => write!(f, "InvalidMove"),
            GameError::CardNotFound => write!(f, "Card not found"),
            GameError::NotFound(s) => write!(f, "Not found: {}", s),
            GameError::Cast(s) => write!(f, "Cast error: {}", s),
            GameError::InsufficientGold => write!(f, "InsufficientGold"),
            GameError::EngineError(engine_error) => write!(f, "EngineError: {}", engine_error),
            GameError::InvalidHandPosition(pos) => write!(f, "Invalud Hand position: {}", pos),
            GameError::Incomplete(s) => write!(f, "Inclomplete: {}", s),
            GameError::InvalidCardType => write!(f, "InvalidCardType"),
            GameError::WrongState => write!(f, "Wrong State"),
            GameError::ActionError(s) => write!(f, "Error when performing action: {}", s),
            GameError::ActionBuilderError(action_builder_error) => {
                write!(f, "Error when building action: {}", action_builder_error)
            }
            GameError::InputCancelled => write!(f, "Input cancelled"),
            GameError::NoInputReceived => write!(f, "No input recieved"),
        }
    }
}

impl std::error::Error for GameError {}
