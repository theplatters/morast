use crate::engine::{
    error::EngineError,
    janet_handler::{
        bindings::{janet_unwrap_tuple, Janet},
        types::janetenum::JanetEnum,
    },
};

#[derive(Debug)]
pub struct Tuple {
    raw: *const Janet,
}

impl Tuple {
    pub fn get(&self, pos: isize) -> Result<JanetEnum, EngineError> {
        unsafe { JanetEnum::from(*self.raw.offset(pos)) }
    }

    pub(crate) fn new(item: Janet) -> Self {
        unsafe {
            Self {
                raw: janet_unwrap_tuple(item),
            }
        }
    }
}
