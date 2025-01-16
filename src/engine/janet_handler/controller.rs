use super::{
    bindings::{
        janet_core_env, janet_deinit, janet_dostring, janet_env_lookup, janet_init, Janet,
        JanetTable,
    },
    types::table::Table,
};

pub struct Environment {
    pub env: Table,
    pub lookup: Table,
}

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

impl Environment {
    pub fn env_ptr(&self) -> *mut JanetTable {
        self.env.raw
    }
}

pub fn do_string(env: &Environment, string: &str) -> Janet {
    let mut out: Janet = Janet {
        pointer: std::ptr::null_mut(),
    };
    unsafe {
        janet_dostring(
            env.env_ptr(),
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

pub fn read_script(env: &Environment, filename: &str) -> Result<Janet, Box<dyn std::error::Error>> {
    let script = std::fs::read_to_string(filename)?;
    let mut out: Janet = Janet {
        pointer: std::ptr::null_mut(),
    };
    unsafe {
        janet_dostring(
            env.env_ptr(),
            std::ffi::CString::new(script)
                .expect("CString::new failed")
                .as_ptr(),
            std::ffi::CString::new(filename)
                .expect("CString::new failed")
                .as_ptr(),
            &mut out as *mut Janet,
        );
    }
    Ok(out)
}
