use crate::engine::janet_handler::bindings::JanetTable;

pub struct Table {
    pub raw: *mut JanetTable,
}
