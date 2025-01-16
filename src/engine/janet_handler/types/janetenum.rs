use std::ffi::{c_void, CStr};

use crate::engine::janet_handler::bindings::{
    janet_type, janet_unwrap_boolean, janet_unwrap_function, janet_unwrap_string, janet_wrap_integer, janet_wrap_number, janet_wrap_pointer, JANET_TYPE_JANET_ARRAY, JANET_TYPE_JANET_BOOLEAN, JANET_TYPE_JANET_FUNCTION, JANET_TYPE_JANET_NIL, JANET_TYPE_JANET_NUMBER, JANET_TYPE_JANET_STRING, JANET_TYPE_JANET_TABLE, JANET_TYPE_JANET_TUPLE
};

use super::function::Function;

pub trait ToVoidPointer {}
pub trait JanetItem {
    fn to_janet(&self) -> crate::engine::janet_handler::bindings::Janet;
}

enum JanetEnum {
    _Int(i64),
    _Float(f64),
    _Bool(bool),
    _String(String),
    _Struct(Box<dyn JanetItem>),
    _Function(Function)
}

impl<T> JanetItem for T
where
    T: ToVoidPointer,
{
    fn to_janet(&self) -> crate::engine::janet_handler::bindings::Janet {
        unsafe { janet_wrap_pointer(std::ptr::from_ref(self) as *mut c_void) }
    }
}

impl JanetItem for i32 {
    fn to_janet(&self) -> crate::engine::janet_handler::bindings::Janet {
        unsafe { janet_wrap_integer(*self) }
    }
}
impl JanetItem for f64 {
    fn to_janet(&self) -> crate::engine::janet_handler::bindings::Janet {
        unsafe { janet_wrap_number(*self) }
    }
}

impl JanetEnum {
    pub fn convert(item: crate::engine::janet_handler::bindings::Janet) -> JanetEnum {
        unsafe {
            match janet_type(item) {
                JANET_TYPE_JANET_FUNCTION => 
                    JanetEnum::_Function(Function::new(janet_unwrap_function(item))),
                JANET_TYPE_JANET_BOOLEAN => 
                    if janet_unwrap_boolean(item) == 1 {JanetEnum::_Bool(true)} else {JanetEnum::_Bool(false)}, 
                JANET_TYPE_JANET_STRING =>
                    match CStr::from_ptr(janet_unwrap_string(item) as *const i8).to_str(){
                        Ok(v) =>   JanetEnum::_String(String::from(v)),
                        Err(e) => panic!("whwhhwh")
                    }
                JANET_TYPE_JANET_TUPLE =>
                JANET_TYPE_JANET_TABLE =>
                JANET_TYPE_JANET_NIL => 
                JANET_TYPE_JANET_NUMBER =>
            }
        }
    }
}
