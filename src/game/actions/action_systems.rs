use crate::{
    game::{card::Playable, error::GameError},
    Result,
};
use bevy::{
    app::Plugin,
    ecs::{
        message::MessageReader,
        system::{Commands, Query, Res},
    },
};

use crate::game::{
    board::placement::CardPlayed,
    card::{card_id::CardID, card_registry::CardRegistry},
};

pub struct ActionPlugin;

impl Plugin for ActionPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        todo!()
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

        commands.spawn(on_play_action.clone());
    }
    Ok(())
}

