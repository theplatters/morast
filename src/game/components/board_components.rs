use bevy::ecs::component::Component;
use macroquad::math::I16Vec2;

use crate::game::board::effect::Effect;

#[derive(Component)]
pub struct Tile;

#[derive(Component)]
pub struct TileModifiers {
    effects: Vec<Effect>,
}

#[derive(Component)]
pub struct Position(I16Vec2);
