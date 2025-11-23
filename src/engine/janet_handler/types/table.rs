use std::ffi::{CStr, CString};

use crate::engine::janet_handler::bindings::{
    janet_cstring, janet_csymbol, janet_table_get, janet_wrap_integer, janet_wrap_string,
    janet_wrap_symbol, Janet, JanetTable,
};

use super::janetenum::JanetEnum;

#[derive(Debug)]
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

        let value_err = JanetEnum::from(unsafe {
            janet_table_get(self.raw, janet_wrap_symbol(janet_csymbol(key_ptr)))
        });

        if value_err.is_err() {
            eprintln!("Error: {:?}", value_err.unwrap_err());
            return None;
        } else {
            match value_err.ok()? {
                JanetEnum::Null => None,
                value => Some(value),
            }
        }
    }
    pub fn get_int(&self, key: i32) -> Option<JanetEnum> {
        let value =
            JanetEnum::from(unsafe { janet_table_get(self.raw, janet_wrap_integer(key)) }).ok()?;

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
}
