use std::collections::HashMap;

use macroquad::math::U16Vec2;

use super::{
    card::{card_holder::CardHolder, card_registry::CardID, Card},
    events::{
        actions::{CardAction, PlaceOnBoardAction},
        event::Event,
        event_handler::EventHandler,
    },
};

#[derive(Debug)]
pub struct Tile {
    ontile: CardHolder,
}

impl Tile {
    pub fn place(&mut self, card: CardID) {
        self.ontile.add_card(card);
    }
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

    pub fn place(
        &mut self,
        &PlaceOnBoardAction {
            card,
            index,
            cost,
            player,
        }: &PlaceOnBoardAction,
    ) -> Event {
        let Some(tile) = self.tiles.get_mut(&index) else {
            return Event::InvalidTile(index);
        };

        tile.place(card);
        Event::CardPlaced(PlaceOnBoardAction {
            card,
            index,
            cost,
            player,
        })
    }
}

impl EventHandler for Board {
    fn handle_event(&mut self, event: &Event) -> Vec<Event> {
        match event {
            Event::PlaceCard(card_action) => vec![self.place(card_action)],
            _ => Vec::new(),
        }
    }
}
