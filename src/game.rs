use card::{card_id::CardID, card_registry::CardRegistry};
use error::Error;
use events::event_scheduler::GameScheduler;
use game_context::GameContext;
use macroquad::{math::I16Vec2, window::next_frame};
use player::{Player, PlayerID};

use crate::{
    engine::{asset_loader::AssetLoader, janet_handler::controller::Environment},
    game::renderer::Renderer,
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
mod renderer;

pub struct Game {
    pub context: GameContext,
    pub scheduler: GameScheduler,
    pub card_registry: CardRegistry,
    env: Environment,
    asset_loader: AssetLoader,
    renderer: Renderer,
}

impl Game {
    pub async fn new() -> Self {
        let mut asset_loader =
            AssetLoader::new(std::env::current_dir().expect("").to_str().expect(""));
        let mut env = Environment::new();
        env.read_script("scripts/loader.janet")
            .expect("Could not find file");
        let players = [Player::new(PlayerID::new(0)), Player::new(PlayerID::new(1))];
        let card_registry = CardRegistry::new(&mut env, &mut asset_loader).await;
        println!("Card Registry: {:?}", card_registry);
        Self {
            env,
            scheduler: GameScheduler::new(),
            context: GameContext::new(players),
            card_registry,
            asset_loader,
            renderer: Renderer {},
        }
    }

    pub async fn main_loop(&mut self) -> Result<(), Error> {
        loop {
            self.context.advance_turn(&mut self.scheduler);
            self.context
                .process_turn_begin(&mut self.scheduler, &self.card_registry)?;

            //TODO: implement Main Phase
            self.context
                .process_main_phase(&mut self.scheduler, &self.card_registry)?;
            self.context
                .process_turn_end(&mut self.scheduler, &self.card_registry)?;
            next_frame().await
        }
    }
}
