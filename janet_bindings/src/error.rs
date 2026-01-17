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

impl std::fmt::Display for JanetError {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
