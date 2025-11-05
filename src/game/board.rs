use std::collections::{HashMap, HashSet};

use card_on_board::CardOnBoard;
use effect::Effect;
use macroquad::{
    math::{I16Vec2, U16Vec2, UVec2},
    text::draw_text,
};
use place_error::BoardError;
use tile::Tile;

use super::{
    card::{card_id::CardID, card_registry::CardRegistry, in_play_id::InPlayID},
    player::PlayerID,
};

pub mod card_on_board;
pub mod effect;
pub mod place_error;
mod tile;

#[derive(Debug)]
pub struct Board {
    tiles: HashMap<I16Vec2, Tile>,
    next_id: InPlayID,
    board_size: I16Vec2,
}

impl Board {
    pub fn new() -> Self {
        let mut tiles = HashMap::new();
        let x_size: i16 = 12;
        let y_size: i16 = 12;
        for x in 0..=x_size {
            for y in 0..=y_size {
                let position = I16Vec2::new(x, y);
                tiles.insert(position, Tile::new());
            }
        }
        Self {
            tiles,
            next_id: InPlayID::new(0),
            board_size: I16Vec2::new(x_size, y_size),
        }
    }

    fn zero_out_attack(&mut self) {
        self.tiles
            .values_mut()
            .for_each(|tile| tile.attack_on_tile = U16Vec2::ZERO);
    }

    pub(crate) fn update_attack_values(
        &mut self,
        card_registry: &CardRegistry,
    ) -> HashSet<CardOnBoard> {
        self.zero_out_attack();

        // First collect all attack contributions
        let mut attack_updates = HashMap::new();

        // Immutable phase - collect data
        for (index, curr_tile) in self.tiles.iter() {
            if let Some(card_on_board) = &curr_tile.ontile {
                let card = card_registry
                    .get(&card_on_board.card_id)
                    .unwrap_or_else(|| panic!("Card not found {:?}", card_on_board.card_id));

                let attack_vector = match card_on_board.player_id {
                    PlayerID(0) => U16Vec2::X * card.attack_strength,
                    _ => U16Vec2::Y * card.attack_strength,
                };

                for attack in card.get_attack_pattern() {
                    let target_index = index.wrapping_add(*attack);
                    attack_updates
                        .entry(target_index)
                        .and_modify(|v| *v += attack_vector)
                        .or_insert(attack_vector);
                }
            }
        }

        // Mutable phase - apply updates
        for (index, attack) in attack_updates {
            if let Some(tile) = self.tiles.get_mut(&index) {
                tile.attack_on_tile += attack;
            }
        }

        // Now handle card removal in a separate pass
        let mut removed = HashSet::new();
        for tile in self.tiles.values_mut() {
            if let Some(card) = &tile.ontile {
                let defense = card_registry
                    .get(&card.card_id)
                    .expect("Card not found in registry")
                    .defense;

                let attacked_idx = card.player_id.next().get() as usize;
                if defense < tile.attack_on_tile[attacked_idx] {
                    removed.insert(*card);
                    tile.ontile = None;
                }
            }
        }

        removed
    }

    pub(crate) fn update_attack_values_for_card(
        &mut self,
        attacking_card: CardOnBoard,
        old_pos: I16Vec2,
        new_pos: I16Vec2,
        card_registry: &CardRegistry,
    ) -> HashSet<CardOnBoard> {
        let card_info = card_registry
            .get(&attacking_card.card_id)
            .unwrap_or_else(|| panic!("Card not found {:?}", attacking_card.card_id));

        let attack_vector = match attacking_card.player_id {
            PlayerID(0) => U16Vec2::X * card_info.attack_strength,
            _ => U16Vec2::Y * card_info.attack_strength,
        };

        // Remove attack contributions from the old position.
        for attack in card_info.get_attack_pattern() {
            let old_target = old_pos.wrapping_add(*attack);
            if let Some(tile) = self.tiles.get_mut(&old_target) {
                // Use saturating_sub to ensure no underflow.
                tile.attack_on_tile = tile.attack_on_tile.saturating_sub(attack_vector);
            }
        }
        let mut removed = HashSet::new();
        // Add attack contributions from the new position.
        for attack in card_info.get_attack_pattern() {
            let new_target = new_pos.wrapping_add(*attack);
            if let Some(tile) = self.tiles.get_mut(&new_target) {
                tile.attack_on_tile += attack_vector;
                match tile.ontile {
                    Some(attacked_card) if attacking_card.player_id != attacked_card.player_id => {
                        let attacked_card_health = card_registry
                            .get(&attacked_card.card_id)
                            .expect("Fatal: Card not found in card_registry")
                            .defense;
                        if attacked_card_health
                            < tile.attack_on_tile[attacking_card.player_id.get() as usize]
                        {
                            removed.insert(attacked_card);
                            tile.ontile = None;
                        }
                    }
                    _ => {}
                }
            } // Calculate attack vector once
        }
        removed
    }

    pub fn place(
        &mut self,
        card_id: CardID,
        player_id: PlayerID,
        index: I16Vec2,
    ) -> Result<InPlayID, BoardError> {
        let Some(tile) = self.tiles.get_mut(&index) else {
            return Err(BoardError::Index);
        };

        if tile.is_occupied() {
            return Err(BoardError::TileOccupied);
        }

        tile.place(CardOnBoard::new(self.next_id, card_id, player_id));
        let id = self.next_id;
        self.next_id = self.next_id.next();
        Ok(id)
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
        const TILE_SIZE: f32 = 64.0;

        for x in 0i16..=self.board_size.x {
            for y in 0i16..=self.board_size.y {
                let pos = I16Vec2::new(x, y);
                let tile = self.tiles.get(&pos).unwrap();

                // Determine tile color
                let color = if tile.has_effects() {
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
                let attack_x = tile.attack_on_tile.x;
                let attack_y = tile.attack_on_tile.y;
                let attack_text = format!("{:}, {:}", attack_x, attack_y);
                if attack_x != 0 || attack_y != 0 {
                    draw_text(
                        &attack_text,
                        screen_x + 8.0,
                        screen_y + 20.0,
                        14.0,
                        macroquad::color::BLACK,
                    );
                }

                // Draw X if occupied
                if tile.ontile.is_some() {
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
            Some(c) => *c,
            _ => return Err(BoardError::TileEmpty),
        };
        let to_tile = self.tiles.get(&to).ok_or(BoardError::Index)?;

        // Check if 'to' tile is valid
        if to_tile.is_occupied() {
            return Err(BoardError::TileOccupied);
        }

        // Move the card
        let from_tile = self.tiles.get_mut(&from).unwrap();
        from_tile.ontile = None;

        let to_tile = self.tiles.get_mut(&to).unwrap();
        to_tile.ontile = Some(card);

        Ok(card)
    }

    pub(crate) fn update_effects(&mut self) {
        for (_, tile) in self.tiles.iter_mut() {
            tile.process_effects();
        }
    }
}
