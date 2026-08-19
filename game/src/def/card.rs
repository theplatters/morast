use bevy::{asset::Asset, reflect::TypePath};
use serde::{Deserialize, Serialize};

use crate::card::abilities::Abilities;

use super::trigger::AbilityDef;

/// A card definition loaded from a `.ron` file in `assets/cards`.
#[derive(Asset, TypePath, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CardDef {
    pub name: String,
    pub cost: u16,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub display_image: String,
    pub kind: CardKindDef,
    /// Triggered abilities of the card.
    #[serde(default)]
    pub abilities: Vec<AbilityDef>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CardKindDef {
    Creature(CreatureStatsDef),
    Spell,
    Trap,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreatureStatsDef {
    pub attack: u16,
    pub defense: u16,
    pub movement_points: u16,
    pub movement: PatternDef,
    pub attack_pattern: PatternDef,
    /// Keyword abilities (Flying, Jumping, Digging).
    #[serde(default)]
    pub abilities: Vec<Abilities>,
}

/// Movement/attack pattern. `Plus(n)` is the orthogonal cross with radius
/// `n`; `Cross(n)` is the diagonal cross with radius `n`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PatternDef {
    /// Explicit list of `[dx, dy]` offsets.
    Offsets(Vec<[i16; 2]>),
    Plus(u16),
    Cross(u16),
    Union(Vec<PatternDef>),
}
