use std::ffi::CStr;
use std::ptr;
use std::{
    ffi::{c_void, CString},
    str::FromStr,
};

use crate::engine::janet_handler::bindings::{Janet, JanetAbstractType};

pub mod cfunction;
pub mod function;
pub mod janetenum;
pub mod table;
pub mod tuple;

impl JanetAbstractType {
    pub const fn new(
        name: &'static CStr,
        gc: unsafe extern "C" fn(*mut c_void, usize) -> i32,
    ) -> Self {
        JanetAbstractType {
            name: name.as_ptr(),
            gc: Some(gc),
            gcmark: None,
            get: None,
            put: None,
            marshal: None,
            unmarshal: None,
            tostring: None,
            compare: None,
            hash: None,
            next: None,
            call: None,
            length: None,
            bytes: None,
        }
    }

    pub const fn with_get_metod(
        mut self,
        get_method: unsafe extern "C" fn(*mut c_void, Janet, *mut Janet) -> i32,
    ) -> Self {
        self.get = Some(get_method);
        self
    }

    pub const fn with_put_metod(
        mut self,
        put_method: unsafe extern "C" fn(*mut c_void, Janet, Janet),
    ) -> Self {
        self.put = Some(put_method);
        self
    }

    pub unsafe extern "C" fn gc<T>(data: *mut c_void, _len: usize) -> i32 {
        // data points to the abstract payload (your Builder stored inline there)
        ptr::drop_in_place(data as *mut T);
        0
    }
}
