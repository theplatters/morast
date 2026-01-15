use core::fmt;
use std::{ffi::CStr, hash::Hash};

use crate::{
    bindings::{
        JANET_TYPE_JANET_ABSTRACT, JANET_TYPE_JANET_ARRAY, JANET_TYPE_JANET_BOOLEAN,
        JANET_TYPE_JANET_FUNCTION, JANET_TYPE_JANET_NIL, JANET_TYPE_JANET_NUMBER,
        JANET_TYPE_JANET_STRING, JANET_TYPE_JANET_SYMBOL, JANET_TYPE_JANET_TABLE,
        JANET_TYPE_JANET_TUPLE, Janet, JanetArray, janet_array, janet_array_push, janet_checktype,
        janet_csymbol, janet_is_int, janet_resolve, janet_type, janet_unwrap_array,
        janet_unwrap_boolean, janet_unwrap_function, janet_unwrap_integer, janet_unwrap_number,
        janet_unwrap_string, janet_unwrap_symbol, janet_unwrap_table, janet_unwrap_u64,
        janet_wrap_array, janet_wrap_boolean, janet_wrap_integer, janet_wrap_nil,
        janet_wrap_number, janet_wrap_string, janet_wrap_u64,
    },
    controller::Environment,
    error::JanetError,
    types::{janetabstract::JanetAbstract, tuple::Tuple},
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
    Abstract(JanetAbstract),
    Null,
}

impl Hash for JanetEnum {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
    }
}

impl JanetEnum {
    pub fn unwrap_array(arr: JanetArray) -> Result<Vec<JanetEnum>, JanetError> {
        let mut arr_vec: Vec<JanetEnum> = Vec::with_capacity(arr.count as usize);

        unsafe {
            for i in 0..arr.count {
                let item = *arr.data.add(i as usize);
                arr_vec.push(JanetEnum::try_from(item)?);
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
                janet_csymbol(c_function_name.as_ptr()),
                &mut out as *mut Janet,
            );

            if janet_checktype(out, JANET_TYPE_JANET_NIL) != 0 {
                println!("Return type is nill");
                return None;
            }
            Self::try_from(out).ok()
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
    pub fn as_f64(&self) -> Option<f64> {
        match self {
            JanetEnum::Int(i) => Some(*i as f64),
            JanetEnum::UInt(u) => Some(*u as f64),
            JanetEnum::Float(f) => Some(*f),
            _ => None,
        }
    }
}

impl From<JanetEnum> for Janet {
    fn from(value: JanetEnum) -> Self {
        unsafe {
            match value {
                JanetEnum::Int(i) => janet_wrap_integer(i),
                JanetEnum::UInt(u) => janet_wrap_u64(u),
                JanetEnum::Float(f) => janet_wrap_number(f),
                JanetEnum::Bool(b) => janet_wrap_boolean(if b { 1 } else { 0 }),
                JanetEnum::String(s) => {
                    let c_str = std::ffi::CString::new(s.as_str()).unwrap_or_default();
                    janet_wrap_string(c_str.as_ptr() as *const u8)
                }
                JanetEnum::Array(arr) => {
                    let janet_arr = janet_array(arr.len() as i32);
                    for item in arr {
                        janet_array_push(janet_arr, item.into());
                    }
                    janet_wrap_array(janet_arr)
                }
                JanetEnum::Null => janet_wrap_nil(),
                // Handle other types as needed
                _ => janet_wrap_nil(),
            }
        }
    }
}

impl TryFrom<Janet> for JanetEnum {
    type Error = JanetError;

    fn try_from(item: Janet) -> Result<Self, Self::Error> {
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
                        Err(_) => Err(JanetError::Cast("Casting to string failed".into())),
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
                    None => Err(JanetError::Cast(
                        "Couldn't cast pointer to reference".into(),
                    )),
                },
                JANET_TYPE_JANET_TABLE => match janet_unwrap_table(item).as_mut() {
                    Some(it) => Ok(JanetEnum::Table(Table::new(it))),
                    None => Err(JanetError::Cast(
                        "Couldn't cast pointer to reference".into(),
                    )),
                },
                JANET_TYPE_JANET_SYMBOL => Ok(JanetEnum::String(
                    CStr::from_ptr(janet_unwrap_symbol(item) as *const std::ffi::c_char)
                        .to_str()
                        .map_err(|e| {
                            JanetError::Cast(format!(
                                "Could not cast symbol at pointer to UTF-8 string: {}",
                                e
                            ))
                        })?
                        .to_owned(),
                )),
                JANET_TYPE_JANET_TUPLE => Ok(JanetEnum::Tuple(Tuple::new(item))),
                JANET_TYPE_JANET_ABSTRACT => {
                    Ok(JanetEnum::Abstract(JanetAbstract::from_janet(item)))
                }
                other => Err(JanetError::Type(format!(
                    "Type '{}' is currently unsupported",
                    other
                ))),
            }
        }
    }
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
            JanetEnum::Abstract(_) => "Abstract",
        };
        write!(f, "{}", s)
    }
}

impl TryFrom<JanetEnum> for String {
    type Error = JanetError;

    fn try_from(value: JanetEnum) -> Result<Self, Self::Error> {
        let JanetEnum::String(s) = value else {
            return Err(JanetError::Cast("Value is not a String".into()));
        };
        Ok(s)
    }
}
impl TryFrom<&JanetEnum> for String {
    type Error = JanetError;

    fn try_from(value: &JanetEnum) -> Result<Self, Self::Error> {
        let JanetEnum::String(s) = value else {
            return Err(JanetError::Cast("Value is not a String".into()));
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

    // Extract with error handling - return Result<T, JanetError>
    pub fn expect_int(self) -> Result<i32, JanetError> {
        match self {
            JanetEnum::Int(i) => Ok(i),
            _ => Err(JanetError::Cast(format!("Expected Int, got {}", self))),
        }
    }

    pub fn expect_uint(self) -> Result<u64, JanetError> {
        match self {
            JanetEnum::UInt(u) => Ok(u),
            _ => Err(JanetError::Cast(format!("Expected UInt, got {}", self))),
        }
    }

    pub fn expect_float(self) -> Result<f64, JanetError> {
        match self {
            JanetEnum::Float(f) => Ok(f),
            _ => Err(JanetError::Cast(format!("Expected Float, got {}", self))),
        }
    }

    pub fn expect_bool(self) -> Result<bool, JanetError> {
        match self {
            JanetEnum::Bool(b) => Ok(b),
            _ => Err(JanetError::Cast(format!("Expected Bool, got {}", self))),
        }
    }

    pub fn expect_string(self) -> Result<String, JanetError> {
        match self {
            JanetEnum::String(s) => Ok(s),
            _ => Err(JanetError::Cast(format!("Expected String, got {}", self))),
        }
    }

    pub fn expect_function(self) -> Result<Function, JanetError> {
        match self {
            JanetEnum::Function(f) => Ok(f),
            _ => Err(JanetError::Cast(format!("Expected Function, got {}", self))),
        }
    }

    pub fn expect_array(self) -> Result<Vec<JanetEnum>, JanetError> {
        match self {
            JanetEnum::Array(arr) => Ok(arr),
            _ => Err(JanetError::Cast(format!("Expected Array, got {}", self))),
        }
    }

    pub fn expect_table(self) -> Result<Table, JanetError> {
        match self {
            JanetEnum::Table(table) => Ok(table),
            _ => Err(JanetError::Cast(format!("Expected Table, got {}", self))),
        }
    }

    pub fn expect_tuple(self) -> Result<Tuple, JanetError> {
        match self {
            JanetEnum::Tuple(tuple) => Ok(tuple),
            _ => Err(JanetError::Cast(format!("Expected Tuple, got {}", self))),
        }
    }

    // Numeric coercion with error handling
    pub fn expect_number(self) -> Result<f64, JanetError> {
        match self {
            JanetEnum::Int(i) => Ok(i as f64),
            JanetEnum::UInt(u) => Ok(u as f64),
            JanetEnum::Float(f) => Ok(f),
            _ => Err(JanetError::Cast(format!(
                "Expected numeric type, got {}",
                self
            ))),
        }
    }

    pub fn expect_number_as_int(self) -> Result<i32, JanetError> {
        self.expect_number().map(|n| n as i32)
    }

    pub fn expect_number_as_uint(self) -> Result<u64, JanetError> {
        let num = self.expect_number()?;
        if num >= 0.0 {
            Ok(num as u64)
        } else {
            Err(JanetError::Cast(format!(
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
    type Error = JanetError;

    fn try_from(value: JanetEnum) -> Result<Self, Self::Error> {
        value.expect_int()
    }
}

impl TryFrom<JanetEnum> for u64 {
    type Error = JanetError;

    fn try_from(value: JanetEnum) -> Result<Self, Self::Error> {
        value.expect_uint()
    }
}

impl TryFrom<JanetEnum> for f64 {
    type Error = JanetError;

    fn try_from(value: JanetEnum) -> Result<Self, Self::Error> {
        value.expect_float()
    }
}

impl TryFrom<JanetEnum> for bool {
    type Error = JanetError;

    fn try_from(value: JanetEnum) -> Result<Self, Self::Error> {
        value.expect_bool()
    }
}

impl TryFrom<JanetEnum> for Vec<JanetEnum> {
    type Error = JanetError;

    fn try_from(value: JanetEnum) -> Result<Self, Self::Error> {
        value.expect_array()
    }
}

impl TryFrom<JanetEnum> for Table {
    type Error = JanetError;

    fn try_from(value: JanetEnum) -> Result<Self, Self::Error> {
        value.expect_table()
    }
}

impl TryFrom<JanetEnum> for Function {
    type Error = JanetError;

    fn try_from(value: JanetEnum) -> Result<Self, Self::Error> {
        value.expect_function()
    }
}

impl TryFrom<JanetEnum> for Tuple {
    type Error = JanetError;

    fn try_from(value: JanetEnum) -> Result<Self, Self::Error> {
        value.expect_tuple()
    }
}

// Reference versions for non-consuming access
impl TryFrom<&JanetEnum> for i32 {
    type Error = JanetError;

    fn try_from(value: &JanetEnum) -> Result<Self, Self::Error> {
        value
            .as_int()
            .ok_or_else(|| JanetError::Cast(format!("Expected Int, got {}", value)))
    }
}

impl TryFrom<&JanetEnum> for u64 {
    type Error = JanetError;

    fn try_from(value: &JanetEnum) -> Result<Self, Self::Error> {
        value
            .as_uint()
            .ok_or_else(|| JanetError::Cast(format!("Expected UInt, got {}", value)))
    }
}

impl TryFrom<&JanetEnum> for f64 {
    type Error = JanetError;

    fn try_from(value: &JanetEnum) -> Result<Self, Self::Error> {
        value
            .as_float()
            .ok_or_else(|| JanetError::Cast(format!("Expected Float, got {}", value)))
    }
}

impl TryFrom<&JanetEnum> for bool {
    type Error = JanetError;

    fn try_from(value: &JanetEnum) -> Result<Self, Self::Error> {
        value
            .as_bool()
            .ok_or_else(|| JanetError::Cast(format!("Expected Bool, got {}", value)))
    }
}

impl TryFrom<JanetEnum> for u16 {
    type Error = JanetError;

    fn try_from(value: JanetEnum) -> Result<Self, Self::Error> {
        match value {
            JanetEnum::Int(i) => Ok(i as u16),
            JanetEnum::UInt(u) => Ok(u as u16),
            _ => Err(JanetError::Cast(format!(
                "expected integer type got  {}",
                value
            ))),
        }
    }
}

impl TryFrom<JanetEnum> for i16 {
    type Error = JanetError;

    fn try_from(value: JanetEnum) -> Result<Self, Self::Error> {
        match value {
            JanetEnum::Int(i) => Ok(i as i16),
            JanetEnum::UInt(u) => Ok(u as i16),
            _ => Err(JanetError::Cast(format!(
                "expected integer type got  {}",
                value
            ))),
        }
    }
}

impl TryFrom<&JanetEnum> for i16 {
    type Error = JanetError;

    fn try_from(value: &JanetEnum) -> Result<Self, Self::Error> {
        match value {
            JanetEnum::Int(i) => Ok(*i as i16),
            JanetEnum::UInt(u) => Ok(*u as i16),
            _ => Err(JanetError::Cast(format!(
                "expected integer type got  {}",
                value
            ))),
        }
    }
}

impl TryFrom<JanetEnum> for [i16; 2] {
    type Error = JanetError;

    fn try_from(value: JanetEnum) -> Result<Self, Self::Error> {
        match value {
            JanetEnum::Array(a) if a.len() == 2 => {
                let x = a.first().expect("Fail");
                let y = a.get(1).expect("Fail");

                Ok([x.try_into()?, y.try_into()?])
            }
            JanetEnum::Tuple(t) => {
                let x = t.get(0).expect("Fail");
                let y = t.get(1).expect("Fail");

                Ok([x.try_into()?, y.try_into()?])
            }
            _ => Err(JanetError::Cast(format!(
                "Janet type not supported expected array or tuple, got {}",
                value
            ))),
        }
    }
}

impl<const N: usize> From<[i32; N]> for JanetEnum {
    fn from(value: [i32; N]) -> Self {
        JanetEnum::Array(value.map(JanetEnum::Int).into())
    }
}
