use std::ffi::NulError;

use macroquad::Error;

#[derive(Debug)]
pub enum EngineError {
    Eval,
    Signal(String),
    Cast(String),
    Type(String),
    File(String),
    String(NulError),
    Load(Error),
}
