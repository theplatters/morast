use std::sync::Arc;

use bevy::prelude::*;
use game::Game;
mod config;
use config::window_config;

use crate::{
    engine::{
        asset_loader::AssetLoader,
        janet_handler::controller::Environment,
        renderer::{render_config::RenderConfig, Renderer},
    },
    game::{
        board::Board, card::card_registry::CardRegistry, startup_systems::*,
        turn_controller::TurnController,
    },
};
mod engine;
mod game;

#[macroquad::main(window_config)]
async fn main() {
    let mut game = Game::new().await;

    let render_config = Arc::new(RenderConfig::default());
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(CardRegistry::new())
        .insert_resource(TurnController::new(render_config.clone()))
        .insert_resource(Renderer::new(render_config))
        .insert_resource(AssetLoader::new(
            std::env::current_dir().expect("").to_str().expect(""),
        ))
        .insert_non_send_resource(Environment::new())
        .add_systems(Startup, (init_card_registry, add_player, add_cards).chain())
        .add_systems(
            Startup,
            (Board::setup_board, Board::setup_player_bases).chain(),
        )
        .add_observer(Board::update_attack_values_on_add)
        .add_observer(Board::update_attack_values_on_move)
        .run();

    game.main_loop()
        .await
        .map_err(|e| println!("Error: {:?}", e));
}
