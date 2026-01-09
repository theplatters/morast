use crate::{
    Result,
    actions::action_prototype::{Pending, ReadyToExecute},
    error::GameError,
};
use bevy::{
    app::{Plugin, Update},
    ecs::{
        component::Component,
        entity::Entity,
        hierarchy::ChildOf,
        message::MessageReader,
        query::With,
        system::{Commands, Query, Res},
    },
    log::info,
};

use crate::board::placement::CardPlayed;
use crate::card::{Playable, card_id::CardID, card_registry::CardRegistry};

pub struct ActionPlugin;

impl Plugin for ActionPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(Update, (ready_on_play_action,));
    }
}

pub fn ready_on_play_action(
    mut cards_played: MessageReader<CardPlayed>,
    card_ids: Query<&CardID>,
    card_registry: Res<CardRegistry>,
    mut commands: Commands,
) -> Result {
    for card_played in cards_played.read() {
        let card_id = card_ids.get(card_played.card)?;
        let card = card_registry.get(card_id).ok_or(GameError::CardNotFound)?;
        let Some(on_play_action) = card.on_play_action() else {
            continue;
        };

        info!("Spawning action {:?}", on_play_action);
        commands.spawn((on_play_action.clone(), Pending, ChildOf(card_played.card)));
    }
    Ok(())
}

#[derive(Component)]
pub struct ManualTargeting {
    pub chosen: Vec<Entity>, // selected target entities (creatures, tiles, etc)
}

#[derive(Component)]
pub struct AutoTargeting {
    pub chosen: Vec<Entity>, // selected target entities (creatures, tiles, etc)
}

pub fn execute_actions(actions: Query<(Entity), With<ReadyToExecute>>) {}
