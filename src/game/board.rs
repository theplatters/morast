use std::{collections::HashMap, fmt::Display};

use bevy::{
    math::{I16Vec2, U16Vec2},
    prelude::*,
};
use effect::Effect;
use place_error::BoardError;

use crate::game::{
    board::{
        effect::{EffectDuration, EffectType},
        tile::{AttackOnTile, Occupant, TileBundel},
    },
    card::{
        AttackPattern, BaseMovement, CreatureCard, CurrentAttack, CurrentMovementPoints,
        MovementPattern, OnBoard,
    },
    components::{Health, Owner},
    events::{CardMoved, EffectAdded, EffectRemoved},
    player::{Player, TurnPlayer},
};

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

    pub fn get_tile(&self, pos: &U16Vec2) -> Option<Entity> {
        self.tiles.get(pos).copied()
    }
}

pub(crate) fn refresh_movement_points(
    creatures: Query<(&mut CurrentMovementPoints, &Owner, &BaseMovement), With<CreatureCard>>,
    player: Query<&Player, With<TurnPlayer>>,
) {
    for (mut movement, owner, base_movement) in creatures.into_iter() {
        if owner.0 == *player.single().expect("No turn player found") {
            movement.remaining_points = base_movement.0
        }
    }
}

pub fn update_attack_values_on_add(
    _event: On<Replace, OnBoard>,
    tiles: Query<&mut AttackOnTile>,
    creatures: Query<(&CurrentAttack, &AttackPattern, &OnBoard, &Owner), With<CreatureCard>>,
    board: Res<Board>,
) {
    update_attack_values(tiles, creatures, board);
}

pub fn update_attack_values_on_move(
    _event: On<Add, OnBoard>,
    tiles: Query<&mut AttackOnTile>,
    creatures: Query<(&CurrentAttack, &AttackPattern, &OnBoard, &Owner), With<CreatureCard>>,
    board: Res<Board>,
) {
    update_attack_values(tiles, creatures, board);
}

pub fn update_attack_values(
    mut tiles: Query<&mut AttackOnTile>,
    creatures: Query<(&CurrentAttack, &AttackPattern, &OnBoard, &Owner), With<CreatureCard>>,
    board: Res<Board>,
) -> Result<(), BoardError> {
    Board::zero_out_attack(&mut tiles);
    for (attack, pattern, on_board, owner) in &creatures {
        for relative_tile in pattern {
            if let Some(tile_index) = board.add_relative_tile(on_board.position, *relative_tile) {
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

#[derive(Message)]
pub struct EffectRequested {
    effect: Effect,
    indices: Vec<U16Vec2>,
}

fn add_effect_to_tile(
    mut commands: Commands,
    board: Res<Board>,
    mut effects: MessageReader<EffectRequested>,
    mut effects_added: MessageWriter<EffectAdded>,
) -> Result<(), BoardError> {
    for effect_play_event in effects.read() {
        for index in &effect_play_event.indices {
            let tile = board.get_tile(&index).ok_or(BoardError::Index)?;
            commands.spawn((ChildOf(tile), effect_play_event.effect));
            effects_added.write(EffectAdded {
                effect: effect_play_event.effect,
                tile,
            });
        }
    }
    Ok(())
}

#[derive(Message)]
pub struct MoveRequest {
    pub entity: Entity,
    pub from: U16Vec2,
    pub to: U16Vec2,
}

pub fn is_validate_move() -> bool {
    true
}

#[derive(Debug)]
pub enum MoveValidationError {
    InsufficientMovementPoints { required: u16, available: u16 },
    InvalidMovePattern { from: U16Vec2, to: U16Vec2 },
    Occupied,
}

impl Display for MoveValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl std::error::Error for MoveValidationError {}

fn check_valid_move_and_get_cost(
    from: U16Vec2,
    to: U16Vec2,
    movement_points: u16,
    movement_pattern: &MovementPattern,
    tile_has_slow: bool,
) -> Result<u16, MoveValidationError> {
    // Check movement points requirement
    let required_points = if tile_has_slow { 2 } else { 1 };

    if movement_points < required_points {
        return Err(MoveValidationError::InsufficientMovementPoints {
            required: required_points,
            available: movement_points,
        });
    }

    let from_signed = from.as_i16vec2();
    let to_signed = to.as_i16vec2();
    let delta = to_signed - from_signed;

    let pattern_valid = movement_pattern.0.iter().any(|&offset| offset == delta);

    if !pattern_valid {
        return Err(MoveValidationError::InvalidMovePattern { from, to });
    }
    Ok(required_points)
}

pub fn handle_movement(
    mut commands: Commands,
    mut move_requests: MessageReader<MoveRequest>,
    mut move_completed: MessageWriter<CardMoved>,
    mut creatures: Query<
        (&mut OnBoard, &mut CurrentMovementPoints, &MovementPattern),
        With<CreatureCard>,
    >,
    board: Res<Board>,
    occupied: Query<&Occupant>,
    effects: Query<(&EffectType, &ChildOf)>,
) -> Result<(), BoardError> {
    for event in move_requests.read() {
        let (mut creature, mut movement, pattern) = creatures
            .get_mut(event.entity)
            .map_err(|_| BoardError::CardNotFound)?;
        let old_pos = event.from;

        let old_tile = board.get_tile(&old_pos).ok_or(BoardError::TileNotFound)?;
        let new_tile = board.get_tile(&event.to).ok_or(BoardError::TileNotFound)?;

        if occupied.contains(new_tile) {
            return Err(BoardError::InvalidMove(MoveValidationError::Occupied));
        }

        let tile_has_slow = effects
            .iter()
            .any(|(ef, co)| co.0 == old_tile && *ef == EffectType::Slow);

        let cost = check_valid_move_and_get_cost(
            old_pos,
            event.to,
            movement.remaining_points,
            pattern,
            tile_has_slow,
        )
        .map_err(BoardError::InvalidMove)?;

        creature.position = event.to;
        movement.remaining_points -= cost;

        commands.entity(old_tile).remove::<Occupant>();
        commands.entity(new_tile).insert(Occupant(event.entity));

        move_completed.write(CardMoved {
            card: event.entity,
            from: old_pos,
            to: event.to,
        });
    }

    Ok(())
}

pub fn decrease_effect_duration(
    mut commands: Commands,
    mut effects: Query<(&mut EffectDuration, &EffectType, Entity, &ChildOf), With<EffectType>>,
    mut effect_removed: MessageWriter<EffectRemoved>,
) {
    for (mut duration, effect_type, effect_entity, tile) in effects.iter_mut() {
        duration.decrease();
        if duration.over() {
            commands.entity(effect_entity).despawn();
            effect_removed.write(EffectRemoved {
                effect: *effect_type,
                tile: tile.0,
            });
        }
    }
}
