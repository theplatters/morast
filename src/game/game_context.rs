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
    pub players: [Player; 2],
    pub board: Board,
    pub game_state: GameStates,
    turn_player: PlayerID,
    pub event_manager: EventManager<'a>,
    pub card_registry: CardRegistry,
}

impl GameContext<'_> {
    pub fn new(players: [Player; 2]) -> Self {
        Self {
            players,
            board: Board::new(),
            game_state: GameStates::Init,
            turn_player: PlayerID::new(0),
            event_manager: EventManager::new(),
            card_registry: CardRegistry::new(),
        }
    }

    pub fn turn_player(&self) -> PlayerID {
        self.turn_player
    }

    pub fn other_player(&self) -> PlayerID {
        if self.players[0].id == self.turn_player {
            self.turn_player
        } else {
            self.players[1].id
        }
    }
}

impl crate::engine::janet_handler::types::janetenum::ToVoidPointer for GameContext<'_> {}
