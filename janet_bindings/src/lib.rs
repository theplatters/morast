use crate::types::cfunction::JanetRawCFunction;

mod bindings;
mod controller;
mod error;
mod types;

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
