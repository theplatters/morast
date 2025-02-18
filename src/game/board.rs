use std::{collections::HashMap, io::Empty};

use card_on_board::CardOnBoard;
use macroquad::math::U16Vec2;
use place_error::PlaceError;
use tile::{Tile, TileState};

use super::{
    card::{
        card_id::CardID,
        card_registry::{self, CardRegistry},
    },
    player::PlayerID,
};

pub mod card_on_board;
mod effect;
pub mod place_error;
mod tile;

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

    fn update_attack_values(&mut self, card_registry: CardRegistry) {
        for (index, tile) in self.tiles.iter() {
            match tile.ontile {
                TileState::Empty => continue,
                TileState::Card(card) => {
                    let card = card_registry
                        .get(&card.card_id)
                        .unwrap_or_else(|| panic!("Card not found {:?}", card.card_id));
                }
            }
        }
    }

    pub fn place(
        &mut self,
        card_id: CardID,
        player_id: PlayerID,
        index: U16Vec2,
    ) -> Result<(), PlaceError> {
        let Some(tile) = self.tiles.get_mut(&index) else {
            return Err(PlaceError::IndexError);
        };

        if tile.is_occupied() {
            return Err(PlaceError::TileOccupiedError);
        }

        tile.place(CardOnBoard::new(card_id, player_id));

        Ok(())
    }
}
