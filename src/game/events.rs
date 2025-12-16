use bevy::{
    app::{App, Plugin},
    ecs::{entity::Entity, message::Message},
    math::U16Vec2,
};

use crate::game::{
    board::effect::{Effect, EffectType},
    player::Player,
};

#[derive(Message, Debug, Clone, PartialEq, Eq, Copy)]
pub struct SpellPlayed {
    owner: Player,
    card: Entity,
}

#[derive(Message, Debug, Clone, PartialEq, Eq, Copy)]
pub struct CreaturePlayed {
    owner: Player,
    card: Entity,
    position: U16Vec2,
}

#[derive(Message, Debug, Clone, PartialEq, Eq, Copy)]
pub struct TrapPlaced {
    owner: Player,
    card: Entity,
    position: U16Vec2,
}

#[derive(Message, Debug, Clone, PartialEq, Eq, Copy)]
pub struct CardMoved {
    pub card: Entity,
    pub from: U16Vec2,
    pub to: U16Vec2,
}

#[derive(Message, Debug, Clone, PartialEq, Eq, Copy)]
pub struct TurnEnd;

#[derive(Message, Debug, Clone, PartialEq, Eq, Copy)]
pub struct EffectAdded {
    pub effect: Effect,
    pub tile: Entity,
}

#[derive(Message, Debug, Clone, PartialEq, Eq, Copy)]
pub struct GoldAdded {
    player_id: Player,
    amount: u16,
}

#[derive(Message, Debug, Clone, PartialEq, Eq, Copy)]
pub struct CardsDrawn {
    player_id: Player,
    count: u16,
}

#[derive(Message, Debug, Clone, PartialEq, Eq, Copy)]
pub struct EffectRemoved {
    pub effect: EffectType,
    pub tile: Entity,
}

pub struct GameMessagesPlugin;

impl Plugin for GameMessagesPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<SpellPlayed>()
            .add_message::<CreaturePlayed>()
            .add_message::<TrapPlaced>()
            .add_message::<CardMoved>()
            .add_message::<TurnEnd>()
            .add_message::<EffectAdded>()
            .add_message::<GoldAdded>()
            .add_message::<CardsDrawn>()
            .add_message::<EffectRemoved>();
    }
}
