use janet_bindings::controller::CoreFunction;

use super::api::*;
macro_rules! core_fns {
    ($( $name:literal => $cfun:path ; $docs:literal ),* $(,)?) => {
        &[
            $(
                CoreFunction { name: $name, cfun: $cfun, docs: $docs },
            )*
        ] as &[CoreFunction]
    };
}
// 2) Your existing list can become:
pub const CORE_FUNCTIONS: &[CoreFunction] = core_fns![
    "plus" => cfun_plus; "Generates a Plus of size n",
    "cross" => cfun_cross; "Generates a Cross of size n",
    "damage" => cfun_damage; "Damages the given entity",
    "heal" => cfun_heal; "Heals the given entity"
];
