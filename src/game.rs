use card::{card_id::CardID, card_registry::CardRegistry};
use error::Error;
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
            &mut self.scheduler,
            &self.card_registry,
        )
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

    pub fn move_card(&mut self, from: I16Vec2, to: I16Vec2) -> Result<(), Error> {
        let card_at_start = *self
            .context
            .get_card_at_index(&from)
            .ok_or(Error::TileEmpty)?;

        let card = self
            .card_registry
            .get(&card_at_start.card_id)
            .ok_or(Error::CardNotFound)?;
        if !self.context.is_legal_move(from, to, card) {
            return Err(Error::InvalidMove);
        }
        self.scheduler
            .schedule_now(-1, move |context| context.move_card(from, to), 1);

        self.scheduler.process_events(&mut self.context)?;
        self.context
            .update_attack_values_for_card(card_at_start, from, to, &self.card_registry);
        Ok(())
    }
}
