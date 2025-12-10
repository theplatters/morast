use std::fmt::Display;

use macroquad::math::I16Vec2;

#[derive(Debug)]
pub enum BoardError {
    Index,
    TileOccupied,
    TileEmpty(I16Vec2),
    TileNotFound,
    NoMovementPoints,
    CardNotFound,
}

impl Display for BoardError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BoardError::Index => write!(f, "index"),
            BoardError::TileOccupied => write!(f, "tileoccupied"),
            BoardError::TileEmpty(tile) => write!(f, "tileempty {}", tile),
            BoardError::TileNotFound => write!(f, "tilenotfound"),
            BoardError::NoMovementPoints => write!(f, "nomovementpoints"),
            BoardError::CardNotFound => write!(f, "cardnotfound"),
        }
    }
}
