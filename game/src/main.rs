use bevy::prelude::*;

mod actions;
mod board;
mod card;
mod components;
mod error;
mod events;
mod janet_api;
mod phases;
mod player;
mod renderer;
mod turn_controller;

use crate::{
    actions::{action_systems::ActionPlugin, targeting::systems::TargetPlugin},
    board::BoardPlugin,
    card::{
        add_cards,
        card_registry::{CardRegistry, init_card_registry},
    },
    events::GameMessagesPlugin,
    janet_api::janet_systems::{JanetSystem, read_card_list},
    player::{add_player, draw_starting_cards},
    renderer::RendererPlugin,
    turn_controller::TurnControllerPlugin,
};

// If you need deterministic randomness, store a RNG resource:
#[derive(Resource, Default)]
pub struct GameRng(pub rand::rngs::OsRng);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(CardRegistry::new())
        .add_plugins(JanetSystem)
        .init_resource::<GameRng>()
        .add_systems(
            Startup,
            (
                init_card_registry.after(read_card_list),
                add_player,
                add_cards,
                draw_starting_cards,
            )
                .chain(),
        )
        .add_plugins((
            GameMessagesPlugin,
            BoardPlugin,
            TurnControllerPlugin,
            RendererPlugin,
            ActionPlugin,
            TargetPlugin,
        ))
        .run();
}
