use std::collections::HashMap;

use card_on_board::CardOnBoard;
use effect::Effect;
use macroquad::{
    math::{I16Vec2, U16Vec2},
    text::draw_text,
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

    pub(crate) fn update_attack_values(&mut self, card_registry: &CardRegistry) {
        self.zero_out_attack();

        // Estimate a capacity: many tiles may be empty so a small capacity helps avoid re-allocations.
        // Adjust the capacity based on typical board density and attack pattern size.
        let mut updates = Vec::with_capacity(16);

        for (index, curr_tile) in self.tiles.iter() {
            if let TileState::Card(card_on_board) = curr_tile.ontile {
                let card = card_registry
                    .get(&card_on_board.card_id)
                    .unwrap_or_else(|| panic!("Card not found {:?}", card_on_board.card_id));

                // Calculate the attack vector once per card.
                let attack_vector = if card_on_board.player_id == PlayerID::new(0) {
                    U16Vec2::X * card.attack_strength
                } else {
                    U16Vec2::Y * card.attack_strength
                };

                // Use the card's attack pattern to queue updates.
                for attack in card.get_attack_pattern() {
                    let target_index = index.wrapping_add(*attack);
                    updates.push((target_index, attack_vector));
                }
            }
        }

        // Apply all updates without incurring a second borrow of self.tiles during iteration.
        for (target_index, attack_vector) in updates {
            if let Some(tile) = self.tiles.get_mut(&target_index) {
                tile.attack_on_tile += attack_vector;
            }
        }
    }

    pub(crate) fn update_attack_values_for_card(
        &mut self,
        card: CardOnBoard,
        old_pos: I16Vec2,
        new_pos: I16Vec2,
        card_registry: &CardRegistry,
    ) {
        let card_info = card_registry
            .get(&card.card_id)
            .unwrap_or_else(|| panic!("Card not found {:?}", card.card_id));

        // Compute the card's attack vector.
        let attack_vector = if card.player_id == PlayerID::new(0) {
            U16Vec2::X * card_info.attack_strength
        } else {
            U16Vec2::Y * card_info.attack_strength
        };

        // Remove attack contributions from the old position.
        for attack in card_info.get_attack_pattern() {
            let old_target = old_pos.wrapping_add(*attack);
            if let Some(tile) = self.tiles.get_mut(&old_target) {
                // Use saturating_sub to ensure no underflow.
                tile.attack_on_tile = tile.attack_on_tile.saturating_sub(attack_vector);
            }
        }

        // Add attack contributions from the new position.
        for attack in card_info.get_attack_pattern() {
            let new_target = new_pos.wrapping_add(*attack);
            if let Some(tile) = self.tiles.get_mut(&new_target) {
                tile.attack_on_tile += attack_vector;
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

                // Draw attack values
                let attack_x = tile.attack_on_tile.x as f32;
                let attack_y = tile.attack_on_tile.y as f32;
                let attack_text = format!("{:.0}, {:.0}", attack_x, attack_y);
                draw_text(
                    &attack_text,
                    screen_x + 8.0,
                    screen_y + 20.0,
                    14.0,
                    macroquad::color::BLACK,
                );

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

    pub fn move_card(&mut self, from: I16Vec2, to: I16Vec2) -> Result<CardOnBoard, BoardError> {
        // Check if 'from' and 'to' are valid
        let from_tile = self.tiles.get(&from).ok_or(BoardError::Index)?;
        let card = match &from_tile.ontile {
            TileState::Card(c) => *c,
            _ => return Err(BoardError::TileEmpty),
        };
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

        Ok(card)
    }
}
