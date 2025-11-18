use crate::engine::error::EngineError;

use super::board::place_error::BoardError;

#[derive(Debug)]
pub enum Error {
    PlayerNotFound,
    PlaceError(BoardError),
    NotCorrectPlayer,
    InvalidMove,
    TileEmpty,
    CardNotFound,
    NotFound(String),
    Cast(String),
    MacroquadError(macroquad::Error),
    InsufficientGold,
    EngineError(EngineError),
    NotPlaceable,
    InvalidHandPosition(usize),
    Incomplete(&'static str),
    InvalidCardType,
}
