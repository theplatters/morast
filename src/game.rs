use std::sync::Arc;

use card::card_registry::CardRegistry;
use error::Error;
use events::event_scheduler::GameScheduler;
use game_context::GameContext;
use macroquad::window::next_frame;

use crate::{
    engine::{
        asset_loader::AssetLoader,
        janet_handler::controller::Environment,
        renderer::{render_config::RenderConfig, Renderer},
    },
    game::turn_controller::TurnController,
};

pub mod board;
pub mod card;
pub mod error;
pub mod events;
pub mod game_action;
pub mod game_context;
pub mod game_objects;
mod phases;
pub mod player;
mod turn_controller;

pub struct Game {
    pub context: GameContext,
    pub scheduler: GameScheduler,
    pub card_registry: CardRegistry,
    env: Environment,
    asset_loader: AssetLoader,
    turn_controller: TurnController,
    renderer: Renderer,
}

impl Game {
    pub async fn new() -> Self {
        let mut asset_loader =
            AssetLoader::new(std::env::current_dir().expect("").to_str().expect(""));
        let mut env = Environment::new();
        env.read_script("scripts/loader.janet")
            .expect("Could not find file");
        let mut card_registry = CardRegistry::new();
        card_registry.init(&mut env, &mut asset_loader).await;
        println!("Card Registry: {:?}", card_registry);
        let render_config = Arc::new(RenderConfig::default());
        Self {
            env,
            scheduler: GameScheduler::new(),
            context: GameContext::new(&card_registry),
            card_registry,
            asset_loader,
            turn_controller: TurnController::new(render_config.clone()),
            renderer: Renderer::new(render_config),
        }
    }

    pub async fn main_loop(&mut self) -> Result<(), Error> {
        loop {
            self.turn_controller.reset_state();
            self.context.advance_turn(&mut self.scheduler);

            self.context
                .process_turn_begin(&mut self.scheduler, &self.card_registry)?;

            self.context
                .process_main_phase(&mut self.scheduler, &self.card_registry)?;

            while !self.turn_controller.turn_over() {
                self.turn_controller.update(
                    &mut self.context,
                    &self.card_registry,
                    &mut self.scheduler,
                )?;

                self.renderer
                    .render(&self.context, &self.asset_loader, &self.card_registry)?;

                next_frame().await;
            }

            self.context
                .process_turn_end(&mut self.scheduler, &self.card_registry)?;
        }
    }
}
