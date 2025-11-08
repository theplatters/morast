use std::ffi::NulError;

#[derive(Debug)]
pub enum EngineError {
    Eval,
    Signal(String),
    Cast(String),
    Type(String),
    File(String),
    String(NulError),
}
