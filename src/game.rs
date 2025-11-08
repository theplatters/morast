use card::{card_id::CardID, card_registry::CardRegistry};
use error::Error;
use events::event_scheduler::GameScheduler;
use game_context::GameContext;
use macroquad::{math::I16Vec2, window::next_frame};
use player::{Player, PlayerID};

use crate::engine::{asset_loader::AssetLoader, janet_handler::controller::Environment};

pub mod board;
pub mod card;
pub mod error;
pub mod events;
pub mod game_action;
pub mod game_context;
pub mod game_objects;
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
        Self {
            env,
            scheduler: GameScheduler::new(),
            context: GameContext::new(players),
            card_registry,
            asset_loader,
        }
    }

    pub fn place(
        &mut self,
        card_id: CardID,
        index: I16Vec2,
        player_id: PlayerID,
    ) -> Result<(), Error> {
        self.context.place(
            card_id,
            index,
            player_id,
            &self.card_registry,
            &mut self.scheduler,
        )?;

        self.context.update_attack_values(&self.card_registry);
        Ok(())
    }

    pub fn turn_player_id(&self) -> PlayerID {
        self.context.turn_player_id()
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
            self.context.draw_board();
            next_frame().await
        }
    }

    pub fn other_player_id(&self) -> PlayerID {
        self.context.other_player_id()
    }

    pub fn move_card(&mut self, from: I16Vec2, to: I16Vec2) -> Result<(), Error> {
        self.context.move_card(from, to, &self.card_registry)?;
        self.scheduler.process_events(&mut self.context)?;
        Ok(())
    }
}
