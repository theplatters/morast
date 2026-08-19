use bevy::prelude::*;
use rand::SeedableRng;

mod actions;
mod board;
mod card;
mod components;
mod def;
mod error;
mod events;
mod phases;
mod player;
mod renderer;
mod stats;
mod turn_controller;

use crate::{
    actions::{ActionPlugin, targeting::systems::TargetPlugin},
    board::BoardPlugin,
    card::{add_cards, card_registry::CardRegistry},
    def::loader::{CardPlugin, LoadState},
    events::GameMessagesPlugin,
    player::{add_player, draw_starting_cards},
    renderer::{RendererPlugin, setup_creature_on_board_renderer},
    stats::StatsPlugin,
    turn_controller::TurnControllerPlugin,
};

#[derive(Resource)]
pub struct GameRng(pub rand::rngs::StdRng);

impl Default for GameRng {
    fn default() -> Self {
        Self(rand::rngs::StdRng::from_os_rng())
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<CardRegistry>()
        .init_resource::<GameRng>()
        .add_plugins((
            CardPlugin,
            GameMessagesPlugin,
            BoardPlugin,
            TurnControllerPlugin,
            RendererPlugin,
            StatsPlugin,
            ActionPlugin,
            TargetPlugin,
        ))
        .add_systems(Startup, add_player)
        .add_systems(
            OnEnter(LoadState::Ready),
            (add_cards, draw_starting_cards, setup_creature_on_board_renderer).chain(),
        )
        .run();
}
