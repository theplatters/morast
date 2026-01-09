// ============================================================================
// File: action_prototype.rs - Refined action system with conditions
// ============================================================================
use bevy::{
    ecs::{bundle::Bundle, component::Component, system::Single},
    math::I16Vec2,
};

use super::{
    action_builder::ActionPrototypeBuilder,
    conditions::Condition,
    spell_speed::SpellSpeed,
    targeting::{
        AnyTargetSelector, CreatureTarget, MultiTarget, MultiTargetSelector, PlayerSel,
        SingleTarget, TargetSelector, TileTarget,
    },
    timing::ActionTiming,
};

use crate::board::{effect::EffectType, tile::Tile};
use crate::card::card_id::CardID;
use crate::error::GameError;

#[derive(Component, Debug, Clone, Copy)]
pub struct Pending;

#[derive(Component, Debug, Clone, Copy)]
pub struct NeedsTargeting;

#[derive(Component, Debug, Clone, Copy)]
pub struct ReadyToExecute;

#[derive(Component, Debug, Clone)]
pub struct Counterable;

#[derive(Bundle, Debug, Clone)]
pub struct GameAction {
    pub action: UnitAction,
    pub timing: ActionTiming,
    pub speed: SpellSpeed,
}

// ============================================================================
// Core Action Types
// ============================================================================

/// Main action effect that can be executed
#[derive(Component, Debug, Clone)]
pub enum UnitAction {
    // Basic game actions
    PlaceCreature,
    CastSpell,
    PlaceTrap,
    MoveCreature {
        direction: I16Vec2,
    },
    EndTurn,

    // Atomic effects
    DealDamage {
        targeting_type:
            TargetSelector<CreatureTarget, super::targeting::Or<SingleTarget, MultiTarget>>,
        amount: ValueSource,
    },
    HealCreature {
        targeting_type:
            TargetSelector<CreatureTarget, super::targeting::Or<SingleTarget, MultiTarget>>,
        amount: ValueSource,
    },
    DrawCards {
        count: ValueSource,
        player_selector: PlayerSel<super::targeting::Or<SingleTarget, MultiTarget>>,
    },
    AddGold {
        amount: ValueSource,
        player_selector: PlayerSel<super::targeting::Or<SingleTarget, MultiTarget>>,
    },
    ApplyEffect {
        effect: EffectType,
        duration: ValueSource,
        targeting_type: TargetSelector<TileTarget, super::targeting::Or<SingleTarget, MultiTarget>>,
    },
    SummonCreature {
        creature_id: CardID,
        position: TargetSelector<TileTarget, SingleTarget>,
    },
    DestroyCreature {
        targeting_type:
            TargetSelector<CreatureTarget, super::targeting::Or<SingleTarget, MultiTarget>>,
    },
    ModifyStats {
        targeting_type:
            TargetSelector<CreatureTarget, super::targeting::Or<SingleTarget, MultiTarget>>,
        stat_modifier: StatModifier,
    },
    DiscardCards {
        count: ValueSource,
        random: bool,
    },
    ReturnToHand {
        targeting_type:
            TargetSelector<CreatureTarget, super::targeting::Or<SingleTarget, MultiTarget>>,
    },
    Mill {
        count: ValueSource,
    },

    // Composite actions with better control flow
    Sequence(Vec<UnitAction>),
    Parallel(Vec<UnitAction>),
    Choice {
        options: Vec<UnitAction>,
        chooser: ChoiceSource,
    },

    Repeat {
        action: Box<UnitAction>,
        count: ValueSource,
    },

    // Conditional actions
    Conditional {
        condition: Condition,
        on_true: Box<UnitAction>,
        on_false: Option<Box<UnitAction>>,
    },

    // Advanced patterns
    ForEach {
        action: Box<UnitAction>,
    },
}

// ============================================================================
// Value Sources - Dynamic value resolution
// ============================================================================

/// Represents where a numeric value comes from
#[derive(Debug, Clone)]
pub enum ValueSource {
    /// Static constant value
    Constant(u16),

    /// Count of entities matching a selector
    Count(MultiTargetSelector),

    /// Random value in range [min, max]
    Random {
        min: u16,
        max: u16,
    },

    /// Value from creature stats
    CreatureStat {
        selector: TargetSelector<CreatureTarget, SingleTarget>,
        stat: StatType,
    },

    /// Mathematical operations
    Add(Box<ValueSource>, Box<ValueSource>),
    Multiply(Box<ValueSource>, Box<ValueSource>),
    Divide(Box<ValueSource>, Box<ValueSource>),
    Min(Box<ValueSource>, Box<ValueSource>),
    Max(Box<ValueSource>, Box<ValueSource>),
}

impl ValueSource {
    pub fn constant(value: u16) -> Self {
        Self::Constant(value)
    }

    pub fn count(selector: MultiTargetSelector) -> Self {
        Self::Count(selector)
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

impl GameAction {
    pub fn builder() -> ActionPrototypeBuilder {
        ActionPrototypeBuilder::new()
    }
}
