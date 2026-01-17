
pub mod bindings;
pub mod controller;
pub mod error;
pub mod types;

// 1) Tiny macro to keep the list readable/consistent
//
#[macro_export]
macro_rules! core_fns {
    ($( $name:literal => $cfun:path ; $docs:literal ),* $(,)?) => {
        &[
            $(
                CoreFunction { name: $name, cfun: $cfun, docs: $docs },
            )*
        ] as &[CoreFunction]
    };
}
