/// High-level signature: `fn(&[Janet]) -> Janet`
///
use crate::types::janetenum::JanetEnum;

type JanetCFunction = fn(&mut [JanetEnum]) -> JanetEnum;
#[macro_export]
macro_rules! janet_cfun {
    ($wrapper_name:ident, $f:path) => {
        pub unsafe extern "C" fn $wrapper_name(
            argc: i32,
            argv: *mut $crate::bindings::Janet,
        ) -> $crate::bindings::Janet {
            let mut args: Vec<$crate::types::janetenum::JanetEnum> = if argc == 0 {
                Vec::new()
            } else {
                unsafe { std::slice::from_raw_parts(argv, argc as usize) }
                    .iter()
                    .copied()
                    .map($crate::types::janetenum::JanetEnum::try_from)
                    .collect::<Result<_, $crate::error::JanetError>>()
                    .unwrap()
            };

            match $f(&mut args) {
                Ok(v) => v.into(),
                Err(_e) => unsafe {
                    $crate::bindings::janet_panic(c"Something gone seriously wrong".as_ptr())
                },
            }
        }
    };
}
