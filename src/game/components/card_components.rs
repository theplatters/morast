// src/game/ecs/components.rs

use bevy::ecs::{component::Component, entity::Entity};
use macroquad::math::I16Vec2;

use crate::game::card::card_id::CardID;

#[derive(Component)]
pub struct CardRef(pub CardID);

// ============================================
// CARD TYPE MARKERS (still useful for queries)
// ============================================

#[derive(Component)]
pub struct CreatureCard;

#[derive(Component)]
pub struct SpellCard;

#[derive(Component)]
pub struct TrapCard;

// ============================================
// LOCATION COMPONENTS (instance-specific)
// ============================================

#[derive(Component)]
pub struct InDeck {
    pub owner: Entity,
    pub position: usize,
}

#[derive(Component)]
pub struct InHand {
    pub owner: Entity,
    pub hand_index: usize,
}

#[derive(Component)]
pub struct OnBoard {
    pub owner: Entity,
    pub board_position: I16Vec2,
}

#[derive(Component)]
pub struct InGraveyard {
    pub owner: Entity,
    pub order: usize,
}

// ============================================
// MUTABLE INSTANCE STATE (what changes during play)
// ============================================

/// Current combat state for a creature on the board
#[derive(Component)]
pub struct CombatState {
    pub current_attack: i16, // Can be modified by buffs/debuffs
    pub current_defense: i16,
    pub health: u16,
}

/// Current movement state
#[derive(Component)]
pub struct MovementState {
    pub remaining_points: u16,
}

/// Accumulated stat modifiers from various effects
#[derive(Component)]
pub struct Modifiers {
    pub attack: i16,
    pub defense: i16,
    pub movement: i16,
    pub cost: i16, // For cards in hand
}

#[derive(Component)]
pub struct Owner(pub Entity);
