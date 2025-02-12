use std::collections::HashMap;

use macroquad::math::U16Vec2;

use super::{
    card::card_id::CardID,
    events::{actions::PlaceOnBoardAction, event::Event, event_handler::EventHandler},
    player::PlayerID,
};

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
pub struct CardOnBoard {
    card_id: CardID,
    player_id: PlayerID,
}

#[derive(Debug)]
pub struct Tile {
    ontile: Vec<CardOnBoard>,
}

impl Tile {
    pub fn place(&mut self, card: CardOnBoard) {
        self.ontile.push(card);
    }
}

#[derive(Debug)]
pub struct Board {
    tiles: HashMap<U16Vec2, Tile>,
}

impl Board {
    pub fn new() -> Self {
        let mut tiles = HashMap::new();
        Self { tiles }
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

        tile.place(CardOnBoard {
            card_id: card,
            player_id: player,
        });
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
