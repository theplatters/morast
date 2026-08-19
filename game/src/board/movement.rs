use std::fmt::Display;

use crate::{Result, card::creature::BaseMovementPoints, components::Owner, player::TurnPlayer};
use bevy::{
    ecs::{
        entity::Entity,
        hierarchy::ChildOf,
        message::{Message, MessageReader, MessageWriter},
        query::With,
        system::{Commands, Query, Res},
    },
    math::U16Vec2,
};

use crate::{
    board::{BoardRes, effect::EffectType, place_error::BoardError, tile::Occupant},
    card::{CreatureCard, CurrentMovementPoints, OnBoard, creature::MovementPattern},
    events::CardMoved,
};

#[derive(Message)]
pub struct MoveRequest {
    pub entity: Entity,
    pub from: U16Vec2,
    pub to: U16Vec2,
}

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

    let pattern_valid = movement_pattern.0.contains(&delta);

    if !pattern_valid {
        return Err(MoveValidationError::InvalidMovePattern { from, to });
    }
    Ok(required_points)
}

pub fn is_validate_move() -> bool {
    true
}

/// Computes the set of tiles a creature standing on `from` can reach in a single
/// move, matching the validation performed by `handle_movement` (bounds,
/// movement pattern, remaining movement points, and occupancy).
pub fn reachable_tiles(
    board: &BoardRes,
    from: U16Vec2,
    movement_points: u16,
    pattern: &MovementPattern,
    occupied: &[U16Vec2],
    current_tile_has_slow: bool,
) -> Vec<U16Vec2> {
    let required_points = if current_tile_has_slow { 2 } else { 1 };
    if movement_points < required_points {
        return Vec::new();
    }
    let mut reachable = Vec::new();
    for delta in &pattern.0 {
        if let Some(target) = board.add_relative_tile(from, *delta) {
            if !occupied.contains(&target) {
                reachable.push(target);
            }
        }
    }
    reachable
}

#[derive(Debug)]
pub enum MoveValidationError {
    InsufficientMovementPoints { required: u16, available: u16 },
    InvalidMovePattern { from: U16Vec2, to: U16Vec2 },
    Occupied,
}

impl Display for MoveValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MoveValidationError::InsufficientMovementPoints {
                required,
                available,
            } => write!(
                f,
                "InsufficientMovementPoints required: {}, available {}",
                required, available
            ),
            MoveValidationError::InvalidMovePattern { from, to } => {
                write!(f, "InvalidMovePattern from {}, to {}", from, to)
            }
            MoveValidationError::Occupied => write!(f, "Tile is occupied"),
        }
    }
}

impl std::error::Error for MoveValidationError {}

pub fn handle_movement(
    mut commands: Commands,
    mut move_requests: MessageReader<MoveRequest>,
    mut move_completed: MessageWriter<CardMoved>,
    mut creatures: Query<(&mut CurrentMovementPoints, &MovementPattern), With<CreatureCard>>,
    board: Res<BoardRes>,
    occupied: Query<&Occupant>,
    effects: Query<(&EffectType, &ChildOf)>,
) -> Result {
    for event in move_requests.read() {
        let (mut movement, pattern) = creatures
            .get_mut(event.entity)
            .map_err(|_| BoardError::CardNotFound)?;
        let old_pos = event.from;

        let old_tile = board.get_tile(&old_pos).ok_or(BoardError::TileNotFound)?;
        let new_tile = board.get_tile(&event.to).ok_or(BoardError::TileNotFound)?;

        if occupied.contains(new_tile) {
            return Err(BoardError::InvalidMove(MoveValidationError::Occupied).into());
        }

        let tile_has_slow = effects
            .iter()
            .any(|(ef, co)| co.0 == old_tile && *ef == EffectType::Slow);

        let cost =
            check_valid_move_and_get_cost(old_pos, event.to, movement.0, pattern, tile_has_slow)
                .map_err(BoardError::InvalidMove)?;

        commands
            .entity(event.entity)
            .insert(OnBoard { position: new_tile });
        movement.0 -= cost;

        move_completed.write(CardMoved {
            card: event.entity,
            from: old_pos,
            to: event.to,
        });
    }
    Ok(())
}

pub(crate) fn refresh_movement_points(
    creatures: Query<(&mut CurrentMovementPoints, &Owner, &BaseMovementPoints), With<CreatureCard>>,
    player: Query<Entity, With<TurnPlayer>>,
) {
    for (mut movement, owner, base_movement) in creatures.into_iter() {
        if owner.0 == player.single().expect("No turn player found") {
            movement.0 = base_movement.0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::board::BoardRes;
    use bevy::math::{I16Vec2, U16Vec2};
    use std::collections::HashMap;

    fn test_board() -> BoardRes {
        BoardRes {
            tiles: HashMap::new(),
            size: U16Vec2::new(5, 5),
            player_base_positions: [U16Vec2::new(0, 0), U16Vec2::new(4, 4)],
        }
    }

    fn orthogonal_pattern() -> MovementPattern {
        MovementPattern(vec![
            I16Vec2::new(0, 1),
            I16Vec2::new(0, -1),
            I16Vec2::new(1, 0),
            I16Vec2::new(-1, 0),
        ])
    }

    #[test]
    fn bounds_are_clipped_at_corner() {
        let board = test_board();
        let pattern = orthogonal_pattern();
        let reachable = reachable_tiles(&board, U16Vec2::new(0, 0), 5, &pattern, &[], false);
        assert_eq!(reachable.len(), 2);
        assert!(reachable.contains(&U16Vec2::new(1, 0)));
        assert!(reachable.contains(&U16Vec2::new(0, 1)));
    }

    #[test]
    fn insufficient_movement_points_returns_empty() {
        let board = test_board();
        let pattern = orthogonal_pattern();
        let reachable = reachable_tiles(&board, U16Vec2::new(2, 2), 0, &pattern, &[], false);
        assert!(reachable.is_empty());
    }

    #[test]
    fn slow_tile_doubles_required_points() {
        let board = test_board();
        let pattern = orthogonal_pattern();
        // With 1 point left, a slow tile costs 2 so nothing is reachable.
        let reachable = reachable_tiles(&board, U16Vec2::new(2, 2), 1, &pattern, &[], true);
        assert!(reachable.is_empty());
        // With 2 points left, all four orthogonal neighbors become reachable.
        let reachable = reachable_tiles(&board, U16Vec2::new(2, 2), 2, &pattern, &[], true);
        assert_eq!(reachable.len(), 4);
        // Without slow, 1 point is enough.
        let reachable = reachable_tiles(&board, U16Vec2::new(2, 2), 1, &pattern, &[], false);
        assert_eq!(reachable.len(), 4);
    }

    #[test]
    fn occupied_tiles_are_filtered_out() {
        let board = test_board();
        let pattern = orthogonal_pattern();
        let occupied = vec![U16Vec2::new(3, 2), U16Vec2::new(2, 3)];
        let reachable = reachable_tiles(&board, U16Vec2::new(2, 2), 5, &pattern, &occupied, false);
        assert_eq!(reachable.len(), 2);
        assert!(reachable.contains(&U16Vec2::new(1, 2)));
        assert!(reachable.contains(&U16Vec2::new(2, 1)));
    }
}
