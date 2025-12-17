use bevy::{
    ecs::error::{DefaultErrorHandler, ErrorContext},
    prelude::*,
};

use crate::{
    engine::{janet_handler::controller::Environment, renderer::RendererPlugin},
    game::{
        board::{update_attack_values_on_add, update_attack_values_on_move, BoardPlugin, BoardRes},
        card::{
            add_cards,
            card_registry::{init_card_registry, CardRegistry},
        },
        events::GameMessagesPlugin,
        player::add_player,
        turn_controller::TurnControllerPlugin,
    },
};
mod engine;
mod game;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(CardRegistry::new())
        .insert_non_send_resource(Environment::new("scripts/loader.janet"))
        .add_plugins(GameMessagesPlugin)
        .add_systems(Startup, (init_card_registry, add_player, add_cards).chain())
        .add_plugins(BoardPlugin)
        .add_plugins(TurnControllerPlugin)
        .add_plugins(RendererPlugin)
        .run();
}
