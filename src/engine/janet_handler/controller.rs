use std::{
    ffi::{CString, NulError},
    str::FromStr,
};

use super::{
    api::{cfun_discard, cfun_draw, cfun_getgold, cfun_other_player, cfun_turn_player},
    bindings::{
        janet_cfuns_prefix, janet_core_env, janet_deinit, janet_dostring, janet_env_lookup,
        janet_init, Janet, JanetReg, JanetTable,
    },
    types::{
        cfunction::{CFunction, JanetRawCFunction},
        janetenum::JanetEnum,
        table::Table,
    },
};

pub struct Environment {
    pub env: Table,
    pub lookup: Table,
}

impl Environment {
    pub fn new() -> Environment {
        let mut _env = std::ptr::null_mut();
        let mut _lookup = std::ptr::null_mut();
        unsafe {
            janet_init();
            _env = janet_core_env(std::ptr::null_mut());
            _lookup = janet_env_lookup(_env);
            let env = Environment {
                env: Table { raw: _env },
                lookup: Table { raw: _lookup },
            };
            env.register_core_functions();
            env
        }
    }

    fn register_core_functions(&self) {
        let functions = [
            (
                "draw",
                cfun_draw as JanetRawCFunction,
                "Draws a card for the current player",
            ),
            (
                "discard",
                cfun_discard as JanetRawCFunction,
                "Discards a card from the hand",
            ),
            (
                "get-gold",
                cfun_getgold as JanetRawCFunction,
                "Get's the amount of gold",
            ),
            (
                "turn-player",
                cfun_turn_player as JanetRawCFunction,
                "Get's the current player",
            ),
            (
                "other-player",
                cfun_other_player as JanetRawCFunction,
                "Get's the other player",
            ),
        ];

        for (name, fun, desc) in functions {
            self.register(name, fun, desc, Some("std"))
                .unwrap_or_else(|_| panic!("Could not register {} function", name));
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
    pub fn read_script(&self, filename: &str) -> Result<JanetEnum, &str> {
        let script = std::fs::read_to_string(filename).map_err(|_| "Couldn't read file")?;
        let mut out: Janet = Janet {
            pointer: std::ptr::null_mut(),
        };
        unsafe {
            janet_dostring(
                self.env_ptr(),
                std::ffi::CString::new(script)
                    .map_err(|_| "CString::new failed")?
                    .as_ptr(),
                std::ffi::CString::new(filename)
                    .map_err(|_| "CString::new failed")?
                    .as_ptr(),
                &mut out as *mut Janet,
            );
        }
        JanetEnum::from::<i32>(out)
    }
}
