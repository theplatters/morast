use std::collections::HashMap;

use macroquad::math::U16Vec2;

use super::{
    card::{card_holder::CardHolder, card_registry::CardID},
    events::{event::Event, event_handler::EventHandler},
};

#[derive(Debug)]
pub struct Tile {
    ontile: CardHolder,
}

#[derive(Debug)]
pub struct Board {
    tiles: HashMap<U16Vec2, Tile>,
}

impl Board {
    pub fn new() -> Self {
        Self {
            tiles: HashMap::new(),
        }
    }

    pub fn place(&mut self, card: CardID, index: U16Vec2) -> Vec<Event> {
        todo!()
    }
}

impl EventHandler for Board {
    fn handle_event(&mut self, event: &Event) -> Vec<Event> {
        match event {
            Event::PlaceCard(card_action) => self.place(card_action.card, card_action.index),
            _ => Vec::new(),
        }
    }
}
