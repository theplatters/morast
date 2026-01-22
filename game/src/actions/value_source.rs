use bevy::ecs::{
    component::Component,
    entity::Entity,
    system::{Query, ResMut, SystemParam},
};

use crate::{GameRng, actions::targeting::systems::CreatureQuery};

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
        min: u16,
        max: u16,
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
        // Helper to clamp i32 -> u16
        fn clamp_u16(v: i32) -> u16 {
            v.clamp(0, u16::MAX as i32) as u16
        }

        match self {
            Self::Constant(v) => *v,

            Self::Random { min, max } => {
                let (a, b) = (*min, *max);
                let (lo, hi) = if a <= b { (a, b) } else { (b, a) };
                todo!()
            }

            Self::Count(sel) => todo!(),

            Self::CreatureStat { selector, stat } => {
                todo!()
            }

            Self::Add(a, b) => {
                let av = a.eval(params, caster) as i32;
                let bv = b.eval(params, caster) as i32;
                clamp_u16(av + bv)
            }

            Self::Multiply(a, b) => {
                let av = a.eval(params, caster) as i32;
                let bv = b.eval(params, caster) as i32;
                clamp_u16(av.saturating_mul(bv))
            }

            Self::Divide(a, b) => {
                let av = a.eval(params, caster) as i32;
                let bv = b.eval(params, caster) as i32;
                if bv == 0 { 0 } else { clamp_u16(av / bv) }
            }

            Self::Min(a, b) => {
                let av = a.eval(params, caster);
                let bv = b.eval(params, caster);
                av.min(bv)
            }

            Self::Max(a, b) => {
                let av = a.eval(params, caster);
                let bv = b.eval(params, caster);
                av.max(bv)
            }
        }
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
