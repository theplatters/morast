use crate::{engine::janet_handler::types::function::Function, game::error::Error};

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum Timing {
    Now,
    End(u32),
    Start(u32),
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum TargetingType {
    None,                // No targeting needed
    SingleTile,          // Click a tile
    Area { radius: u8 }, // Area around clicked tile
    Line { length: u8 }, // Line from caster
    Caster,              // Targets the card itself
    AreaAroundCaster { radius: u8 },
    AllEnemies, // All enemy units
}

#[derive(Debug)]
pub struct GameAction {
    pub function: Function,
    pub speed: Timing,
    pub targeting: TargetingType,
}

impl GameAction {
    pub fn new(function: Function, speed: Timing, targeting: TargetingType) -> Self {
        Self {
            function,
            speed,
            targeting,
        }
    }

    pub fn _eval(
        &self,
        argv: &[crate::engine::janet_handler::bindings::Janet],
    ) -> Result<crate::engine::janet_handler::types::janetenum::JanetEnum, Error> {
        self.function.eval(argv).map_err(Error::EngineError)
    }
}
