use bevy::ecs::{entity::Entity, event::Event};
use macroquad::math::{I16Vec2, U16Vec2};

use crate::game::{components::player_components::Player, player::PlayerID};

#[derive(Event, Debug, Clone, PartialEq, Eq, Copy)]
pub struct SpellPlayed {
    owner: PlayerID,
    card: Entity,
}

#[derive(Event, Debug, Clone, PartialEq, Eq, Copy)]
pub struct CreaturePlayed {
    owner: Player,
    card: Entity,
    position: U16Vec2,
}

#[derive(Event, Debug, Clone, PartialEq, Eq, Copy)]
pub struct TrapPlaced {
    owner: PlayerID,
    card: Entity,
    position: U16Vec2,
}

#[derive(Event, Debug, Clone, PartialEq, Eq, Copy)]
pub struct CardMoved {
    card: Entity,
    from: U16Vec2,
    to: U16Vec2,
}

#[derive(Event, Debug, Clone, PartialEq, Eq, Copy)]
pub struct TurnEnd;

#[derive(Event, Debug, Clone, PartialEq, Eq, Copy)]
pub struct EffectAdded {
    effect: crate::game::board::effect::Effect,
}

#[derive(Event, Debug, Clone, PartialEq, Eq, Copy)]
pub struct GoldAdded {
    player_id: Player,
    amount: u16,
}

#[derive(Event, Debug, Clone, PartialEq, Eq, Copy)]
pub struct CardsDrawn {
    player_id: Player,
    count: u16,
}
