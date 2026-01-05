use crate::{
    game::{
        actions::{
            action_prototype::{NeedsTargeting, Pending, ReadyToExecute},
            targeting::TargetetSelector,
        },
        card::Playable,
        error::GameError,
        turn_controller::TurnState,
    },
    Result,
};
use bevy::{
    app::{Plugin, Update},
    ecs::{
        entity::Entity,
        hierarchy::ChildOf,
        message::MessageReader,
        query::With,
        schedule::IntoScheduleConfigs,
        system::{Commands, Query, Res, ResMut},
    },
    log::info,
    state::{condition::in_state, state::NextState},
};

use crate::game::{
    board::placement::CardPlayed,
    card::{card_id::CardID, card_registry::CardRegistry},
};

pub struct ActionPlugin;

impl Plugin for ActionPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(
            Update,
            (
                ready_on_play_action,
                handle_pending_actions.run_if(in_state(TurnState::Idle)),
            ),
        );
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

pub fn handle_pending_actions(
    mut commands: Commands,
    actions: Query<(Entity, &TargetetSelector), With<Pending>>,
    mut next_state: ResMut<NextState<TurnState>>,
) {
    let Some((entity, targeting_type)) = actions.iter().next() else {
        return;
    };

    let needs_targeting = targeting_type.requires_selection();

    commands
        .entity(entity)
        .remove::<Pending>()
        .insert_if(NeedsTargeting, || needs_targeting)
        .insert_if(ReadyToExecute, || !needs_targeting);

    if needs_targeting {
        next_state.set(TurnState::AwaitingInputs);
        info!("Action needs targeting")
    } else {
        info!("Action is ready to execute")
    }
}

pub fn execute_actions(actions: Query<(Entity), With<ReadyToExecute>>) {}
