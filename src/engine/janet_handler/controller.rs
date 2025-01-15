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
            env: Table { t: *_env },
            lookup: Table { t: *_lookup },
        }
    }
}

pub fn do_string(env: &Environment, string: &str) {
    unsafe {
        janet_dostring(
            env.env.get_raw_pointer() as *mut JanetTable,
            string.as_ptr() as *const i8,
            "main".as_ptr() as *const i8,
            std::ptr::null_mut(),
        );
    }
}

pub fn deinit() {
    unsafe {
        janet_deinit();
    }
}

pub fn read_script(filename: &str) {}
