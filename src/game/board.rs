use std::collections::{HashMap, HashSet};

use card_on_board::CreatureOnBoard;
use effect::Effect;
use macroquad::math::{I16Vec2, U16Vec2};
use place_error::BoardError;
use tile::Tile;

use crate::game::{
    card::creature::Creature,
    error::Error,
    game_objects::player_base::{PlayerBase, PlayerBaseStatus},
};

use super::{
    card::{card_registry::CardRegistry, in_play_id::InPlayID},
    player::PlayerID,
};

pub mod card_on_board;
pub mod effect;
pub mod place_error;
pub mod tile;

#[derive(Debug)]
pub struct Board {
    tiles: HashMap<I16Vec2, Tile>,
    next_id: InPlayID,
    board_size: I16Vec2,
    pub cards_placed: HashMap<InPlayID, CreatureOnBoard>,
    player_bases_positions: [I16Vec2; 2],
}

impl Board {
    pub fn new() -> Self {
        let mut tiles = HashMap::new();
        let x_size: i16 = 24;
        let y_size: i16 = 12;
        for x in 0..x_size {
            for y in 0..y_size {
                let position = I16Vec2::new(x, y);

                let tile = if x == 2 && y == y_size / 2 {
                    Tile::new().with_player_base(PlayerBase::new(PlayerID(0)))
                } else if x == x_size - 3 && y == y_size / 2 {
                    Tile::new().with_player_base(PlayerBase::new(PlayerID(1)))
                } else {
                    Tile::new()
                };

                tiles.insert(position, tile);
            }
        }
        Self {
            tiles,
            next_id: InPlayID::new(0),
            board_size: I16Vec2::new(x_size, y_size),
            cards_placed: HashMap::new(),
            player_bases_positions: [
                I16Vec2::new(2, y_size / 2),
                I16Vec2::new(x_size - 3, y_size / 2),
            ],
        }
    }

    fn zero_out_attack(&mut self) {
        self.tiles
            .values_mut()
            .for_each(|tile| tile.attack_on_tile = U16Vec2::ZERO);
    }

    fn get_attack_vector(
        &self,
        card_on_board: &CreatureOnBoard,
        tile: &Tile,
        card: &Creature,
    ) -> Result<U16Vec2, Error> {
        let effective_attack = self.calculate_effective_attack_strength(
            card.attack_strength,
            card_on_board.player_id,
            tile,
        );

        let attack_vector = match card_on_board.player_id {
            PlayerID(0) => U16Vec2::X,
            _ => U16Vec2::Y,
        };

        Ok(attack_vector * effective_attack)
    }

    fn calculate_effective_attack_strength(
        &self,
        base_attack: u16,
        player_id: PlayerID,
        tile: &Tile,
    ) -> u16 {
        let opponent = player_id.next();

        if tile.has_effect(opponent, effect::EffectType::Weakening) {
            base_attack / 2
        } else {
            base_attack
        }
    }

    fn get_attack_updates(
        &self,
        card_registry: &CardRegistry,
    ) -> Result<HashMap<I16Vec2, U16Vec2>, Error> {
        // First collect all attack contributions
        let mut attack_updates = HashMap::new();

        // Immutable phase - collect data
        for (index, curr_tile) in self.tiles.iter() {
            if let Some(in_play_id) = &curr_tile.ontile {
                let card_on_board = self
                    .cards_placed
                    .get(in_play_id)
                    .ok_or(Error::CardNotFound)?;
                let card = card_registry
                    .get_creature(&card_on_board.card_id)
                    .ok_or(Error::CardNotFound)?;

                let attack_vector = self.get_attack_vector(card_on_board, curr_tile, card)?;
                for attack in &card.attack {
                    let adjusted_attack = if card_on_board.player_id == PlayerID(1) {
                        I16Vec2::new(-attack.x, attack.y)
                    } else {
                        *attack
                    };
                    let target_index = index.wrapping_add(adjusted_attack);
                    attack_updates
                        .entry(target_index)
                        .and_modify(|v| *v += attack_vector)
                        .or_insert(attack_vector);
                }
            }
        }
        Ok(attack_updates)
    }

    fn apply_attack_updates(&mut self, attack_updates: &HashMap<I16Vec2, U16Vec2>) {
        for (index, attack) in attack_updates {
            if let Some(tile) = self.tiles.get_mut(index) {
                tile.attack_on_tile += *attack;
            }
        }
    }

    pub(crate) fn update_attack_values(
        &mut self,
        card_registry: &CardRegistry,
    ) -> Result<HashSet<InPlayID>, Error> {
        self.zero_out_attack();

        //Get new attack updates
        let attack_updates = self.get_attack_updates(card_registry)?;

        // Mutable phase - apply updates
        self.apply_attack_updates(&attack_updates);

        // Now handle card removal in a separate pass
        let mut removed = HashSet::new();
        for tile in self.tiles.values_mut() {
            if let Some(in_play_id) = &tile.ontile {
                let card = self
                    .cards_placed
                    .get(in_play_id)
                    .ok_or(Error::CardNotFound)?;
                let defense = card_registry
                    .get_creature(&card.card_id)
                    .ok_or(Error::CardNotFound)?
                    .defense;

                let attacked_idx = card.player_id.next().get() as usize;
                if defense < tile.attack_on_tile[attacked_idx] {
                    removed.insert(*in_play_id);
                    tile.ontile = None;
                }
            }
        }

        Ok(removed)
    }

    pub(crate) fn get_card_at_index(&self, from: &I16Vec2) -> Option<&CreatureOnBoard> {
        let card_on_tile = self.get_tile(from)?.ontile?;
        self.cards_placed.get(&card_on_tile)
    }

    pub fn generate_in_play_id(&mut self) -> InPlayID {
        let id = self.next_id;
        self.next_id = self.next_id.next();
        id
    }

    pub fn place(
        &mut self,
        index: I16Vec2,
        card_on_board: CreatureOnBoard,
    ) -> Result<InPlayID, BoardError> {
        let Some(tile) = self.tiles.get_mut(&index) else {
            return Err(BoardError::Index);
        };

        if tile.is_occupied() {
            return Err(BoardError::TileOccupied);
        }

        tile.place(self.next_id);
        let id = self.generate_in_play_id();

        self.cards_placed.insert(id, card_on_board);
        Ok(id)
    }

    pub fn width(&self) -> i16 {
        self.board_size.x
    }

    pub fn height(&self) -> i16 {
        self.board_size.y
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

    pub fn tile_pos_iter(&self) -> impl Iterator<Item = I16Vec2> + '_ {
        self.tiles.keys().copied()
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

    pub fn get_tile(&self, pos: &I16Vec2) -> Option<&Tile> {
        self.tiles.get(pos)
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
        from: I16Vec2,
        to: I16Vec2,
        move_player: PlayerID,
    ) -> Result<(), BoardError> {
        // Check if 'from' and 'to' are valid
        let from_tile = self.tiles.get(&from).ok_or(BoardError::Index)?;
        let card_id = match &from_tile.ontile {
            Some(c) => *c,
            _ => return Err(BoardError::TileEmpty),
        };
        // Check if the card has movement points left
        if !self.can_card_move(move_player.next(), &from) {
            return Err(BoardError::NoMovementPoints);
        }

        let to_tile = self.tiles.get(&to).ok_or(BoardError::Index)?;
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
        let from_tile = self.tiles.get_mut(&from).unwrap();
        from_tile.ontile = None;

        let to_tile = self.tiles.get_mut(&to).unwrap();
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
            .ok_or(Error::TileEmpty)?;

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
}
