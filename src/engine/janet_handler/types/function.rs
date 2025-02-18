
use crate::engine::janet_handler::{
    bindings::{
        janet_checktype, janet_pcall, janet_resolve, janet_unwrap_function,
        janet_wrap_nil, Janet, JanetFunction, JanetSignal, JANET_TYPE_JANET_FUNCTION,
    },
    controller::Environment,
};

use super::janetenum::{JanetEnum, JanetItem};

#[derive(Clone, Debug)]
pub struct Function {
    janet_fun: *mut JanetFunction,
}

impl Function {
    pub fn new(janet_fun: *mut JanetFunction) -> Self {
        Self { janet_fun }
    }

    pub fn eval<T>(&self, argv: &[Janet]) -> Result<JanetEnum, String>
    where
        T: JanetItem + 'static,
    {
        let signal: JanetSignal;
        unsafe {
            let mut out: Janet = janet_wrap_nil();
            signal = janet_pcall(
                self.janet_fun as *mut _,
                argv.len() as i32,
                argv.as_ptr(),
                &mut out as *mut _,
                std::ptr::null_mut(),
            );

            if signal != 0 {
                return Err(format!("Got signal {}", signal));
            }

            super::janetenum::JanetEnum::from::<T>(out).map_err(|e| e.to_string())
        }
    }

    pub fn get_method(
        env: &Environment,
        method_name: &str,
        namespace: Option<&str>,
    ) -> Option<Function> {
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

            if janet_checktype(out, JANET_TYPE_JANET_FUNCTION) == 0 {
                return None;
            }
            Some(Function {
                janet_fun: janet_unwrap_function(out),
            })
        }
    }
}

unsafe impl Send for Function {}
unsafe impl Sync for Function {}
