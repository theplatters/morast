use std::collections::{HashMap, HashSet};

use bevy::prelude::*;
use card_on_board::CreatureOnBoard;
use effect::Effect;
use macroquad::math::{I16Vec2, U16Vec2};
use place_error::BoardError;
use tile::Tile;

use crate::game::{
    board::{
        effect::EffectType,
        tile::{AttackOnTile, Occupant, TileBundel},
    },
    card::CreatureCard,
    components::{
        card_components::{AttackPattern, CurrentAttack, OnBoard},
        player_components::Player,
        Health, Owner,
    },
    error::Error,
    events::event::{CardMoved, CreaturePlayed},
};

use super::{
    card::{card_registry::CardRegistry, in_play_id::InPlayID},
    player::PlayerID,
};

pub mod card_on_board;
pub mod effect;
pub mod place_error;
pub mod tile;

#[derive(Bundle)]
pub struct PlayerBaseBundle {
    player_base: PlayerBase,
    health: Health,
}

impl PlayerBaseBundle {
    fn new() -> Self {
        Self {
            player_base: PlayerBase::default(),
            health: Health::player_base_health(),
        }
    }
}

#[derive(Component, Default)]
pub struct PlayerBase;

#[derive(Debug, Resource)]
pub struct Board {
    tiles: HashMap<U16Vec2, Entity>,
    size: U16Vec2,
    player_base_positions: [U16Vec2; 2],
}

impl Board {
    pub fn setup_board(mut commands: Commands) {
        let x_size: u16 = 24;
        let y_size: u16 = 12;

        let mut tiles = HashMap::new();
        let player_base_positions = [
            U16Vec2::new(2, y_size / 2),
            U16Vec2::new(x_size - 3, y_size / 2),
        ];

        for x in 0..x_size {
            for y in 0..y_size {
                let position = U16Vec2::new(x, y);
                let tile_id = commands.spawn(TileBundel::default()).id();
                tiles.insert(position, tile_id);
            }
        }

        commands.insert_resource(Board {
            tiles,
            size: U16Vec2::new(x_size, y_size),
            player_base_positions,
        });
    }

    pub fn setup_player_bases(mut commands: Commands, board: Res<Board>, players: Query<&Player>) {
        for (player_entity, pos) in players
            .iter()
            .zip(board.player_base_positions.into_iter())
            .into_iter()
        {
            let base_entity = commands
                .spawn((PlayerBaseBundle::new(), Owner(*player_entity)))
                .id();
            let tile = board
                .get_tile(&pos)
                .ok_or(BoardError::TileNotFound)
                .expect("This is a setup error and should never happen");
            commands.entity(tile).insert(Occupant(base_entity));
        }
    }

    fn zero_out_attack(tiles: &mut Query<&mut AttackOnTile>) {
        for mut tile in tiles.iter_mut() {
            tile.zero_out();
        }
    }

    pub fn update_attack_values_on_add(
        _event: On<Replace, OnBoard>,
        tiles: Query<&mut AttackOnTile>,
        creatures: Query<(&CurrentAttack, &AttackPattern, &OnBoard, &Owner), With<CreatureCard>>,
        board: Res<Board>,
    ) {
        Self::update_attack_values(tiles, creatures, board);
    }

    pub fn update_attack_values_on_move(
        _event: On<Add, OnBoard>,
        tiles: Query<&mut AttackOnTile>,
        creatures: Query<(&CurrentAttack, &AttackPattern, &OnBoard, &Owner), With<CreatureCard>>,
        board: Res<Board>,
    ) {
        Self::update_attack_values(tiles, creatures, board);
    }

    pub fn update_attack_values(
        mut tiles: Query<&mut AttackOnTile>,
        creatures: Query<(&CurrentAttack, &AttackPattern, &OnBoard, &Owner), With<CreatureCard>>,
        board: Res<Board>,
    ) -> Result<(), BoardError> {
        Board::zero_out_attack(&mut tiles);
        for (attack, pattern, on_board, owner) in &creatures {
            for relative_tile in pattern {
                if let Some(tile_index) = board.add_relative_tile(on_board.position, *relative_tile)
                {
                    let tile = board
                        .get_tile(&tile_index)
                        .ok_or(BoardError::TileNotFound)?;

                    let mut tile = tiles.get_mut(tile).unwrap();
                    let attack_delta = match owner.0.number {
                        1 => U16Vec2::new(0, attack.value),
                        2 => U16Vec2::new(attack.value, 0),
                        _ => panic!("Invalid player number: {}", owner.0.number),
                    };
                    **tile += attack_delta;
                }
            }
        }
        Ok(())
    }

    pub fn add_relative_tile(&self, pos: U16Vec2, reltile: I16Vec2) -> Option<U16Vec2> {
        let new_x = pos.x.checked_add_signed(reltile.x as i16)?;
        let new_y = pos.y.checked_add_signed(reltile.y as i16)?;

        let new_pos = U16Vec2::new(new_x, new_y);

        if new_pos.x < self.size.x && new_pos.y < self.size.y {
            Some(new_pos)
        } else {
            None
        }
    }

    pub fn width(&self) -> u16 {
        self.size.x
    }

    pub fn height(&self) -> u16 {
        self.size.y
    }

    pub fn add_effect(
        mut commands: Commands,
        effect: Effect,
        index: U16Vec2,
        board: Res<Board>,
    ) -> Result<(), BoardError> {
        let tile = board.get_tile(&index).ok_or(BoardError::Index)?;
        commands
            .get_entity(tile)
            .map_err(|_| BoardError::TileNotFound)?
            .insert(effect);
        Ok(())
    }

    pub fn remove_effect(&mut self, effect: EffectType, index: I16Vec2) -> Result<(), BoardError> {
        let tile = self.tiles.get_mut(&index).ok_or(BoardError::Index)?;
        tile.remove_effect(effect);
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

    pub fn get_tile(&self, pos: &U16Vec2) -> Option<Entity> {
        self.tiles.get(pos).copied()
    }

    pub(crate) fn refresh_movement_points(
        &mut self,
        player_id: PlayerID,
        card_registry: &CardRegistry,
    ) -> Result<(), Error> {
        for (_, card_on_board) in self.cards_placed.iter_mut() {
            if card_on_board.player_id == player_id {
                let card = card_registry
                    .get_creature(&card_on_board.card_id)
                    .ok_or(Error::CardNotFound)?;
                card_on_board.movement_points = card.movement_points;
            }
        }
        Ok(())
    }

    // Helper method to check if a card can move
    pub fn can_card_move(&self, move_player: PlayerID, from: &I16Vec2) -> bool {
        let from_tile = self.tiles.get(from).unwrap();
        if let Some(card_on_board) = self.get_card_at_index(from) {
            card_on_board.movement_points > 0
                || (from_tile.has_effect(move_player, effect::EffectType::Slow)
                    && card_on_board.movement_points > 1)
        } else {
            false
        }
    }

    pub fn move_card(
        &mut self,
        from: &I16Vec2,
        to: &I16Vec2,
        move_player: PlayerID,
    ) -> Result<(), BoardError> {
        print!("Tiles {:?}", self.tiles);
        // Check if 'from' and 'to' are valid
        let from_tile = self.tiles.get(from).ok_or(BoardError::Index)?;
        let card_id = match &from_tile.ontile {
            Some(c) => *c,
            _ => return Err(BoardError::TileEmpty(*from)),
        };

        // Check if the card has movement points left
        if !self.can_card_move(move_player.next(), from) {
            return Err(BoardError::NoMovementPoints);
        }

        let to_tile = self.tiles.get(to).ok_or(BoardError::Index)?;
        let card_on_board = self
            .cards_placed
            .get_mut(&card_id)
            .ok_or(BoardError::CardNotFound)?;

        // Check if 'to' tile is valid
        if to_tile.is_occupied() {
            return Err(BoardError::TileOccupied);
        }

        if from_tile.has_effect(move_player.next(), effect::EffectType::Slow) {
            card_on_board.movement_points -= 2;
        } else {
            card_on_board.movement_points -= 1;
        }
        // Move the card
        let from_tile = self.tiles.get_mut(from).unwrap();
        from_tile.ontile = None;

        let to_tile = self.tiles.get_mut(to).unwrap();
        to_tile.ontile = Some(card_id);
        Ok(())
    }

    pub(crate) fn update_effects(&mut self, turn_player: PlayerID) {
        for (_, tile) in self.tiles.iter_mut() {
            tile.process_effects(turn_player);
        }
    }

    pub(crate) fn get_card_on_tile(&self, pos: &I16Vec2) -> Result<&CreatureOnBoard, Error> {
        let card_id = self
            .tiles
            .get(pos)
            .and_then(|tile| tile.ontile)
            .ok_or(Error::PlaceError(BoardError::TileEmpty(*pos)))?;

        self.cards_placed.get(&card_id).ok_or(Error::CardNotFound)
    }

    pub(crate) fn get_card_index(&self, searched_for_id: InPlayID) -> Option<I16Vec2> {
        self.tiles
            .iter()
            .find_map(|(pos, tile)| (tile.ontile == Some(searched_for_id)).then_some(*pos))
    }

    pub(crate) fn player_base_take_damage(&mut self) -> [PlayerBaseStatus; 2] {
        let mut results = [PlayerBaseStatus::default(); 2]; // Assuming Default is implemented

        for (i, &pos) in self.player_bases_positions.iter().enumerate() {
            let tile = self
                .tiles
                .get_mut(&pos)
                .expect("Fatal error: Tile not found");
            let attack = if i == 1 {
                tile.attack_on_tile.x
            } else {
                tile.attack_on_tile.y
            };
            results[i] = tile
                .get_player_base_mut()
                .expect("Fatal base: Player base is not where it should be")
                .damage(attack);
        }

        results
    }

    pub(crate) fn get_card_owner(&self, card_id: &InPlayID) -> Option<PlayerID> {
        self.cards_placed
            .get(card_id)
            .map(|card_on_board| card_on_board.player_id)
    }

    pub(crate) fn do_damage(&self, tile: &I16Vec2, amount: u16) {
        todo!()
    }

    pub(crate) fn heal_creature(&self, tile: &I16Vec2, amount: u16) {
        todo!()
    }

    pub(crate) fn destroy_card(&self, tile: &I16Vec2) {
        todo!()
    }
}

// Define an extension trait
pub trait BoardCommandsExt {
    fn add_effect_to_tile(
        &mut self,
        board: &Board,
        effect: Effect,
        index: U16Vec2,
    ) -> Result<(), BoardError>;

    fn remove_effect(
        &mut self,
        board: &Board,
        effect: EffectType,
        index: U16Vec2,
    ) -> Result<(), BoardError>;
}

impl BoardCommandsExt for Commands<'_, '_> {
    fn add_effect_to_tile(
        &mut self,
        board: &Board,
        effect: Effect,
        index: U16Vec2,
    ) -> Result<(), BoardError> {
        let tile = board.get_tile(&index).ok_or(BoardError::Index)?;
        self.get_entity(tile)
            .map_err(|_| BoardError::TileNotFound)?
            .insert(effect);
        Ok(())
    }

    fn remove_effect(
        &mut self,
        board: &Board,
        effect: EffectType,
        index: U16Vec2,
    ) -> Result<(), BoardError> {
        let tile = board.get_tile(&index).ok_or(BoardError::Index)?;

        self.get_entity(tile)
            .map_err(|_| BoardError::TileNotFound)?
            .remove::<Effect>();
        Ok(())
    }
}
