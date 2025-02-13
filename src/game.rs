use events::event_scheduler::GameScheduler;
use game_context::GameContext;
use player::PlayerID;

pub mod board;
pub mod card;
pub mod events;
pub mod game_context;
mod phases;
pub mod player;

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
}

impl crate::engine::janet_handler::types::janetenum::ToVoidPointer for Game<'_> {}
