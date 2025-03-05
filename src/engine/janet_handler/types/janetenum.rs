use std::{
    collections::HashMap,
    error::Error,
    ffi::{c_void, CStr},
    hash::Hash,
};

use macroquad::math::I16Vec2;

use crate::engine::janet_handler::{
    bindings::{
        janet_array_pop, janet_checktype, janet_is_int, janet_resolve, janet_table_to_struct,
        janet_type, janet_unwrap_array, janet_unwrap_boolean, janet_unwrap_function,
        janet_unwrap_integer, janet_unwrap_number, janet_unwrap_string, janet_unwrap_symbol,
        janet_unwrap_table, janet_unwrap_u64, janet_wrap_integer, janet_wrap_nil,
        janet_wrap_number, janet_wrap_pointer, Janet, JanetArray, JANET_TYPE_JANET_ARRAY,
        JANET_TYPE_JANET_BOOLEAN, JANET_TYPE_JANET_FUNCTION, JANET_TYPE_JANET_NIL,
        JANET_TYPE_JANET_NUMBER, JANET_TYPE_JANET_STRING, JANET_TYPE_JANET_SYMBOL,
        JANET_TYPE_JANET_TABLE,
    },
    controller::Environment,
};

use super::function::Function;

pub trait ToVoidPointer {}
pub trait JanetItem {
    fn to_janet(&self) -> Janet;
}

#[derive(Debug)]
pub enum JanetEnum {
    _Int(i32),
    _UInt(u64),
    _Float(f64),
    _Bool(bool),
    _String(String),
    _Function(Function),
    _Array(Vec<JanetEnum>),
    _HashMap(HashMap<String, JanetEnum>),
    _Null,
}

impl Hash for JanetEnum {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
    }
}

impl JanetEnum {
    pub fn unwrap_array(mut arr: JanetArray) -> Result<Vec<JanetEnum>, &'static str> {
        let mut arr_vec: Vec<JanetEnum> = Vec::with_capacity(arr.count as usize);
        while arr.count != 0 {
            unsafe {
                let item = janet_array_pop(&mut arr as *mut _);
                match JanetEnum::from(item) {
                    Ok(v) => arr_vec.push(v),
                    Err(e) => return Err(e),
                }
            }
        }
        Ok(arr_vec)
    }

    pub fn get(env: &Environment, method_name: &str, namespace: Option<&str>) -> Option<JanetEnum> {
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
                println!("Return type is nill");
                return None;
            }
            match Self::from(out) {
                Ok(v) => Some(v),
                Err(e) => panic!("{}", e),
            }
        }
    }

    pub fn from(item: Janet) -> Result<JanetEnum, &'static str> {
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
                    match CStr::from_ptr(janet_unwrap_string(item) as *const std::ffi::c_char)
                        .to_str()
                    {
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
                JANET_TYPE_JANET_ARRAY => match janet_unwrap_array(item).as_mut() {
                    Some(it) => match JanetEnum::unwrap_array(*it) {
                        Ok(v) => Ok(JanetEnum::_Array(v)),
                        Err(_) => Err("Error while creating array"),
                    },
                    None => Err("Couldn't cast pointer to reference"),
                },
                JANET_TYPE_JANET_TABLE => match janet_unwrap_table(item).as_mut() {
                    Some(it) => match JanetEnum::unwrap_table(it) {
                        Some(v) => Ok(v),
                        None => Err("Error while creating table"),
                    },
                    None => Err("Couldn't cast pointer to reference"),
                },
                JANET_TYPE_JANET_SYMBOL => Ok(JanetEnum::_String(
                    CStr::from_ptr(janet_unwrap_symbol(item) as *const std::ffi::c_char)
                        .to_str()
                        .map_err(|_| "Could not cast to a string")?
                        .to_owned(),
                )),
                _ => Err("Type is Currently unsuported"),
            }
        }
    }

    fn unwrap_table(
        it: *mut crate::engine::janet_handler::bindings::JanetTable,
    ) -> Option<JanetEnum> {
        let mut map = HashMap::new();
        unsafe {
            let count = (*it).count as usize;
            let kv_ptr = janet_table_to_struct(it);
            let kvs = std::slice::from_raw_parts(kv_ptr, count);
            for kv in kvs {
                let JanetEnum::_String(key) = JanetEnum::from(kv.key.clone()).ok()? else {
                    return None;
                };
                map.insert(key, JanetEnum::from(kv.value.clone()).ok()?);
            }
        }
        Some(JanetEnum::_HashMap(map))
    }
}

pub fn to_u16_vec(item: JanetEnum) -> Option<Vec<I16Vec2>> {
    let JanetEnum::_Array(arr) = item else {
        return None;
    };

    let mut result = Vec::new();
    for item in arr {
        // Ensure the item is am `JanetEnum::_Array`
        if let JanetEnum::_Array(inner_vec) = item {
            // Ensure the inner array has exactly two elements
            if inner_vec.len() != 2 {
                return None;
            }
            // Extract the two values
            let x = match inner_vec[..] {
                [JanetEnum::_Int(value_x), JanetEnum::_Int(value_y)] => {
                    [value_x as i16, value_y as i16]
                }
                _ => return None,
            };

            result.push(I16Vec2::from_array(x));
        } else {
            return None;
        }
    }
    Some(result)
}

pub fn convert_to_u16_vec(env: &Environment, attribute: &str, name: &str) -> Option<Vec<I16Vec2>> {
    return to_u16_vec(JanetEnum::get(env, attribute, Some(name))?);
}

impl TryInto<Function> for JanetEnum {
    type Error = ();
    fn try_into(self) -> Result<Function, Self::Error> {
        if let Self::_Function(v) = self {
            Ok(v)
        } else {
            Err(())
        }
    }
}

impl TryInto<Vec<JanetEnum>> for JanetEnum {
    type Error = ();
    fn try_into(self) -> Result<Vec<JanetEnum>, Self::Error> {
        if let Self::_Array(v) = self {
            Ok(v)
        } else {
            Err(())
        }
    }
}

impl TryInto<i32> for JanetEnum {
    type Error = ();
    fn try_into(self) -> Result<i32, Self::Error> {
        if let Self::_Int(v) = self {
            Ok(v)
        } else {
            Err(())
        }
    }
}

impl TryInto<f64> for JanetEnum {
    type Error = ();
    fn try_into(self) -> Result<f64, Self::Error> {
        if let Self::_Float(v) = self {
            Ok(v)
        } else {
            Err(())
        }
    }
}

impl TryInto<String> for JanetEnum {
    type Error = ();
    fn try_into(self) -> Result<String, Self::Error> {
        if let Self::_String(v) = self {
            Ok(v)
        } else {
            Err(())
        }
    }
}

impl TryInto<bool> for JanetEnum {
    type Error = ();
    fn try_into(self) -> Result<bool, Self::Error> {
        if let Self::_Bool(v) = self {
            Ok(v)
        } else {
            Err(())
        }
    }
}

impl TryInto<HashMap<String, JanetEnum>> for JanetEnum {
    type Error = ();
    fn try_into(self) -> Result<HashMap<String, JanetEnum>, Self::Error> {
        if let Self::_HashMap(v) = self {
            Ok(v)
        } else {
            Err(())
        }
    }
}
