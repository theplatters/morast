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
}
