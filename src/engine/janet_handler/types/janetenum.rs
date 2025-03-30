use std::{
    ffi::CStr,
    fmt::{self},
    hash::Hash,
};

use macroquad::math::I16Vec2;

use crate::{
    engine::janet_handler::{
        bindings::{
            janet_array_pop, janet_checktype, janet_is_int, janet_resolve, janet_type,
            janet_unwrap_array, janet_unwrap_boolean, janet_unwrap_function, janet_unwrap_integer,
            janet_unwrap_number, janet_unwrap_string, janet_unwrap_symbol, janet_unwrap_table,
            janet_unwrap_u64, janet_wrap_nil, Janet, JanetArray, JANET_TYPE_JANET_ARRAY,
            JANET_TYPE_JANET_BOOLEAN, JANET_TYPE_JANET_FUNCTION, JANET_TYPE_JANET_NIL,
            JANET_TYPE_JANET_NUMBER, JANET_TYPE_JANET_STRING, JANET_TYPE_JANET_SYMBOL,
            JANET_TYPE_JANET_TABLE,
        },
        controller::Environment,
    },
    game::error::Error,
};

use super::{function::Function, table::Table};

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
    _Table(Table),
    _Null,
}

impl Hash for JanetEnum {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
    }
}

impl JanetEnum {
    pub fn unwrap_array(mut arr: JanetArray) -> Result<Vec<JanetEnum>, String> {
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

    pub fn from(item: Janet) -> Result<JanetEnum, String> {
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
                        Err(_) => Err("Casting to String failed".to_owned()),
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
                        Err(e) => Err(format!("Error while creating array {}", e)),
                    },
                    None => Err("Couldn't cast pointer to reference".into()),
                },
                JANET_TYPE_JANET_TABLE => match janet_unwrap_table(item).as_mut() {
                    Some(it) => Ok(JanetEnum::_Table(Table::new(it))),
                    None => Err("Couldn't cast pointer to reference".into()),
                },
                JANET_TYPE_JANET_SYMBOL => Ok(JanetEnum::_String(
                    CStr::from_ptr(janet_unwrap_symbol(item) as *const std::ffi::c_char)
                        .to_str()
                        .map_err(|_| "Could not cast to a string")?
                        .to_owned(),
                )),
                janet_type => Err(format!("Type {} is Currently unsuported", janet_type)),
            }
        }
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
    to_u16_vec(JanetEnum::get(env, attribute, Some(name))?)
}

impl fmt::Display for JanetEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            JanetEnum::_Int(_) => "Int",
            JanetEnum::_UInt(_) => "UInt",
            JanetEnum::_Float(_) => "Float",
            JanetEnum::_Bool(_) => "Bool",
            JanetEnum::_String(_) => "String",
            JanetEnum::_Function(_) => "Function",
            JanetEnum::_Array(_) => "Array",
            JanetEnum::_Table(_) => "Table",
            JanetEnum::_Null => "Null",
        };
        write!(f, "{}", s)
    }
}

impl TryFrom<JanetEnum> for String {
    type Error = Error;

    fn try_from(value: JanetEnum) -> Result<Self, Self::Error> {
        let JanetEnum::_String(s) = value else {
            return Err(Error::Cast("Value is not a String".into()));
        };
        Ok(s)
    }
}
impl TryFrom<&JanetEnum> for String {
    type Error = Error;

    fn try_from(value: &JanetEnum) -> Result<Self, Self::Error> {
        let JanetEnum::_String(s) = value else {
            return Err(Error::Cast("Value is not a String".into()));
        };
        Ok(s.to_owned())
    }
}
