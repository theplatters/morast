use crate::engine::janet_handler::types::function::Function;

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum Timing {
    Now,
    End(u32),
    Start(u32),
}

#[derive(Debug)]
pub struct GameAction {
    pub function: Function,
    pub speed: Timing,
}

impl GameAction {
    pub fn new(function: Function, speed: Timing) -> Self {
        Self { function, speed }
    }

    pub fn eval(
        &self,
        argv: &[crate::engine::janet_handler::bindings::Janet],
    ) -> Result<crate::engine::janet_handler::types::janetenum::JanetEnum, String> {
        self.function.eval(argv)
    }
}
