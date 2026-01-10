/// High-level signature: `fn(&[Janet]) -> Janet`
///
use crate::{bindings::Janet, error::JanetError, types::janetenum::JanetEnum};

type JanetCFunction = fn(&[JanetEnum]) -> JanetEnum;
#[macro_export]
macro_rules! janet_cfun {
    ($wrapper_name:ident, $f:path) => {
        pub unsafe extern "C" fn $wrapper_name(
            argc: i32,
            argv: *mut $crate::bindings::Janet,
        ) -> $crate::bindings::Janet {
            let args: &[$crate::bindings::Janet] = if argc <= 0 {
                &[]
            } else {
                // Safety: argv points to argc Janets for this call (Janet ABI contract).
                unsafe { std::slice::from_raw_parts(argv, argc as usize) }
            };

            $f(args).to_janet()
        }
    };
}

fn wrapper(argc: usize, argv: *mut Janet, f: JanetCFunction) -> Janet {
    let args: Vec<JanetEnum> = if argc <= 0 {
        Vec::new()
    } else {
        // Safety: argv points to argc Janets for this call (Janet ABI contract).
        unsafe { std::slice::from_raw_parts(argv, argc as usize) }
            .iter()
            .copied()
            .map(JanetEnum::try_from) // or JanetEnum::from if infallible
            .collect::<Result<Vec<JanetEnum>, JanetError>>()?
    };

    f(&args).to_janet()
}
