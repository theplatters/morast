use std::{
    ffi::{CString, NulError},
    str::FromStr,
};

use crate::bindings::janet_def;

use super::{
    bindings::{
        Janet, JanetReg, JanetTable, janet_cfuns_prefix, janet_core_env, janet_deinit,
        janet_dostring, janet_env_lookup, janet_init,
    },
    error::JanetError,
    types::{cfunction::JanetRawCFunction, janetenum::JanetEnum, table::Table},
};

pub struct Environment {
    pub env_pointer: Table,
    pub lookup: Table,
}

impl Environment {
    pub fn new(filename: &str) -> Environment {
        unsafe {
            janet_init();
            let env_pointer = janet_core_env(std::ptr::null_mut());
            let lookup = janet_env_lookup(env_pointer);
            let env = Environment {
                env_pointer: Table { raw: env_pointer },
                lookup: Table { raw: lookup },
            };

            env.read_script(filename);
            env
        }
    }

    pub fn register_constant(
        &self,
        name: &str,
        value: &JanetEnum,
        docs: Option<&str>,
    ) -> Result<(), NulError> {
        let name_cstr = CString::new(name)?;
        let doc_cstr = docs.map(CString::new).transpose()?;

        unsafe {
            let janet_value = value.to_janet();
            janet_def(
                self.env_ptr(),
                name_cstr.as_ptr(),
                janet_value,
                doc_cstr.as_ref().map_or(std::ptr::null(), |s| s.as_ptr()),
            );
        }
        Ok(())
    }

    pub fn env_ptr(&self) -> *mut JanetTable {
        self.env_pointer.raw
    }

    pub fn register(
        &self,
        name: &str,
        cfun: JanetRawCFunction,
        docs: &str,
        namespace: Option<&str>,
    ) -> Result<(), NulError> {
        let function_name = CString::from_str(name)?;
        let documentation = CString::from_str(docs)?;
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
            if let Some(name) = namespace {
                let namespace_cstr = CString::new(name)?;
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
        Ok(())
    }

    pub fn do_string(&self, string: &str) -> Result<Janet, NulError> {
        let mut out: Janet = Janet {
            pointer: std::ptr::null_mut(),
        };
        unsafe {
            janet_dostring(
                self.env_ptr(),
                std::ffi::CString::new(string)?.as_ptr(),
                std::ffi::CString::new("main")?.as_ptr(),
                &mut out as *mut Janet,
            );
        }
        Ok(out)
    }
    pub fn deinit() {
        unsafe {
            janet_deinit();
        }
    }
    pub fn read_script(&self, filename: &str) -> Result<JanetEnum, JanetError> {
        let script = std::fs::read_to_string(filename).map_err(|e| {
            JanetError::File(format!("Couldn't read file {}, Error: {}", filename, e))
        })?;
        let mut out: Janet = Janet {
            pointer: std::ptr::null_mut(),
        };
        unsafe {
            janet_dostring(
                self.env_ptr(),
                std::ffi::CString::new(script)
                    .map_err(JanetError::String)?
                    .as_ptr(),
                std::ffi::CString::new(filename)
                    .map_err(JanetError::String)?
                    .as_ptr(),
                &mut out as *mut Janet,
            );
        }
        JanetEnum::from(out)
    }
}
