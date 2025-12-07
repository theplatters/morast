use crate::engine::{
    error::EngineError,
    janet_handler::{
        bindings::{
            janet_tuple_begin, janet_tuple_end, janet_tuple_head, janet_unwrap_tuple, Janet,
        },
        types::janetenum::JanetEnum,
    },
};

#[derive(Debug, Clone)]
pub struct Tuple {
    raw: *const Janet,
    length: usize,
}

impl Tuple {
    pub fn get(&self, pos: usize) -> Result<JanetEnum, EngineError> {
        if pos >= self.length {
            return Err(EngineError::OutOfBounds);
        }
        unsafe { JanetEnum::from(*self.raw.add(pos)) }
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub(crate) fn new(item: Janet) -> Self {
        unsafe {
            let raw = janet_unwrap_tuple(item);
            let length = (*janet_tuple_head(raw)).length;
            Self {
                raw,
                length: length as usize,
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
            Self {
                raw: janet_tuple,
                length: values.len(),
            }
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
            Self {
                raw: janet_tuple,
                length: values.len(),
            }
        }
    }
}
