use bevy::{
    ecs::{bundle::Bundle, component::Component, entity::Entity},
    math::U16Vec2,
};

use crate::game::card::OnBoard;

#[derive(Bundle, Default)]
pub struct TileBundel {
    tile: Tile,
}

#[derive(Component, Default)]
pub struct Tile;

#[derive(Component)]
pub struct Position(pub U16Vec2);

#[derive(Component)]
#[relationship_target(relationship = OnBoard)]
pub struct Occupant(Entity);

impl Occupant {
    pub fn get(&self) -> Entity {
        self.0
    }
}
