use std::{
    ffi::NulError,
    fmt::{Debug, Display},
};

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
    OutOfBounds,
}

impl Display for EngineError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Eval => write!(f, "Eval"),
            Self::Signal(arg0) => f.debug_tuple("Signal").field(arg0).finish(),
            Self::Cast(arg0) => f.debug_tuple("Cast").field(arg0).finish(),
            Self::Type(arg0) => f.debug_tuple("Type").field(arg0).finish(),
            Self::File(arg0) => f.debug_tuple("File").field(arg0).finish(),
            Self::String(arg0) => f.debug_tuple("String").field(arg0).finish(),
            Self::Load(arg0) => f.debug_tuple("Load").field(arg0).finish(),
            Self::OutOfBounds => write!(f, "OutOfBounds"),
        }
    }
}
