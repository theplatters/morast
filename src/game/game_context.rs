use super::{
    board::Board,
    card::card_registry::CardRegistry,
    events::event_manager::EventManager,
    player::{Player, PlayerID},
};

pub enum GameStates {
    Init,
    Draw,
    Play,
    Move,
    End,
}

pub struct GameContext<'a> {
    pub players: Vec<Player>,
    pub board: Board,
    pub game_state: GameStates,
    pub turn_player: PlayerID,
    pub event_manager: EventManager<'a>,
    pub card_registry: CardRegistry,
}

impl GameContext<'_> {
    pub fn new(players: Vec<Player>) -> Self {
        Self {
            players,
            board: Board::new(),
            game_state: GameStates::Init,
            turn_player: PlayerID::new(0),
            event_manager: EventManager::new(),
            card_registry: CardRegistry::new(),
        }
    }
}

impl crate::engine::janet_handler::types::janetenum::ToVoidPointer for GameContext<'_> {}
