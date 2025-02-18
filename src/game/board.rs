use std::{collections::HashMap, io::Empty};

use card_on_board::CardOnBoard;
use macroquad::math::{I16Vec2, U16Vec2};
use place_error::PlaceError;
use tile::{Tile, TileState};

use super::{
    card::{
        card_id::CardID,
        card_registry::{self, CardRegistry},
    },
    player::{Player, PlayerID},
};

pub mod card_on_board;
mod effect;
pub mod place_error;
mod tile;

#[derive(Debug)]
pub struct Board {
    tiles: HashMap<I16Vec2, Tile>,
}

impl Board {
    pub fn new() -> Self {
        let mut tiles = HashMap::new();

        for x in 0..=64 {
            for y in 0..=64 {
                let position = I16Vec2::new(x, y);
                tiles.insert(position, Tile::new());
            }
        }
        Self { tiles }
    }

    fn zero_out_attack(&mut self) {
        for (index, curr_tile) in self.tiles.iter_mut() {
            curr_tile.attack_on_tile = U16Vec2::ZERO;
        }
    }

    fn update_attack_values(&mut self, card_registry: &CardRegistry) {
        self.zero_out_attack();
        for (index, curr_tile) in self.tiles.iter() {
            match curr_tile.ontile {
                TileState::Empty => continue,
                TileState::Card(card_on_board) => {
                    let card = card_registry
                        .get(&card_on_board.card_id)
                        .unwrap_or_else(|| panic!("Card not found {:?}", card_on_board.card_id));

                    for attack in card.get_attack_pattern() {
                        let Some(tile) = self.tiles.get(&index.wrapping_add(*attack)) else {
                            continue;
                        };
                        let attack_vector = if card_on_board.player_id == PlayerID::new(0) {
                            U16Vec2::X * card.attack_strength
                        } else {
                            U16Vec2::Y * card.attack_strength
                        };
                        tile.attack_on_tile.saturating_add(attack_vector);
                    }
                }
            }
        }
    }

    pub fn place(
        &mut self,
        card_id: CardID,
        player_id: PlayerID,
        index: I16Vec2,
        card_registry: &CardRegistry,
    ) -> Result<(), PlaceError> {
        let Some(tile) = self.tiles.get_mut(&index) else {
            return Err(PlaceError::IndexError);
        };

        if tile.is_occupied() {
            return Err(PlaceError::TileOccupiedError);
        }

        tile.place(CardOnBoard::new(card_id, player_id));
        self.update_attack_values(card_registry);
        Ok(())
    }
}
