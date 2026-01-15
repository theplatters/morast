use bevy::{
    ecs::{bundle::Bundle, component::Component, entity::Entity},
    math::U16Vec2,
};

use crate::{board::effect::EffectTile, card::OnBoard};

#[derive(Bundle, Default)]
pub struct TileBundel {
    tile: Tile,
}

#[derive(Component, Clone, Debug, PartialEq, Eq, Default)]
pub struct Tile;

#[derive(Component, Clone, Debug, PartialEq, Eq)]
pub struct Position(pub U16Vec2);

#[derive(Component, Clone, Debug, PartialEq, Eq)]
#[relationship_target(relationship = OnBoard)]
pub struct Occupant(Entity);

#[derive(Component, Clone, Debug, PartialEq, Eq)]
#[relationship_target(relationship = EffectTile)]
pub struct EffectsOnTile(Vec<Entity>);

impl Occupant {
    pub fn get(&self) -> Entity {
        self.0
    }
}
