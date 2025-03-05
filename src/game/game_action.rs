use crate::engine::janet_handler::types::function::Function;

#[derive(Debug)]
pub struct GameAction {
    function: Function,
    pub speed: u32,
}

impl GameAction {
    pub fn new(function: Function, speed: u32) -> Self {
        Self { function, speed }
    }

    pub fn eval(
        &self,
        argv: &[crate::engine::janet_handler::bindings::Janet],
    ) -> Result<crate::engine::janet_handler::types::janetenum::JanetEnum, String> {
        self.function.eval(argv)
    }
}
