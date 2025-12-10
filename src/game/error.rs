use std::fmt::{write, Display};

use crate::{engine::error::EngineError, game::actions::action_builder::ActionBuilderError};

use super::board::place_error::BoardError;

#[derive(Debug)]
pub enum Error {
    PlayerNotFound,
    PlaceError(BoardError),
    IncorrectPlayer,
    InvalidMove,
    CardNotFound,
    NotFound(String),
    Cast(String),
    MacroquadError(macroquad::Error),
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

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::PlayerNotFound => write!(f, "Player not found"),
            Error::PlaceError(board_error) => write!(f, "Board Error {}", board_error),
            Error::IncorrectPlayer => write!(f, "Incorrect Player"),
            Error::InvalidMove => write!(f, "InvalidMove"),
            Error::CardNotFound => write!(f, "Card not found"),
            Error::NotFound(s) => write!(f, "Not found: {}", s),
            Error::Cast(s) => write!(f, "Cast error: {}", s),
            Error::MacroquadError(error) => write!(f, "MacroquadError: {}", error),
            Error::InsufficientGold => write!(f, "InsufficientGold"),
            Error::EngineError(engine_error) => write!(f, "EngineError: {}", engine_error),
            Error::InvalidHandPosition(pos) => write!(f, "Invalud Hand position: {}", pos),
            Error::Incomplete(s) => write!(f, "Inclomplete: {}", s),
            Error::InvalidCardType => write!(f, "InvalidCardType"),
            Error::WrongState => write!(f, "Wrong State"),
            Error::ActionError(s) => write!(f, "Error when performing action: {}", s),
            Error::ActionBuilderError(action_builder_error) => {
                write!(f, "Error when building action: {}", action_builder_error)
            }
            Error::InputCancelled => write!(f, "Input cancelled"),
            Error::NoInputReceived => write!(f, "No input recieved"),
        }
    }
}

impl std::error::Error for Error {}
