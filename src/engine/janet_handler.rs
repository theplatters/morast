use crate::engine::janet_handler::types::cfunction::JanetRawCFunction;

pub mod api;
pub mod bindings;
pub mod controller;
mod core_constants;
mod core_functions;
pub mod types;

// 1) Tiny macro to keep the list readable/consistent
macro_rules! core_fns {
    ($( $name:literal => $cfun:path ; $docs:literal ),* $(,)?) => {
        &[
            $(
                CoreFunction { name: $name, cfun: $cfun, docs: $docs },
            )*
        ] as &[CoreFunction]
    };
}

pub struct CoreFunction {
    pub name: &'static str,
    pub cfun: JanetRawCFunction,
    pub docs: &'static str,
}
