use std::ffi::NulError;

#[derive(Debug)]
pub enum JanetError {
    OutOfBounds,
    Cast(String),
    Signal(String),
    Type(String),
    String(NulError),
    File(String),
}
