use super::{
    bindings::{janet_core_env, janet_env_lookup, janet_init},
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

pub fn read_script(filename: &str) {}
