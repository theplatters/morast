use std::collections::HashMap;

use macroquad::math::U16Vec2;

use super::{
    card::card_id::CardID,
    events::{actions::PlaceOnBoardAction, event::Event, event_handler::EventHandler},
    player::PlayerID,
    tile::{CardOnBoard, Tile},
};

#[derive(Debug)]
pub struct Board {
    tiles: HashMap<U16Vec2, Tile>,
}

impl Board {
    pub fn new() -> Self {
        let mut tiles = HashMap::new();

        for x in 0..=64 {
            for y in 0..=64 {
                let position = U16Vec2::new(x, y);
                tiles.insert(position, Tile::new());
            }
        }
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

        tile.place(CardOnBoard::new(card, player));
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
