use std::ptr::{from_mut, from_ref};

use crate::engine::janet_handler::bindings::{
    janet_pcall, janet_resolve, janet_unwrap_function, Janet, JanetFunction, JanetSignal,
    JanetTable,
};

use super::table::Table;

pub struct Function {
    janet_fun: JanetFunction,
}

impl Function {
    fn eval(&self, argv: &[Janet]) -> Result<Janet, u32> {
        let mut out: Janet = Janet {
            pointer: std::ptr::null_mut(),
        };
        let signal: JanetSignal;
        unsafe {
            signal = janet_pcall(
                std::ptr::from_ref(&self.janet_fun) as *mut JanetFunction,
                argv.len() as i32,
                argv.as_ptr(),
                std::ptr::from_mut(&mut out),
                std::ptr::null_mut(),
            );
        }
        if signal != 0 {
            return Err(signal);
        }
        Ok(out)
    }

    fn get_method(env: &Table, method_name: &str, namespace: &str) -> *const JanetFunction {
        let together = format!("{namespace}{method_name}").into_bytes();
        let mut out: Janet = Janet {
            pointer: std::ptr::null_mut(),
        };

        unsafe {
            janet_resolve(
                std::ptr::from_ref(env) as *mut JanetTable,
                std::ptr::from_ref(&together) as *const u8,
                std::ptr::from_mut(&mut out),
            );
            janet_unwrap_function(out)
        }
    }
}
