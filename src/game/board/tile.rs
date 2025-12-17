use std::ops::Deref;

use bevy::{
    ecs::{
        bundle::Bundle, component::Component, entity::Entity, hierarchy::ChildOf, observer::On,
        system::Query,
    },
    log::info,
    math::U16Vec2,
    picking::events::{Click, Pointer},
};

use super::effect::Effect;

#[derive(Bundle, Default)]
pub struct TileBundel {
    tile: Tile,
    attack_on_tile: AttackOnTile,
    effects_on_tile: EffectsOnTile,
}

#[derive(Component, Default)]
pub struct Tile;

#[derive(Component)]
pub struct Position(pub U16Vec2);

#[derive(Component, Default)]
pub struct AttackOnTile(pub U16Vec2);

impl AttackOnTile {
    pub fn zero_out(&mut self) {
        self.0 = U16Vec2::ZERO;
    }
}

impl Deref for AttackOnTile {
    type Target = U16Vec2;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl std::ops::DerefMut for AttackOnTile {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Component)]
pub struct Occupant(pub Entity);

#[derive(Component, Default)]
pub struct EffectsOnTile(pub Vec<Effect>);
