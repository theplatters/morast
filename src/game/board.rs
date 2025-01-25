use std::collections::HashMap;

use super::events::{event::Event, event_handler::EventHandler, event_manager::EventManager};

struct Index {
    x: usize,
    y: usize,
}

pub struct Tile {
    ontile: Vec<super::card::Card>,
}

pub struct Board {
    tiles: HashMap<Index, Tile>,
}

impl EventHandler for Board {
    fn handle_event(&mut self, event: &Event) -> Vec<Event> {
        match event {
            _ => panic!("Not compatible"),
        };
    }
}
