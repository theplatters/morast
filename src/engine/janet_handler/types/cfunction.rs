use crate::engine::janet_handler::bindings::Janet;

use super::janetenum::{JanetEnum, JanetItem};

pub type JanetRawCFunction = unsafe extern "C" fn(i32, *mut Janet) -> Janet;

pub struct CFunction {
    raw: JanetRawCFunction,
}
