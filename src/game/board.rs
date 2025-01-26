use std::collections::HashMap;

use macroquad::math::U16Vec2;

use super::events::{event::Event, event_handler::EventHandler};

pub struct Tile {
    ontile: Vec<super::card::Card>,
}

pub struct Board {
    tiles: HashMap<U16Vec2, Tile>,
}

impl EventHandler for Board {
    fn handle_event(&mut self, event: &Event) -> Vec<Event> {
        match event {
            Event::DrawCard(_) => todo!(),
            Event::DiscardCard(_) => todo!(),
            Event::SendCardToHand(card_action) => todo!(),
            Event::SendCardToDiscard(card_action) => todo!(),
            Event::CardDrawn(_) => todo!(),
            Event::DeckEmpty(_) => todo!(),
            Event::CardDiscarded(_) => todo!(),
            Event::HandEmpty(_) => todo!(),
        };
    }
}
