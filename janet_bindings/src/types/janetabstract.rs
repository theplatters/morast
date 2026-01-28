use std::{fmt::format, ptr};

use crate::{
    bindings::{
        Janet, JanetAbstractType, janet_abstract, janet_abstract_head, janet_unwrap_abstract,
        janet_wrap_abstract,
    },
    error::JanetError,
    types::janetenum::JanetEnum,
};

#[derive(Debug)]
pub struct JanetAbstract {
    raw: *mut std::ffi::c_void,
}

impl<T: IsAbstract> From<T> for JanetEnum {
    fn from(value: T) -> Self {
        JanetEnum::Abstract(JanetAbstract::new(value))
    }
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

    pub fn from_janet<T: IsAbstract>(item: Janet) -> Result<Self, JanetError> {
        {
            let raw = unsafe { crate::bindings::janet_checkabstract(item, T::type_info()) };
            if raw.is_null() {
                return Err(JanetError::Type(
                    "Type is not the correct abstract type".to_string(),
                ));
            }

            Ok(JanetAbstract { raw })
        }
    }

    pub fn from_janet_unchecked(item: Janet) -> Self {
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

    pub fn verify<T: IsAbstract>(&self) -> bool {
        unsafe { (*janet_abstract_head(self.raw)).type_ == T::type_info() }
    }

    pub fn as_ref<T: IsAbstract>(&self) -> Option<&T> {
        if self.verify::<T>() {
            unsafe { (self.raw as *const T).as_ref() }
        } else {
            None
        }
    }
    pub fn as_mut<T: IsAbstract>(&mut self) -> Option<&mut T> {
        if self.verify::<T>() {
            unsafe { (self.raw as *mut T).as_mut() }
        } else {
            None
        }
    }
}

impl From<JanetAbstract> for Janet {
    fn from(value: JanetAbstract) -> Self {
        unsafe { janet_wrap_abstract(value.raw) }
    }
}
