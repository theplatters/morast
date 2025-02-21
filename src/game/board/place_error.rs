#[derive(Debug)]
pub enum BoardError {
    Index,
    TileOccupied,
    TileEmpty,
    InvalidMove,
}
