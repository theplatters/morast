use std::fmt::Display;

use bevy::{
    ecs::{bundle::Bundle, component::Component, entity::Entity},
    math::I16Vec2,
};
use macroquad::math::U16Vec2;

use super::effect::Effect;

#[derive(Bundle, Default)]
pub struct TileBundel {
    tile: Tile,
    attack_on_tile: AttackOnTile,
    effects_on_tile: EffectsOnTile,
}

#[derive(Component, Default)]
pub struct Tile;

#[derive(Component, Default)]
pub struct AttackOnTile(pub U16Vec2);

#[derive(Component)]
pub struct Occupant(pub Entity);

#[derive(Component, Default)]
pub struct EffectsOnTile(pub Vec<Effect>);
