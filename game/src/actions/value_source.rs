use bevy::ecs::{
    component::Component,
    entity::Entity,
    query::With,
    system::Query,
};

use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::{
    GameRng,
    actions::targeting::{
        IsTargetSelectMode,
        filters::FilterParams,
        systems::{CreatureQuery, HandQuery, PlayerQuery, TileQuery},
    },
    board::{effect::EffectType, tile::Tile},
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
    Sub(Box<ValueSource>, Box<ValueSource>),
    Multiply(Box<ValueSource>, Box<ValueSource>),
    Divide(Box<ValueSource>, Box<ValueSource>),
    Min(Box<ValueSource>, Box<ValueSource>),
    Max(Box<ValueSource>, Box<ValueSource>),
}

/// Borrowed view of all runtime queries needed to evaluate a [`ValueSource`].
///
/// This is intentionally not a [`SystemParam`](bevy::ecs::system::SystemParam);
/// callers build it from [`FilterParams`] plus a mutable borrow of [`GameRng`].
pub struct ValueEvalParams<'a, 'w, 's> {
    pub creatures: &'a Query<'w, 's, CreatureQuery>,
    pub tiles: &'a Query<'w, 's, TileQuery, With<Tile>>,
    pub hand: &'a Query<'w, 's, HandQuery>,
    pub player: &'a Query<'w, 's, PlayerQuery>,
    pub effects: &'a Query<'w, 's, &'static EffectType>,
    pub rng: &'a mut GameRng,
    pub current_target: Option<Entity>,
}

impl<'w, 's> FilterParams<'w, 's> {
    pub fn as_value_params<'a>(&'a self, rng: &'a mut GameRng) -> ValueEvalParams<'a, 'w, 's> {
        ValueEvalParams {
            creatures: &self.creatures,
            tiles: &self.tiles,
            hand: &self.hand,
            player: &self.player,
            effects: &self.effects,
            rng,
            current_target: None,
        }
    }
}

impl ValueSource {
    pub fn constant(value: u16) -> Self {
        Self::Constant(value)
    }

    pub fn count(selector: MultiTargetSelector) -> Self {
        Self::Count(Box::new(selector))
    }

    pub fn eval(&self, params: &mut ValueEvalParams, caster: Entity) -> u16 {
        match self {
            ValueSource::Constant(v) => *v,

            ValueSource::Count(selector) => match selector.as_ref() {
                MultiTargetSelector::Creature(s) => s.selection.find_suitable(params, caster),
                MultiTargetSelector::Tile(s) => s.selection.find_suitable(params, caster),
                MultiTargetSelector::Player(s) => s.selection.find_suitable(params, caster),
                MultiTargetSelector::Hand(s) => s.selection.find_suitable(params, caster),
            }
            .len() as u16,

            ValueSource::Random { min, max } => {
                let min = min.eval(params, caster);
                let max = max.eval(params, caster);
                if min > max {
                    min
                } else {
                    params.rng.0.random_range(min..=max)
                }
            }

            ValueSource::CreatureStat { selector, stat } => {
                let entities = selector.as_ref().selection.find_suitable(params, caster);
                let entity = params
                    .current_target
                    .or(entities.first().copied())
                    .unwrap_or(Entity::PLACEHOLDER);
                if entity == Entity::PLACEHOLDER {
                    return 0;
                }
                let Ok(creature) = params.creatures.get(entity) else {
                    return 0;
                };

                match stat {
                    StatType::Attack => creature.current_atttack.0,
                    // Health is the current-HP value; MaxHealth is the immutable max.
                    StatType::Health => creature.current_defense.0,
                    StatType::MaxHealth => creature.health.value(),
                    StatType::Speed => creature.movement_points.0,
                }
            }

            ValueSource::Add(a, b) => {
                a.eval(params, caster).saturating_add(b.eval(params, caster))
            }
            ValueSource::Sub(a, b) => {
                a.eval(params, caster).saturating_sub(b.eval(params, caster))
            }
            ValueSource::Multiply(a, b) => {
                a.eval(params, caster).saturating_mul(b.eval(params, caster))
            }
            ValueSource::Divide(a, b) => {
                let a = a.eval(params, caster);
                let b = b.eval(params, caster);
                if b == 0 {
                    0
                } else {
                    a / b
                }
            }
            ValueSource::Min(a, b) => a.eval(params, caster).min(b.eval(params, caster)),
            ValueSource::Max(a, b) => a.eval(params, caster).max(b.eval(params, caster)),
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

fn apply_delta(value: &mut u16, delta: i16) {
    if delta >= 0 {
        *value = value.saturating_add(delta as u16);
    } else {
        *value = value.saturating_sub((-delta) as u16);
    }
}

impl StatModifier {
    /// Apply this modifier to raw stat values.
    pub fn apply(
        &self,
        attack: &mut u16,
        health: &mut u16,
        max_health: &mut u16,
        speed: &mut u16,
    ) {
        match *self {
            StatModifier::Attack(d) => apply_delta(attack, d),
            StatModifier::Health(d) => apply_delta(health, d),
            StatModifier::MaxHealth(d) => apply_delta(max_health, d),
            StatModifier::Speed(d) => apply_delta(speed, d),
            StatModifier::Both { attack: a, health: h } => {
                apply_delta(attack, a);
                apply_delta(health, h);
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
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
