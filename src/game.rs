use card::{
    card_id::CardID,
    card_registry::{self, CardRegistry},
};
use events::event_scheduler::GameScheduler;
use game_context::GameContext;
use log::debug;
use macroquad::math::I16Vec2;
use player::{Player, PlayerID};

use crate::engine::{asset_loader::AssetLoader, janet_handler::controller::Environment};

pub mod board;
pub mod card;
pub mod error;
pub mod events;
pub mod game_context;
mod phases;
pub mod player;

pub struct Game {
    pub context: GameContext,
    pub scheduler: GameScheduler,
    pub card_registry: CardRegistry,
    env: Environment,
    asset_loader: AssetLoader,
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
        let mut s = Self {
            env,
            scheduler: GameScheduler::new(),
            context: GameContext::new(players),
            card_registry,
            asset_loader,
        };
        s.context
            .place(
                CardID::new(0),
                I16Vec2::new(5, 5),
                PlayerID::new(0),
                &s.card_registry,
            )
            .expect("Couldn't place card");

        s.context
            .place(
                CardID::new(1),
                I16Vec2::new(2, 3),
                PlayerID::new(0),
                &s.card_registry,
            )
            .expect("Couldn't place card");
        s
    }
    pub fn turn_player_id(&self) -> PlayerID {
        self.context.turn_player_id()
    }

    pub fn other_player_id(&self) -> PlayerID {
        self.context.other_player_id()
    }

    pub fn advance_turn(&mut self) {
        self.context
            .proces_turn_begin(&mut self.scheduler, &self.card_registry);

        debug!("scheduler {:?}", self.scheduler);
    }

    pub fn end_turn(&mut self) {
        self.context
            .proces_turn_end(&mut self.scheduler, &self.card_registry);

        self.advance_turn();
    }
}
