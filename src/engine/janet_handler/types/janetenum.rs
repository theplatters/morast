use std::{
    ffi::{c_void, CStr},
    fmt::Debug,
};

use crate::engine::janet_handler::{
    bindings::{
        janet_array_pop, janet_checktype, janet_csymbol, janet_is_int, janet_resolve, janet_symbol,
        janet_table_get, janet_type, janet_unwrap_array, janet_unwrap_boolean,
        janet_unwrap_function, janet_unwrap_integer, janet_unwrap_number, janet_unwrap_pointer,
        janet_unwrap_string, janet_unwrap_u64, janet_wrap_integer, janet_wrap_nil,
        janet_wrap_number, janet_wrap_pointer, janet_wrap_symbol, Janet, JanetArray,
        JANET_TYPE_JANET_ARRAY, JANET_TYPE_JANET_BOOLEAN, JANET_TYPE_JANET_FUNCTION,
        JANET_TYPE_JANET_NIL, JANET_TYPE_JANET_NUMBER, JANET_TYPE_JANET_POINTER,
        JANET_TYPE_JANET_STRING,
    },
    controller::Environment,
};

use super::function::Function;

pub trait ToVoidPointer {}
pub trait JanetItem {
    fn to_janet(&self) -> Janet;
}

pub enum JanetEnum {
    _Int(i32),
    _UInt(u64),
    _Float(f64),
    _Bool(bool),
    _String(String),
    _Struct(Box<dyn JanetItem>),
    _Function(Function),
    _Array(Vec<JanetEnum>),
    _Null,
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
    fn unwrap_array<T>(arr: &mut JanetArray) -> Result<Vec<JanetEnum>, &'static str>
    where
        T: JanetItem + 'static,
    {
        let mut arr_vec: Vec<JanetEnum> = Vec::with_capacity(arr.count as usize);
        while arr.count != 0 {
            unsafe {
                let item = janet_array_pop(arr);
                match JanetEnum::from::<T>(item) {
                    Ok(v) => arr_vec.push(v),
                    Err(e) => return Err(e),
                }
            }
        }
        Ok(arr_vec)
    }

    pub fn get<T>(
        env: &Environment,
        method_name: &str,
        namespace: Option<&str>,
    ) -> Option<JanetEnum>
    where
        T: JanetItem + 'static,
    {
        let together = match namespace {
            None => method_name.to_string(),
            Some(n) => format!("{n}/{method_name}"),
        };
        let c_function_name = match std::ffi::CString::new(together) {
            Ok(it) => it,
            Err(_) => return None,
        };

        unsafe {
            let mut out: Janet = janet_wrap_nil();
            janet_resolve(
                env.env_ptr(),
                crate::engine::janet_handler::bindings::janet_csymbol(c_function_name.as_ptr()),
                &mut out as *mut Janet,
            );

            if janet_checktype(out, JANET_TYPE_JANET_NIL) != 0 {
                println!("AFHLVBSAIKLDJBVLJKB");
                return None;
            }
            match Self::from::<T>(out) {
                Ok(v) => Some(v),
                Err(e) => panic!("{}", e),
            }
        }
    }

    pub fn from<T>(item: Janet) -> Result<JanetEnum, &'static str>
    where
        T: JanetItem + 'static,
    {
        unsafe {
            match janet_type(item) {
                JANET_TYPE_JANET_FUNCTION => Ok(JanetEnum::_Function(Function::new(
                    janet_unwrap_function(item),
                ))),
                JANET_TYPE_JANET_BOOLEAN => {
                    if janet_unwrap_boolean(item) == 1 {
                        Ok(JanetEnum::_Bool(true))
                    } else {
                        Ok(JanetEnum::_Bool(false))
                    }
                }
                JANET_TYPE_JANET_STRING => {
                    match CStr::from_ptr(janet_unwrap_string(item) as *const i8).to_str() {
                        Ok(v) => Ok(JanetEnum::_String(String::from(v))),
                        Err(_) => Err("Casting to String failed"),
                    }
                }
                JANET_TYPE_JANET_NIL => Ok(JanetEnum::_Null),
                JANET_TYPE_JANET_NUMBER => {
                    if janet_is_int(item) == 0 {
                        Ok(JanetEnum::_Int(janet_unwrap_integer(item)))
                    } else if janet_is_int(item) == 1 {
                        Ok(JanetEnum::_UInt(janet_unwrap_u64(item)))
                    } else {
                        Ok(JanetEnum::_Float(janet_unwrap_number(item)))
                    }
                }
                JANET_TYPE_JANET_POINTER => Ok(JanetEnum::_Struct(Box::from_raw(
                    janet_unwrap_pointer(item) as *mut T,
                ))),
                JANET_TYPE_JANET_ARRAY => match janet_unwrap_array(item).as_mut() {
                    Some(it) => match JanetEnum::unwrap_array::<T>(it) {
                        Ok(v) => Ok(JanetEnum::_Array(v)),
                        Err(_) => Err("Error while creating array"),
                    },
                    None => Err("Couldn't cast pointer to reference"),
                },
                _ => Err("Type is Currently unsuported"),
            }
        }
    }
}
