use bevy::{
    ecs::{bundle::Bundle, component::Component, entity::Entity},
    math::U16Vec2,
};

use crate::{board::effect::EffectTile, card::OnBoard};

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

#[derive(Component, Debug)]
#[relationship_target(relationship = EffectTile)]
pub struct EffectsOnTile(Vec<Entity>);

impl Occupant {
    pub fn get(&self) -> Entity {
        self.0
    }
}
