use std::sync::Arc;

use bevy::prelude::*;

use actions::action_manager::ActionManager;
use card::card_registry::CardRegistry;
use error::Error;
use game_context::GameContext;
use macroquad::window::next_frame;

use crate::{
    engine::{
        asset_loader::AssetLoader,
        janet_handler::controller::Environment,
        renderer::{render_config::RenderConfig, Renderer},
    },
    game::{events::event_manager::EventManager, turn_controller::TurnController},
};

pub mod actions;
pub mod board;
pub mod card;
pub mod components;
pub mod error;
pub mod events;
pub mod game_context;
pub mod game_objects;
pub mod janet_action;
mod phases;
pub mod player;
pub mod startup_systems;
pub mod turn_controller;

pub struct Game {
    pub context: GameContext,
    pub scheduler: ActionManager,
    pub card_registry: CardRegistry,
    env: Environment,
    asset_loader: AssetLoader,
    turn_controller: TurnController,
    renderer: Renderer,
    event_manager: EventManager,
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
            scheduler: ActionManager::new(),
            context: GameContext::new(&card_registry),
            card_registry,
            asset_loader,
            turn_controller: TurnController::new(render_config.clone()),
            renderer: Renderer::new(render_config),
            event_manager: EventManager::default(),
        }
    }

    pub async fn main_loop(&mut self) -> Result<(), Error> {
        loop {
            self.turn_controller.reset_state();
            self.context.advance_turn(&mut self.scheduler);

            self.context
                .process_turn_begin(&mut self.scheduler, &self.card_registry)?;

            self.context.process_main_phase(&mut self.scheduler)?;

            while !self.turn_controller.turn_over() {
                if let Some(play_command) = self
                    .turn_controller
                    .update(&mut self.context, &self.card_registry)?
                {
                    self.scheduler.schedule(
                        play_command
                            .try_into()
                            .expect("Could not convert play command"),
                    );
                }

                let mut events = self
                    .scheduler
                    .process_actions(&mut self.context, &self.card_registry)?;

                while let Some(event) = events.pop() {
                    let mut actions = self
                        .event_manager
                        .process_event(event, &mut self.turn_controller, &self.card_registry)
                        .await?;

                    while let Some(action) = actions.pop() {
                        self.scheduler.schedule(action);
                    }
                    events.extend(
                        self.scheduler
                            .process_actions(&mut self.context, &self.card_registry)?,
                    );
                }

                self.renderer.render(
                    &self.context,
                    &self.turn_controller.state,
                    &self.asset_loader,
                    &self.card_registry,
                )?;

                next_frame().await;
            }
            self.context
                .process_turn_end(&mut self.scheduler, &self.card_registry)?;
        }
    }
}
