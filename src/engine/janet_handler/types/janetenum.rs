use core::fmt;
use std::{ffi::CStr, hash::Hash};

use macroquad::math::I16Vec2;

use crate::engine::{
    error::EngineError,
    janet_handler::{
        bindings::{
            janet_array, janet_array_push, janet_checktype, janet_getarray, janet_getinteger64,
            janet_is_int, janet_resolve, janet_type, janet_unwrap_array, janet_unwrap_boolean,
            janet_unwrap_function, janet_unwrap_integer, janet_unwrap_number, janet_unwrap_string,
            janet_unwrap_symbol, janet_unwrap_table, janet_unwrap_u64, janet_wrap_array,
            janet_wrap_boolean, janet_wrap_integer, janet_wrap_nil, janet_wrap_number,
            janet_wrap_string, janet_wrap_u64, Janet, JanetArray, JANET_TYPE_JANET_ARRAY,
            JANET_TYPE_JANET_BOOLEAN, JANET_TYPE_JANET_FUNCTION, JANET_TYPE_JANET_NIL,
            JANET_TYPE_JANET_NUMBER, JANET_TYPE_JANET_STRING, JANET_TYPE_JANET_SYMBOL,
            JANET_TYPE_JANET_TABLE, JANET_TYPE_JANET_TUPLE,
        },
        controller::Environment,
        types::tuple::Tuple,
    },
};

use super::{function::Function, table::Table};

pub trait JanetItem {
    fn to_janet(&self) -> Janet;
}

#[derive(Debug, Clone)]
pub enum JanetEnum {
    Int(i32),
    UInt(u64),
    Float(f64),
    Bool(bool),
    String(String),
    Function(Function),
    Array(Vec<JanetEnum>),
    Table(Table),
    Tuple(Tuple),
    Null,
}

impl Hash for JanetEnum {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
    }
}

impl JanetEnum {
    pub fn to_janet(&self) -> Janet {
        unsafe {
            match self {
                JanetEnum::Int(i) => janet_wrap_integer(*i),
                JanetEnum::UInt(u) => janet_wrap_u64(*u),
                JanetEnum::Float(f) => janet_wrap_number(*f),
                JanetEnum::Bool(b) => janet_wrap_boolean(if *b { 1 } else { 0 }),
                JanetEnum::String(s) => {
                    let c_str = std::ffi::CString::new(s.as_str()).unwrap_or_default();
                    janet_wrap_string(c_str.as_ptr() as *const u8)
                }
                JanetEnum::Array(arr) => {
                    let janet_arr = janet_array(arr.len() as i32);
                    for item in arr {
                        janet_array_push(janet_arr, item.to_janet());
                    }
                    janet_wrap_array(janet_arr)
                }
                JanetEnum::Null => janet_wrap_nil(),
                // Handle other types as needed
                _ => janet_wrap_nil(),
            }
        }
    }
    pub fn unwrap_array(arr: JanetArray) -> Result<Vec<JanetEnum>, EngineError> {
        let mut arr_vec: Vec<JanetEnum> = Vec::with_capacity(arr.count as usize);

        // Use a more efficient approach - iterate without popping
        unsafe {
            for i in 0..arr.count {
                let item = *arr.data.add(i as usize);
                arr_vec.push(JanetEnum::from(item)?);
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
            Self::from(out).ok()
        }
    }
    /// Check if the value is null
    pub fn is_null(&self) -> bool {
        matches!(self, JanetEnum::Null)
    }

    /// Check if the value is a number (int, uint, or float)
    pub fn is_number(&self) -> bool {
        matches!(
            self,
            JanetEnum::Int(_) | JanetEnum::UInt(_) | JanetEnum::Float(_)
        )
    }

    /// Convert any numeric type to f64
    pub fn as_number(&self) -> Option<f64> {
        match self {
            JanetEnum::Int(i) => Some(*i as f64),
            JanetEnum::UInt(u) => Some(*u as f64),
            JanetEnum::Float(f) => Some(*f),
            _ => None,
        }
    }

    pub fn from(item: Janet) -> Result<JanetEnum, EngineError> {
        unsafe {
            match janet_type(item) {
                JANET_TYPE_JANET_FUNCTION => Ok(JanetEnum::Function(Function::new(
                    janet_unwrap_function(item),
                ))),
                JANET_TYPE_JANET_BOOLEAN => {
                    if janet_unwrap_boolean(item) == 1 {
                        Ok(JanetEnum::Bool(true))
                    } else {
                        Ok(JanetEnum::Bool(false))
                    }
                }
                JANET_TYPE_JANET_STRING => {
                    match CStr::from_ptr(janet_unwrap_string(item) as *const std::ffi::c_char)
                        .to_str()
                    {
                        Ok(v) => Ok(JanetEnum::String(String::from(v))),
                        Err(_) => Err(EngineError::Cast("Casting to string failed".into())),
                    }
                }
                JANET_TYPE_JANET_NIL => Ok(JanetEnum::Null),
                JANET_TYPE_JANET_NUMBER => {
                    if janet_is_int(item) == 0 {
                        Ok(JanetEnum::Int(janet_unwrap_integer(item)))
                    } else if janet_is_int(item) == 1 {
                        Ok(JanetEnum::UInt(janet_unwrap_u64(item)))
                    } else {
                        Ok(JanetEnum::Float(janet_unwrap_number(item)))
                    }
                }
                JANET_TYPE_JANET_ARRAY => match janet_unwrap_array(item).as_mut() {
                    Some(it) => match JanetEnum::unwrap_array(*it) {
                        Ok(v) => Ok(JanetEnum::Array(v)),
                        Err(e) => Err(e),
                    },
                    None => Err(EngineError::Cast(
                        "Couldn't cast pointer to reference".into(),
                    )),
                },
                JANET_TYPE_JANET_TABLE => match janet_unwrap_table(item).as_mut() {
                    Some(it) => Ok(JanetEnum::Table(Table::new(it))),
                    None => Err(EngineError::Cast(
                        "Couldn't cast pointer to reference".into(),
                    )),
                },
                JANET_TYPE_JANET_SYMBOL => Ok(JanetEnum::String(
                    CStr::from_ptr(janet_unwrap_symbol(item) as *const std::ffi::c_char)
                        .to_str()
                        .map_err(|e| {
                            EngineError::Cast(format!(
                                "Could not cast symbol at pointer to UTF-8 string: {}",
                                e
                            ))
                        })?
                        .to_owned(),
                )),
                JANET_TYPE_JANET_TUPLE => Ok(JanetEnum::Tuple(Tuple::new(item))),
                other => Err(EngineError::Type(format!(
                    "Type '{}' is currently unsupported",
                    other
                ))),
            }
        }
    }
}

pub unsafe fn vec_to_janet_array(coords: &[I16Vec2]) -> *mut JanetArray {
    let arr = janet_array(coords.len() as i32);
    for coord in coords {
        let sub = janet_array(2);
        janet_array_push(sub, janet_wrap_integer(coord.x as i32));
        janet_array_push(sub, janet_wrap_integer(coord.y as i32));
        janet_array_push(arr, janet_wrap_array(sub));
    }
    arr
}

pub unsafe fn ptr_to_i16_vec(arr_ptr: *mut JanetArray) -> Option<Vec<I16Vec2>> {
    if arr_ptr.is_null() {
        return None;
    }
    // Safety: rely on JanetArray layout from bindings; treat data as pointer to Janet elements.
    let count = (*arr_ptr).count as usize;
    let mut out = Vec::with_capacity(count);
    for i in 0..count {
        // pointer to the i-th Janet value in the outer array
        let elem_janet_ptr = (*arr_ptr).data.add(i);
        // obtain inner array pointer from that element (index 0 of a one-element argv)
        let sub_arr = janet_getarray(elem_janet_ptr, 0);
        if sub_arr.is_null() {
            return None;
        }
        // read integers from sub array's data (first and second element)
        let x = janet_getinteger64((*sub_arr).data, 0) as i16;
        let y = janet_getinteger64((*sub_arr).data, 1) as i16;
        out.push(I16Vec2::new(x, y));
    }
    Some(out)
}

pub fn to_i16_vec(item: JanetEnum) -> Option<Vec<I16Vec2>> {
    let JanetEnum::Array(arr) = item else {
        return None;
    };

    let mut result = Vec::new();
    for item in arr {
        // Ensure the item is am `JanetEnum::_Array`
        if let JanetEnum::Array(inner_vec) = item {
            // Ensure the inner array has exactly two elements
            if inner_vec.len() != 2 {
                return None;
            }
            // Extract the two values
            let x = match inner_vec[..] {
                [JanetEnum::Int(value_x), JanetEnum::Int(value_y)] => {
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

impl fmt::Display for JanetEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            JanetEnum::Int(_) => "Int",
            JanetEnum::UInt(_) => "UInt",
            JanetEnum::Float(_) => "Float",
            JanetEnum::Bool(_) => "Bool",
            JanetEnum::String(_) => "String",
            JanetEnum::Function(_) => "Function",
            JanetEnum::Array(_) => "Array",
            JanetEnum::Table(_) => "Table",
            JanetEnum::Null => "Null",
            JanetEnum::Tuple(_) => "Tuple",
        };
        write!(f, "{}", s)
    }
}

impl TryFrom<JanetEnum> for String {
    type Error = EngineError;

    fn try_from(value: JanetEnum) -> Result<Self, Self::Error> {
        let JanetEnum::String(s) = value else {
            return Err(EngineError::Cast("Value is not a String".into()));
        };
        Ok(s)
    }
}
impl TryFrom<&JanetEnum> for String {
    type Error = EngineError;

    fn try_from(value: &JanetEnum) -> Result<Self, Self::Error> {
        let JanetEnum::String(s) = value else {
            return Err(EngineError::Cast("Value is not a String".into()));
        };
        Ok(s.to_owned())
    }
}

impl JanetEnum {
    pub fn as_int(&self) -> Option<i32> {
        match self {
            JanetEnum::Int(i) => Some(*i),
            _ => None,
        }
    }

    pub fn as_uint(&self) -> Option<u64> {
        match self {
            JanetEnum::UInt(u) => Some(*u),
            _ => None,
        }
    }

    pub fn as_float(&self) -> Option<f64> {
        match self {
            JanetEnum::Float(f) => Some(*f),
            _ => None,
        }
    }

    pub fn as_bool(&self) -> Option<bool> {
        match self {
            JanetEnum::Bool(b) => Some(*b),
            _ => None,
        }
    }

    pub fn as_string(&self) -> Option<&String> {
        match self {
            JanetEnum::String(s) => Some(s),
            _ => None,
        }
    }

    pub fn as_string_owned(&self) -> Option<String> {
        match self {
            JanetEnum::String(s) => Some(s.clone()),
            _ => None,
        }
    }

    pub fn as_function(&self) -> Option<&Function> {
        match self {
            JanetEnum::Function(f) => Some(f),
            _ => None,
        }
    }

    pub fn as_array(&self) -> Option<&Vec<JanetEnum>> {
        match self {
            JanetEnum::Array(arr) => Some(arr),
            _ => None,
        }
    }

    pub fn as_array_owned(&self) -> Option<Vec<JanetEnum>> {
        match self {
            JanetEnum::Array(arr) => Some(arr.clone()),
            _ => None,
        }
    }

    pub fn as_table(&self) -> Option<&Table> {
        match self {
            JanetEnum::Table(table) => Some(table),
            _ => None,
        }
    }

    pub fn as_tuple(&self) -> Option<&Tuple> {
        match self {
            JanetEnum::Tuple(tuple) => Some(tuple),
            _ => None,
        }
    }

    // Extract underlying types - consume self and return Option<T>
    pub fn into_int(self) -> Option<i32> {
        match self {
            JanetEnum::Int(i) => Some(i),
            _ => None,
        }
    }

    pub fn into_uint(self) -> Option<u64> {
        match self {
            JanetEnum::UInt(u) => Some(u),
            _ => None,
        }
    }

    pub fn into_float(self) -> Option<f64> {
        match self {
            JanetEnum::Float(f) => Some(f),
            _ => None,
        }
    }

    pub fn into_bool(self) -> Option<bool> {
        match self {
            JanetEnum::Bool(b) => Some(b),
            _ => None,
        }
    }

    pub fn into_string(self) -> Option<String> {
        match self {
            JanetEnum::String(s) => Some(s),
            _ => None,
        }
    }

    pub fn into_function(self) -> Option<Function> {
        match self {
            JanetEnum::Function(f) => Some(f),
            _ => None,
        }
    }

    pub fn into_array(self) -> Option<Vec<JanetEnum>> {
        match self {
            JanetEnum::Array(arr) => Some(arr),
            _ => None,
        }
    }

    pub fn into_table(self) -> Option<Table> {
        match self {
            JanetEnum::Table(table) => Some(table),
            _ => None,
        }
    }

    pub fn into_tuple(self) -> Option<Tuple> {
        match self {
            JanetEnum::Tuple(tuple) => Some(tuple),
            _ => None,
        }
    }

    // Extract with error handling - return Result<T, EngineError>
    pub fn expect_int(self) -> Result<i32, EngineError> {
        match self {
            JanetEnum::Int(i) => Ok(i),
            _ => Err(EngineError::Cast(format!("Expected Int, got {}", self))),
        }
    }

    pub fn expect_uint(self) -> Result<u64, EngineError> {
        match self {
            JanetEnum::UInt(u) => Ok(u),
            _ => Err(EngineError::Cast(format!("Expected UInt, got {}", self))),
        }
    }

    pub fn expect_float(self) -> Result<f64, EngineError> {
        match self {
            JanetEnum::Float(f) => Ok(f),
            _ => Err(EngineError::Cast(format!("Expected Float, got {}", self))),
        }
    }

    pub fn expect_bool(self) -> Result<bool, EngineError> {
        match self {
            JanetEnum::Bool(b) => Ok(b),
            _ => Err(EngineError::Cast(format!("Expected Bool, got {}", self))),
        }
    }

    pub fn expect_string(self) -> Result<String, EngineError> {
        match self {
            JanetEnum::String(s) => Ok(s),
            _ => Err(EngineError::Cast(format!("Expected String, got {}", self))),
        }
    }

    pub fn expect_function(self) -> Result<Function, EngineError> {
        match self {
            JanetEnum::Function(f) => Ok(f),
            _ => Err(EngineError::Cast(format!(
                "Expected Function, got {}",
                self
            ))),
        }
    }

    pub fn expect_array(self) -> Result<Vec<JanetEnum>, EngineError> {
        match self {
            JanetEnum::Array(arr) => Ok(arr),
            _ => Err(EngineError::Cast(format!("Expected Array, got {}", self))),
        }
    }

    pub fn expect_table(self) -> Result<Table, EngineError> {
        match self {
            JanetEnum::Table(table) => Ok(table),
            _ => Err(EngineError::Cast(format!("Expected Table, got {}", self))),
        }
    }

    pub fn expect_tuple(self) -> Result<Tuple, EngineError> {
        match self {
            JanetEnum::Tuple(tuple) => Ok(tuple),
            _ => Err(EngineError::Cast(format!("Expected Tuple, got {}", self))),
        }
    }

    // Numeric coercion with error handling
    pub fn expect_number(self) -> Result<f64, EngineError> {
        match self {
            JanetEnum::Int(i) => Ok(i as f64),
            JanetEnum::UInt(u) => Ok(u as f64),
            JanetEnum::Float(f) => Ok(f),
            _ => Err(EngineError::Cast(format!(
                "Expected numeric type, got {}",
                self
            ))),
        }
    }

    pub fn expect_number_as_int(self) -> Result<i32, EngineError> {
        self.expect_number().map(|n| n as i32)
    }

    pub fn expect_number_as_uint(self) -> Result<u64, EngineError> {
        let num = self.expect_number()?;
        if num >= 0.0 {
            Ok(num as u64)
        } else {
            Err(EngineError::Cast(format!(
                "Cannot convert negative number {} to uint",
                num
            )))
        }
    }

    // Type checking methods
    pub fn is_int(&self) -> bool {
        matches!(self, JanetEnum::Int(_))
    }

    pub fn is_uint(&self) -> bool {
        matches!(self, JanetEnum::UInt(_))
    }

    pub fn is_float(&self) -> bool {
        matches!(self, JanetEnum::Float(_))
    }

    pub fn is_bool(&self) -> bool {
        matches!(self, JanetEnum::Bool(_))
    }

    pub fn is_string(&self) -> bool {
        matches!(self, JanetEnum::String(_))
    }

    pub fn is_function(&self) -> bool {
        matches!(self, JanetEnum::Function(_))
    }

    pub fn is_array(&self) -> bool {
        matches!(self, JanetEnum::Array(_))
    }

    pub fn is_table(&self) -> bool {
        matches!(self, JanetEnum::Table(_))
    }

    pub fn is_tuple(&self) -> bool {
        matches!(self, JanetEnum::Tuple(_))
    }
}

// Add more TryFrom implementations for convenience
impl TryFrom<JanetEnum> for i32 {
    type Error = EngineError;

    fn try_from(value: JanetEnum) -> Result<Self, Self::Error> {
        value.expect_int()
    }
}

impl TryFrom<JanetEnum> for u64 {
    type Error = EngineError;

    fn try_from(value: JanetEnum) -> Result<Self, Self::Error> {
        value.expect_uint()
    }
}

impl TryFrom<JanetEnum> for f64 {
    type Error = EngineError;

    fn try_from(value: JanetEnum) -> Result<Self, Self::Error> {
        value.expect_float()
    }
}

impl TryFrom<JanetEnum> for bool {
    type Error = EngineError;

    fn try_from(value: JanetEnum) -> Result<Self, Self::Error> {
        value.expect_bool()
    }
}

impl TryFrom<JanetEnum> for Vec<JanetEnum> {
    type Error = EngineError;

    fn try_from(value: JanetEnum) -> Result<Self, Self::Error> {
        value.expect_array()
    }
}

impl TryFrom<JanetEnum> for Table {
    type Error = EngineError;

    fn try_from(value: JanetEnum) -> Result<Self, Self::Error> {
        value.expect_table()
    }
}

impl TryFrom<JanetEnum> for Function {
    type Error = EngineError;

    fn try_from(value: JanetEnum) -> Result<Self, Self::Error> {
        value.expect_function()
    }
}

impl TryFrom<JanetEnum> for Tuple {
    type Error = EngineError;

    fn try_from(value: JanetEnum) -> Result<Self, Self::Error> {
        value.expect_tuple()
    }
}

// Reference versions for non-consuming access
impl TryFrom<&JanetEnum> for i32 {
    type Error = EngineError;

    fn try_from(value: &JanetEnum) -> Result<Self, Self::Error> {
        value
            .as_int()
            .ok_or_else(|| EngineError::Cast(format!("Expected Int, got {}", value)))
    }
}

impl TryFrom<&JanetEnum> for u64 {
    type Error = EngineError;

    fn try_from(value: &JanetEnum) -> Result<Self, Self::Error> {
        value
            .as_uint()
            .ok_or_else(|| EngineError::Cast(format!("Expected UInt, got {}", value)))
    }
}

impl TryFrom<&JanetEnum> for f64 {
    type Error = EngineError;

    fn try_from(value: &JanetEnum) -> Result<Self, Self::Error> {
        value
            .as_float()
            .ok_or_else(|| EngineError::Cast(format!("Expected Float, got {}", value)))
    }
}

impl TryFrom<&JanetEnum> for bool {
    type Error = EngineError;

    fn try_from(value: &JanetEnum) -> Result<Self, Self::Error> {
        value
            .as_bool()
            .ok_or_else(|| EngineError::Cast(format!("Expected Bool, got {}", value)))
    }
}

impl TryFrom<JanetEnum> for u16 {
    type Error = EngineError;

    fn try_from(value: JanetEnum) -> Result<Self, Self::Error> {
        match value {
            JanetEnum::Int(i) => Ok(i as u16),
            JanetEnum::UInt(u) => Ok(u as u16),
            _ => Err(EngineError::Cast(format!(
                "expected integer type got  {}",
                value
            ))),
        }
    }
}

impl TryFrom<JanetEnum> for i16 {
    type Error = EngineError;

    fn try_from(value: JanetEnum) -> Result<Self, Self::Error> {
        match value {
            JanetEnum::Int(i) => Ok(i as i16),
            JanetEnum::UInt(u) => Ok(u as i16),
            _ => Err(EngineError::Cast(format!(
                "expected integer type got  {}",
                value
            ))),
        }
    }
}

impl TryFrom<&JanetEnum> for i16 {
    type Error = EngineError;

    fn try_from(value: &JanetEnum) -> Result<Self, Self::Error> {
        match value {
            JanetEnum::Int(i) => Ok(*i as i16),
            JanetEnum::UInt(u) => Ok(*u as i16),
            _ => Err(EngineError::Cast(format!(
                "expected integer type got  {}",
                value
            ))),
        }
    }
}

impl TryFrom<JanetEnum> for I16Vec2 {
    type Error = EngineError;

    fn try_from(value: JanetEnum) -> Result<Self, Self::Error> {
        match value {
            JanetEnum::Array(a) if a.len() == 2 => {
                let x = a.get(0).expect("Fail");
                let y = a.get(1).expect("Fail");

                Ok(I16Vec2::new(x.try_into()?, y.try_into()?))
            }
            JanetEnum::Tuple(t) => {
                let x = t.get(0).expect("Fail");
                let y = t.get(1).expect("Fail");

                Ok(I16Vec2::new(x.try_into()?, y.try_into()?))
            }
            _ => Err(EngineError::Cast(format!(
                "Janet type not supported expected array or tuple, got {}",
                value
            ))),
        }
    }
}
