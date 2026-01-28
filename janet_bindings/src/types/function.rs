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
pub struct JFunction {
    janet_fun: *mut JanetFunction,
}

impl JFunction {
    pub fn new(janet_fun: *mut JanetFunction) -> Self {
        Self { janet_fun }
    }

    pub fn eval(&self, argv: &mut [JanetEnum]) -> Result<JanetEnum, JanetError> {
        let signal: JanetSignal;
        let argv_as_janet: Vec<Janet> = argv.iter().map(|el| el.into()).collect();
        unsafe {
            let mut out: Janet = janet_wrap_nil();
            signal = janet_pcall(
                self.janet_fun as *mut _,
                argv.len() as i32,
                argv_as_janet.as_ptr(),
                &mut out as *mut _,
                std::ptr::null_mut(),
            );

            if signal != 0 {
                return Err(JanetError::Signal(format!("Got signal {}", signal)));
            }

            JanetEnum::try_from(out)
        }
    }

    pub fn get_method(
        env: &Environment,
        method_name: &str,
        namespace: Option<&str>,
    ) -> Option<JFunction> {
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
            Some(JFunction {
                janet_fun: janet_unwrap_function(out),
            })
        }
    }
}

unsafe impl Send for JFunction {}
unsafe impl Sync for JFunction {}
