use std::ffi::CString;

use crate::bindings::{
    JanetTable, janet_csymbol, janet_table_get, janet_wrap_integer, janet_wrap_symbol,
};
use crate::types::tuple::Tuple;

use super::janetenum::JanetEnum;

#[derive(Debug, Clone)]
pub struct Table {
    pub raw: *mut JanetTable,
}

impl Table {
    pub fn new(raw: *mut JanetTable) -> Self {
        Self { raw }
    }

    pub fn get(&self, key: &str) -> Option<JanetEnum> {
        let key_cstring = CString::new(key).unwrap();
        let key_ptr = key_cstring.as_ptr();

        JanetEnum::try_from(unsafe {
            janet_table_get(self.raw, janet_wrap_symbol(janet_csymbol(key_ptr)))
        })
        .ok()
    }

    pub fn get_int(&self, key: i32) -> Option<JanetEnum> {
        let value =
            JanetEnum::try_from(unsafe { janet_table_get(self.raw, janet_wrap_integer(key)) })
                .ok()?;

        unsafe {
            println!("Value: {:?}", value);

            if !self.raw.is_null() {
                // Ensure raw pointer is valid before dereferencing
                println!("{}", (*self.raw).count);
            }
        }

        match value {
            JanetEnum::Null => None,
            _ => Some(value),
        }
    }

    // Helper methods for specific types with string keys
    pub fn get_string(&self, key: &str) -> Option<String> {
        match self.get(key)? {
            JanetEnum::String(s) => Some(s),
            _ => None,
        }
    }

    pub fn get_integer(&self, key: &str) -> Option<i32> {
        match self.get(key)? {
            JanetEnum::Int(i) => Some(i),
            _ => None,
        }
    }

    pub fn get_uint(&self, key: &str) -> Option<u64> {
        match self.get(key)? {
            JanetEnum::UInt(u) => Some(u),
            _ => None,
        }
    }

    pub fn get_float(&self, key: &str) -> Option<f64> {
        match self.get(key)? {
            JanetEnum::Float(f) => Some(f),
            _ => None,
        }
    }

    pub fn get_bool(&self, key: &str) -> Option<bool> {
        match self.get(key)? {
            JanetEnum::Bool(b) => Some(b),
            _ => None,
        }
    }

    pub fn get_tuple(&self, key: &str) -> Option<Tuple> {
        match self.get(key)? {
            JanetEnum::Tuple(b) => Some(b),
            _ => None,
        }
    }

    pub fn get_array(&self, key: &str) -> Option<Vec<JanetEnum>> {
        match self.get(key)? {
            JanetEnum::Array(arr) => Some(arr),
            _ => None,
        }
    }

    pub fn get_table(&self, key: &str) -> Option<Table> {
        match self.get(key)? {
            JanetEnum::Table(table) => Some(table),
            _ => None,
        }
    }

    pub fn get_function(&self, key: &str) -> Option<super::function::Function> {
        match self.get(key)? {
            JanetEnum::Function(func) => Some(func),
            _ => None,
        }
    }

    // Helper methods for numeric types that can be coerced
    pub fn get_number(&self, key: &str) -> Option<f64> {
        self.get(key)?.as_number()
    }

    pub fn get_number_as_int(&self, key: &str) -> Option<i32> {
        self.get_number(key).map(|n| n as i32)
    }

    pub fn get_number_as_uint(&self, key: &str) -> Option<u64> {
        self.get_number(key)
            .and_then(|n| if n >= 0.0 { Some(n as u64) } else { None })
    }

    // Convenience methods that return Results instead of Options
    pub fn require_string(&self, key: &str) -> Result<String, String> {
        self.get_string(key)
            .ok_or_else(|| format!("Required string field '{}' not found or wrong type", key))
    }

    pub fn require_integer(&self, key: &str) -> Result<i32, String> {
        self.get_integer(key)
            .ok_or_else(|| format!("Required integer field '{}' not found or wrong type", key))
    }

    pub fn require_uint(&self, key: &str) -> Result<u64, String> {
        self.get_uint(key)
            .ok_or_else(|| format!("Required uint field '{}' not found or wrong type", key))
    }

    pub fn require_float(&self, key: &str) -> Result<f64, String> {
        self.get_float(key)
            .ok_or_else(|| format!("Required float field '{}' not found or wrong type", key))
    }

    pub fn require_bool(&self, key: &str) -> Result<bool, String> {
        self.get_bool(key)
            .ok_or_else(|| format!("Required bool field '{}' not found or wrong type", key))
    }

    pub fn require_array(&self, key: &str) -> Result<Vec<JanetEnum>, String> {
        self.get_array(key)
            .ok_or_else(|| format!("Required array field '{}' not found or wrong type", key))
    }

    pub fn require_table(&self, key: &str) -> Result<Table, String> {
        self.get_table(key)
            .ok_or_else(|| format!("Required table field '{}' not found or wrong type", key))
    }

    // Helper method to check if a key exists
    pub fn has_key(&self, key: &str) -> bool {
        self.get(key).is_some()
    }

    // Method to get with default values
    pub fn get_string_or(&self, key: &str, default: &str) -> String {
        self.get_string(key).unwrap_or_else(|| default.to_string())
    }

    pub fn get_integer_or(&self, key: &str, default: i32) -> i32 {
        self.get_integer(key).unwrap_or(default)
    }

    pub fn get_uint_or(&self, key: &str, default: u64) -> u64 {
        self.get_uint(key).unwrap_or(default)
    }

    pub fn get_float_or(&self, key: &str, default: f64) -> f64 {
        self.get_float(key).unwrap_or(default)
    }

    pub fn get_bool_or(&self, key: &str, default: bool) -> bool {
        self.get_bool(key).unwrap_or(default)
    }
}
