use bevy::prelude::*;

#[derive(Component, Debug, Clone, Copy)]
pub struct Health(pub(crate) u16);

impl Health {
    pub fn player_base_health() -> Self {
        Self(10)
    }

    pub fn value(&self) -> u16 {
        self.0
    }

    pub fn set_value(&mut self, value: u16) {
        self.0 = value;
    }
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Owner(pub Entity);

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Caster(pub Entity);
