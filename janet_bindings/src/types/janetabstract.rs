use std::ptr;

use crate::bindings::JanetAbstractType;

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
