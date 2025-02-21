use std::collections::HashMap;

use card_on_board::CardOnBoard;
use effect::Effect;
use macroquad::{
    math::{I16Vec2, U16Vec2},
    Error,
};
use place_error::BoardError;
use tile::{Tile, TileState};

use super::{
    card::{
        card_id::CardID,
        card_registry::{self, CardRegistry},
        Card,
    },
    player::PlayerID,
};

pub mod card_on_board;
pub mod effect;
pub mod place_error;
mod tile;

#[derive(Debug)]
pub struct Board {
    tiles: HashMap<I16Vec2, Tile>,
    next_id: i32,
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
        Self { tiles, next_id: 0 }
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
    ) -> Result<i32, BoardError> {
        let Some(tile) = self.tiles.get_mut(&index) else {
            return Err(BoardError::Index);
        };

        if tile.is_occupied() {
            return Err(BoardError::TileOccupied);
        }

        tile.place(CardOnBoard::new(self.next_id, card_id, player_id));
        self.next_id += 1;
        self.update_attack_values(card_registry);
        Ok(self.next_id - 1)
    }

    pub fn add_effect(&mut self, effect: Effect, index: I16Vec2) -> Result<(), BoardError> {
        let tile = self.tiles.get_mut(&index).ok_or(BoardError::Index)?;
        tile.add_effect(effect);
        Ok(())
    }

    pub fn remove_effect(&mut self, effect: Effect, index: I16Vec2) -> Result<(), BoardError> {
        let tile = self.tiles.get_mut(&index).ok_or(BoardError::Index)?;
        tile.remove_effect(effect);
        Ok(())
    }

    pub(crate) fn add_effects(
        &mut self,
        effect: Effect,
        tiles: &[I16Vec2],
    ) -> Result<(), BoardError> {
        for tile in tiles {
            self.tiles
                .get_mut(tile)
                .ok_or(BoardError::Index)?
                .add_effect(effect);
        }
        Ok(())
    }

    pub(crate) fn remove_effects(
        &mut self,
        effect: Effect,
        tiles: &[I16Vec2],
    ) -> Result<(), BoardError> {
        for tile in tiles {
            self.tiles
                .get_mut(tile)
                .ok_or(BoardError::Index)?
                .remove_effect(effect);
        }
        Ok(())
    }

    pub fn draw(&self) {
        const TILE_SIZE: f32 = 32.0;

        for x in 0i16..=64 {
            for y in 0i16..=64 {
                let pos = I16Vec2::new(x, y);
                let tile = self.tiles.get(&pos).unwrap();

                // Determine tile color
                let color = if !tile.has_effects() {
                    macroquad::color::GREEN
                } else {
                    macroquad::color::WHITE
                };

                // Calculate screen position
                let screen_x = x as f32 * TILE_SIZE;
                let screen_y = y as f32 * TILE_SIZE;

                // Draw tile background
                macroquad::shapes::draw_rectangle(screen_x, screen_y, TILE_SIZE, TILE_SIZE, color);

                // Draw X if occupied
                if let TileState::Card(_) = &tile.ontile {
                    let thickness = 2.0;
                    let padding = 4.0;

                    // Draw two crossing lines for X
                    macroquad::shapes::draw_line(
                        screen_x + padding,
                        screen_y + padding,
                        screen_x + TILE_SIZE - padding,
                        screen_y + TILE_SIZE - padding,
                        thickness,
                        macroquad::color::BLACK,
                    );
                    macroquad::shapes::draw_line(
                        screen_x + padding,
                        screen_y + TILE_SIZE - padding,
                        screen_x + TILE_SIZE - padding,
                        screen_y + padding,
                        thickness,
                        macroquad::color::BLACK,
                    );
                }
            }
        }
    }

    fn is_legal_move(
        &self,
        card_id: &CardID,
        from: I16Vec2,
        to: I16Vec2,
        card_registry: &CardRegistry,
    ) -> bool {
        let movement = &card_registry.get(card_id).unwrap().movement;
        movement.contains(&(from - to))
    }

    pub fn move_card(
        &mut self,
        from: I16Vec2,
        to: I16Vec2,
        card_registry: &CardRegistry,
    ) -> Result<CardOnBoard, BoardError> {
        // Check if 'from' and 'to' are valid
        let from_tile = self.tiles.get(&from).ok_or(BoardError::Index)?;
        let card = match &from_tile.ontile {
            TileState::Card(c) => *c,
            _ => return Err(BoardError::TileEmpty),
        };
        if !self.is_legal_move(&card.card_id, from, to, card_registry) {
            return Err(BoardError::InvalidMove);
        }
        let to_tile = self.tiles.get(&to).ok_or(BoardError::Index)?;

        // Check if 'to' tile is valid
        if to_tile.is_occupied() {
            return Err(BoardError::TileOccupied);
        }

        // Move the card
        let from_tile = self.tiles.get_mut(&from).unwrap();
        from_tile.ontile = TileState::Empty;

        let to_tile = self.tiles.get_mut(&to).unwrap();
        to_tile.ontile = TileState::Card(card);

        self.update_attack_values(card_registry);
        Ok(card)
    }
}
