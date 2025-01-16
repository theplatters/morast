use crate::engine::janet_handler::{
    bindings::{
        janet_checktype, janet_pcall, janet_resolve, janet_type, janet_unwrap_function,
        janet_wrap_nil, Janet, JanetFunction, JanetSignal, JANET_TYPE_JANET_FUNCTION,
    },
    controller::Environment,
};

pub struct Function {
    janet_fun: *mut JanetFunction,
}

impl Function {
    pub fn new(janet_fun: *mut JanetFunction) -> Self {
        Self { janet_fun }
    }

    pub fn eval<T: super::janetenum::JanetItem>(&self, argv: &[T]) -> Result<Janet, u32> {
        let wrapped: Vec<Janet> = argv.iter().map(|x| x.to_janet()).collect();
        let signal: JanetSignal;
        unsafe {
            let mut out: Janet = janet_wrap_nil();
            signal = janet_pcall(
                self.janet_fun as *mut _,
                argv.len() as i32,
                wrapped.as_ptr(),
                &mut out as *mut _,
                std::ptr::null_mut(),
            );

            if signal != 0 {
                return Err(signal);
            }

            Ok(out)
        }
    }

    pub fn get_method(env: &Environment, method_name: &str, namespace: &str) -> Option<Function> {
        let together = format!("{namespace}/{method_name}");
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
