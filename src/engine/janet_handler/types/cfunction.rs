use crate::engine::janet_handler::bindings::Janet;

use super::janetenum::{JanetEnum, JanetItem};

pub type JanetRawCFunction = unsafe extern "C" fn(i32, *mut Janet) -> Janet;
pub type RustJanetFunction = unsafe extern "C" fn(&[Box<dyn JanetItem>]) -> JanetEnum;

pub struct CFunction {
    raw: JanetRawCFunction,
}
