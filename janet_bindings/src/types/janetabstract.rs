use std::ptr;

use crate::{
    bindings::{
        Janet, JanetAbstractType, janet_abstract, janet_unwrap_abstract, janet_wrap_abstract,
    },
    types::janetenum::JanetItem,
};

#[derive(Debug, Clone)]
pub struct JanetAbstract {
    raw: *mut std::ffi::c_void,
}

pub trait IsAbstact: Sized {
    fn gc(data: *mut std::ffi::c_void, _len: usize) -> i32 {
        unsafe { ptr::drop_in_place(data as *mut Self) };
        0
    }
    const SIZE: usize;

    fn type_info() -> &'static JanetAbstractType;
}

impl JanetAbstract {
    pub fn new<T: IsAbstact>(value: T) -> Self {
        unsafe {
            let abst = janet_abstract(T::type_info(), T::SIZE);
            ptr::write(abst as *mut T, value);

            Self { raw: abst }
        }
    }

    pub fn from_janet(item: Janet) -> Self {
        unsafe {
            Self {
                raw: janet_unwrap_abstract(item),
            }
        }
    }
}

impl JanetItem for JanetAbstract {
    fn to_janet(&self) -> Janet {
        unsafe { janet_wrap_abstract(self.raw) }
    }
}
