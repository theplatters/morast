use events::event_scheduler::GameScheduler;
use game_context::GameContext;
use player::PlayerID;

pub mod board;
pub mod card;
pub mod error;
pub mod events;
pub mod game_context;
mod phases;
pub mod player;

const NUM_CARDS_AT_START: u16 = 2;

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

    pub fn advance_turn(&mut self) {
        self.context.change_turn_player();
        self.context
            .draw_cards(self.context.turn_player_id(), NUM_CARDS_AT_START);
        self.scheduler.advance_turn();
    }

    pub fn shuffe_deck(&mut self, player_id: PlayerID) -> Option<()> {
        self.context.shuffe_deck(player_id)
    }

    pub fn get_turn_count(&mut self) -> u32 {
        self.scheduler.current_turn
    }

    pub fn get_player_gold(&self, player_id: PlayerID) -> Option<i32> {
        self.context.get_player(player_id).map(|p| p.get_gold())
    }
}

impl crate::engine::janet_handler::types::janetenum::ToVoidPointer for Game<'_> {}
