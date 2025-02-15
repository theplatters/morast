use super::board::place_error::PlaceError;

#[derive(Debug)]
pub enum Error {
    PlayerNotFound,
    PlaceError(PlaceError),
}
