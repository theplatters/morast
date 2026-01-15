use std::ptr;

use crate::{
    bindings::{
        Janet, JanetAbstractType, janet_abstract, janet_unwrap_abstract, janet_wrap_abstract,
    },
    types::janetenum::JanetItem,
};

#[derive(Debug, Clone)]
pub struct JanetAbstract {
    raw: *mut std::ffi::c_void,
}

pub trait IsAbstract: Sized {
    unsafe extern "C" fn gc(data: *mut std::ffi::c_void, _len: usize) -> i32 {
        unsafe { ptr::drop_in_place(data as *mut Self) };
        0
    }
    const SIZE: usize = std::mem::size_of::<Self>();

    fn type_info() -> &'static JanetAbstractType;
}

impl JanetAbstract {
    pub fn new<T: IsAbstract>(value: T) -> Self {
        unsafe {
            let abst = janet_abstract(T::type_info(), T::SIZE);
            ptr::write(abst as *mut T, value);

            Self { raw: abst }
        }
    }

    pub fn from_janet(item: Janet) -> Self {
        unsafe {
            Self {
                raw: janet_unwrap_abstract(item),
            }
        }
    }
    /// Registering the type is required to be able to marshal the type.
    pub fn register<T: IsAbstract>() {
        let at = T::type_info();
        unsafe {
            let syn = crate::bindings::janet_wrap_symbol(crate::bindings::janet_csymbol(at.name));

            // If `abs_type_ptr` is NULL, the type is not registered, so we then register it
            let abs_type_ptr = crate::bindings::janet_get_abstract_type(syn);
            if abs_type_ptr.is_null() {
                crate::bindings::janet_register_abstract_type(at);
            }
        }
    }
}

impl From<JanetAbstract> for Janet {
    fn from(value: JanetAbstract) -> Self {
        unsafe { janet_wrap_abstract(value.raw) }
    }
}
