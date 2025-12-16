use std::sync::Arc;

use bevy::prelude::*;
mod config;

use crate::{
    engine::{
        asset_loader::AssetLoader,
        janet_handler::controller::Environment,
        renderer::{render_config::RenderConfig, Renderer, RendererPlugin},
    },
    game::{
        board::{update_attack_values_on_add, update_attack_values_on_move, Board},
        card::card_registry::CardRegistry,
        events::GameMessagesPlugin,
        startup_systems::*,
        turn_controller::TurnControllerPlugin,
    },
};
mod engine;
mod game;

fn main() {
    let render_config = Arc::new(RenderConfig::default());

    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(CardRegistry::new())
        .insert_resource(Renderer::new(render_config))
        .insert_resource(AssetLoader::new(
            std::env::current_dir().expect("").to_str().expect(""),
        ))
        .insert_non_send_resource(Environment::new())
        .add_plugins(GameMessagesPlugin)
        .add_systems(Startup, (init_card_registry, add_player, add_cards).chain())
        .add_systems(
            Startup,
            (Board::setup_board, Board::setup_player_bases).chain(),
        )
        .add_plugins(TurnControllerPlugin)
        .add_plugins(RendererPlugin)
        .add_observer(update_attack_values_on_add)
        .add_observer(update_attack_values_on_move)
        .run();
}
