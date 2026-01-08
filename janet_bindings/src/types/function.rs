use crate::{
    bindings::{
        JANET_TYPE_JANET_FUNCTION, Janet, JanetFunction, JanetSignal, janet_checktype,
        janet_csymbol, janet_pcall, janet_resolve, janet_unwrap_function, janet_wrap_nil,
    },
    controller::Environment,
    error::JanetError,
};

use super::janetenum::JanetEnum;

#[derive(Clone, Debug)]
pub struct Function {
    janet_fun: *mut JanetFunction,
}

impl Function {
    pub fn new(janet_fun: *mut JanetFunction) -> Self {
        Self { janet_fun }
    }

    pub fn eval(&self, argv: &[Janet]) -> Result<JanetEnum, JanetError> {
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
                return Err(JanetError::Signal(format!("Got signal {}", signal)));
            }

            JanetEnum::from(out)
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
                janet_csymbol(c_function_name.as_ptr()),
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
