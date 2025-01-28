use super::{board::Board, events::event_manager::EventManager, player::Player};

pub enum GameStates {
    Draw,
    Play,
    Move,
    End,
}

pub struct GameContext<'a> {
    pub players: Vec<Player>,
    pub board: Board,
    pub game_state: GameStates,
    pub turn_player: u16,
    pub event_manager: EventManager<'a>,
}

impl<'a> GameContext<'a> {
    pub fn new(
        players: Vec<Player>,
        board: Board,
        game_state: GameStates,
        turn_player: u16,
        event_manager: EventManager<'a>,
    ) -> Self {
        Self {
            players,
            board,
            game_state,
            turn_player,
            event_manager,
        }
    }
}

impl crate::engine::janet_handler::types::janetenum::ToVoidPointer for GameContext<'_> {}
