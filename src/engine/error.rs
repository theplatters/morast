use std::ffi::NulError;

#[derive(Debug)]
pub enum EngineError {
    EvalError,
    SignalError(String),
    CastError(String),
    TypeError(String),
    FileError(String),
    StringError(NulError),
}
