use std::ffi::CStr;
use std::ptr;
use std::{
    ffi::{CString, c_void},
    str::FromStr,
};

use crate::bindings::{Janet, JanetAbstractType};

pub mod cfunction;
pub mod function;
pub mod janetabstract;
pub mod janetcfunction;
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
        unsafe { ptr::drop_in_place(data as *mut T) };
        0
    }
}

pub trait Get {
    fn get(&mut self, key: Janet) -> Result<Janet, i32>;
}

unsafe extern "C" fn get_thunk<T: Get>(data: *mut c_void, key: Janet, out: *mut Janet) -> i32 {
    let this = &mut *(data as *mut T);
    match this.get(key) {
        Ok(v) => {
            if !out.is_null() {
                *out = v;
            }
            0
        }
        Err(e) => e,
    }
}

impl JanetAbstractType {
    pub fn with_get_method_for<T: Get>(self) -> Self {
        self.with_get_metod(get_thunk::<T>)
    }
}
