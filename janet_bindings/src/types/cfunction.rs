use crate::bindings::Janet;

pub type JanetRawCFunction = unsafe extern "C" fn(i32, *mut Janet) -> Janet;

pub struct CFunction {
    pub raw: JanetRawCFunction,
}
