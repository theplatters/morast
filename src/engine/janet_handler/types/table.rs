use crate::engine::janet_handler::bindings::JanetTable;

pub struct Table {
    pub t: JanetTable,
}

impl Table {
    pub fn get_raw_pointer(&self) -> *const JanetTable {
        std::ptr::from_ref(&self.t)
    }
}
