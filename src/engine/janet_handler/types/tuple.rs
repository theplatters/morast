use crate::engine::{
    error::EngineError,
    janet_handler::{
        bindings::{janet_tuple_begin, janet_tuple_end, janet_unwrap_tuple, Janet},
        types::janetenum::JanetEnum,
    },
};

#[derive(Debug, Clone)]
pub struct Tuple {
    raw: *const Janet,
}

impl Tuple {
    pub fn get(&self, pos: usize) -> Result<JanetEnum, EngineError> {
        unsafe { JanetEnum::from(*self.raw.add(pos)) }
    }

    pub(crate) fn new(item: Janet) -> Self {
        unsafe {
            Self {
                raw: janet_unwrap_tuple(item),
            }
        }
    }
    // Create a new tuple from a vector of JanetEnum
    pub fn from_vec(values: Vec<JanetEnum>) -> Self {
        unsafe {
            let tuple_ptr = janet_tuple_begin(values.len() as i32);
            for (i, value) in values.iter().enumerate() {
                *tuple_ptr.add(i) = value.to_janet();
            }
            let janet_tuple = janet_tuple_end(tuple_ptr);
            Self { raw: janet_tuple }
        }
    }

    // Create from a slice of JanetEnum
    pub fn from_slice(values: &[JanetEnum]) -> Self {
        unsafe {
            let tuple_ptr = janet_tuple_begin(values.len() as i32);
            for (i, value) in values.iter().enumerate() {
                *tuple_ptr.add(i) = value.to_janet();
            }
            let janet_tuple = janet_tuple_end(tuple_ptr);
            Self { raw: janet_tuple }
        }
    }
}
