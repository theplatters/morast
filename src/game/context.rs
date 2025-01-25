use std::collections::HashMap;

use super::{
    board::Board,
    deck::Deck,
    events::{event::Event, event_handler::EventHandler, event_manager::EventManager},
    hand::Hand,
};

pub enum GameStates {
    Draw,
    Play,
    Move,
    End,
}

pub struct Context {
    pub decks: HashMap<u16, Deck>,
    pub hands: HashMap<u16, Hand>,
    pub discard_piles: HashMap<u16, Deck>,
    pub board: Board,
    pub game_state: GameStates,
    pub turn_player: u16,
    pub event_manager: EventManager,
}

impl crate::engine::janet_handler::types::janetenum::ToVoidPointer for Context {}
