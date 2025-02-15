use events::event_scheduler::GameScheduler;
use game_context::GameContext;
use player::PlayerID;

pub mod board;
pub mod card;
pub mod events;
pub mod game_context;
mod phases;
pub mod player;

const NUM_CARDS_AT_START: u32 = 2;

pub struct Game<'a> {
    context: GameContext,
    pub scheduler: GameScheduler<'a>,
}

impl Game<'_> {
    pub fn turn_player_id(&self) -> PlayerID {
        self.context.turn_player_id()
    }

    pub fn other_player_id(&self) -> PlayerID {
        self.context.other_player_id()
    }

    pub fn get_player_gold(&self, player_id: PlayerID) -> Option<i32> {
        let player = self.context.get_player(player_id)?;
        Some(player.get_gold())
    }

    pub fn get_turn_count(&self) -> u32 {
        self.scheduler.current_turn
    }

    pub fn shuffe(&mut self, player_id: PlayerID) -> Option<()> {
        let player = self.context.get_player_mut(player_id)?;
        player.shuffle_deck();
        Some(())
    }
}

impl crate::engine::janet_handler::types::janetenum::ToVoidPointer for Game<'_> {}
