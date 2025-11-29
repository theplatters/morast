use crate::engine::{
    error::EngineError,
    janet_handler::{
        bindings::{
            janet_array, janet_array_push, janet_unwrap_array, janet_wrap_array, Janet, JanetArray,
        },
        types::janetenum::JanetEnum,
    },
};

#[derive(Debug, Clone)]
pub struct Array {
    raw: *mut JanetArray,
}

impl Array {
    pub fn get(&self, pos: usize) -> Result<JanetEnum, EngineError> {
        unsafe {
            let element = janet_wrap_array(self.raw.add(pos));
            JanetEnum::from(element)
        }
    }

    pub(crate) fn new(item: Janet) -> Self {
        unsafe {
            Self {
                raw: janet_unwrap_array(item),
            }
        }
    }
    // Create a new tuple from a vector of JanetEnum
    pub fn from_vec(values: Vec<JanetEnum>) -> Self {
        unsafe {
            let array_ptr = janet_array(values.len() as i32);

            for value in values.iter() {
                janet_array_push(array_ptr, value.to_janet());
            }
            Self { raw: array_ptr }
        }
    }

    // Create from a slice of JanetEnum
    pub fn from_slice(values: &[JanetEnum]) -> Self {
        unsafe {
            let array_ptr = janet_array(values.len() as i32);

            for value in values.iter() {
                janet_array_push(array_ptr, value.to_janet());
            }
            Self { raw: array_ptr }
        }
    }
}
