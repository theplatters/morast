use crate::engine::janet_handler::types::function::Function;

#[derive(Debug, Clone)]
pub struct JanetAction {
    pub function: Function,
}

impl JanetAction {
    pub fn new(function: Function) -> Self {
        Self { function }
    }
}
