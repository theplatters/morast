use bevy::prelude::*;

#[derive(Component)]
pub struct Health(u16);

impl Health {
    pub fn player_base_health() -> Self {
        Self(10)
    }
}

#[derive(Component)]
pub struct Owner(pub Entity);
