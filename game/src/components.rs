use bevy::prelude::*;

#[derive(Component, Debug, Clone, Copy)]
pub struct Health(u16);

impl Health {
    pub fn player_base_health() -> Self {
        Self(10)
    }
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Owner(pub Entity);
