use bevy::{
    ecs::{
        component::Component,
        entity::Entity,
        system::{Query, ResMut, SystemParam},
    },
    ui::Val,
};

use crate::{
    GameRng,
    actions::{IsWaiter, Requirement, targeting::systems::CreatureQuery},
};

use super::targeting::{CreatureTarget, MultiTargetSelector, SingleTarget, TargetSelector};

// ============================================================================
// Value Sources - Dynamic value resolution
// ============================================================================

/// Represents where a numeric value comes from
#[derive(Component, Debug, Clone)]
pub enum ValueSource {
    /// Static constant value
    Constant(u16),

    /// Count of entities matching a selector
    Count(Box<MultiTargetSelector>),

    /// Random value in range [min, max]
    Random {
        min: Box<ValueSource>,
        max: Box<ValueSource>,
    },

    /// Value from creature stats
    CreatureStat {
        selector: Box<TargetSelector<CreatureTarget, SingleTarget>>,
        stat: StatType,
    },

    /// Mathematical operations
    Add(Box<ValueSource>, Box<ValueSource>),
    Multiply(Box<ValueSource>, Box<ValueSource>),
    Divide(Box<ValueSource>, Box<ValueSource>),
    Min(Box<ValueSource>, Box<ValueSource>),
    Max(Box<ValueSource>, Box<ValueSource>),
}

impl IsWaiter for ValueSource {
    fn emit_requirements(&self, f: &mut dyn FnMut(Requirement)) {
        match self {
            ValueSource::Constant(_) => {
                // no requirements
            }

            ValueSource::Count(multi_target_selector) => {
                f(Requirement::target(*multi_target_selector.clone()));
            }

            ValueSource::Random { min, max } => {
                f(Requirement::value(*min.clone()));
                f(Requirement::value(*max.clone()));
            }

            ValueSource::CreatureStat { selector, stat: _ } => {
                f(Requirement::target(*selector.clone()));
            }

            ValueSource::Add(a, b)
            | ValueSource::Multiply(a, b)
            | ValueSource::Divide(a, b)
            | ValueSource::Min(a, b)
            | ValueSource::Max(a, b) => {
                f(Requirement::value(*a.clone()));
                f(Requirement::value(*b.clone()));
            }
        }
    }
}

#[derive(SystemParam)]
pub struct ValueEvalParams<'w, 's> {
    pub creatures: Query<'w, 's, CreatureQuery>,
    pub rng: ResMut<'w, GameRng>,
}

impl ValueSource {
    pub fn constant(value: u16) -> Self {
        Self::Constant(value)
    }

    pub fn count(selector: MultiTargetSelector) -> Self {
        Self::Count(Box::new(selector))
    }

    pub fn eval<'w, 's>(&self, params: &mut ValueEvalParams<'w, 's>, caster: Entity) -> u16 {
        todo!()
    }
}

// ============================================================================
// Stat Modifiers
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum StatModifier {
    Attack(i16),
    Health(i16),
    MaxHealth(i16),
    Speed(i16),
    Both { attack: i16, health: i16 },
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum StatType {
    Attack,
    Health,
    MaxHealth,
    Speed,
}

// ============================================================================
// Choice Sources
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ChoiceSource {
    /// Active player chooses
    ActivePlayer,

    /// Owner of the card chooses
    Owner,

    /// Opponent chooses
    Opponent,

    /// Random choice
    Random,
}

// ============================================================================
// Implementation
// ============================================================================
