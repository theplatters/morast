use std::{ffi::CString, str::FromStr};

use super::bindings::{
    janet_cfuns_prefix, janet_core_env, janet_deinit, janet_dostring, janet_env_lookup, janet_init,
    Janet, JanetReg, JanetTable,
};
use super::types::janetenum::{JanetEnum, JanetItem};
use super::types::table::Table;

pub type JanetRawCFunction = unsafe extern "C" fn(i32, *mut Janet) -> Janet;

pub struct Environment {
    pub env: Table,
    pub lookup: Table,
}

impl Environment {
    pub fn init() -> Environment {
        let mut _env = std::ptr::null_mut();
        let mut _lookup = std::ptr::null_mut();
        unsafe {
            janet_init();
            _env = janet_core_env(std::ptr::null_mut());
            _lookup = janet_env_lookup(_env);
            Environment {
                env: Table { raw: _env },
                lookup: Table { raw: _lookup },
            }
        }
    }

    pub fn env_ptr(&self) -> *mut JanetTable {
        self.env.raw
    }

    pub fn register(
        &self,
        name: &str,
        cfun: JanetRawCFunction,
        docs: &str,
        namespace: Option<&str>,
    ) {
        let function_name =
            CString::from_str(name).expect("Couldn't create function name C_String");
        let documentation = CString::from_str(docs).expect("Couldn't create docs C_String");
        let funs_null_terminated = [
            JanetReg {
                name: function_name.as_ptr(),
                cfun: Some(cfun),
                documentation: documentation.as_ptr(),
            },
            JanetReg {
                name: std::ptr::null(),
                cfun: None,
                documentation: std::ptr::null(),
            },
        ];
        unsafe {
            if namespace.is_some() {
                let namespace_cstr =
                    CString::new(namespace.unwrap()).expect("Couldn't create namespace C_String");
                janet_cfuns_prefix(
                    self.env_ptr(),
                    namespace_cstr.as_ptr(),
                    funs_null_terminated.as_ptr(),
                );
            } else {
                janet_cfuns_prefix(
                    self.env_ptr(),
                    std::ptr::null(),
                    funs_null_terminated.as_ptr(),
                );
            }
        }
    }

    pub fn do_string(&self, string: &str) -> Janet {
        let mut out: Janet = Janet {
            pointer: std::ptr::null_mut(),
        };
        unsafe {
            janet_dostring(
                self.env_ptr(),
                std::ffi::CString::new(string)
                    .expect("CString::new failed")
                    .as_ptr(),
                std::ffi::CString::new("main")
                    .expect("CString::new failed")
                    .as_ptr(),
                &mut out as *mut Janet,
            );
        }
        out
    }
    pub fn deinit() {
        unsafe {
            janet_deinit();
        }
    }
    pub fn read_script(&self, filename: &str) -> Result<JanetEnum, Box<dyn std::error::Error>> {
        let script = std::fs::read_to_string(filename)?;
        let mut out: Janet = Janet {
            pointer: std::ptr::null_mut(),
        };
        unsafe {
            janet_dostring(
                self.env_ptr(),
                std::ffi::CString::new(script)
                    .expect("CString::new failed")
                    .as_ptr(),
                std::ffi::CString::new(filename)
                    .expect("CString::new failed")
                    .as_ptr(),
                &mut out as *mut Janet,
            );
        }
        Ok(JanetEnum::from::<dyn JanetItem>(out))
    }
}
