use bevy::{
    ecs::{bundle::Bundle, component::Component},
    math::I16Vec2,
};

use crate::{
    actions::{
        action_builder::ActionPrototypeBuilder,
        action_prototype::{ChoiceSource, StatModifier, ValueSource},
        conditions::Condition,
        spell_speed::SpellSpeed,
        targeting::{CreatureSel, MultiTarget, PlayerSel, SingleTarget, TileSel},
        timing::ActionTiming,
    },
    board::effect::EffectType,
    card::card_id::CardID,
};

pub mod action_builder;
pub mod action_prototype;
pub mod action_systems;
pub mod conditions;
pub mod spell_speed;
pub mod targeting;
pub mod timing;
#[derive(Component, Debug, Clone, Copy)]
pub struct Pending;

#[derive(Component, Debug, Clone, Copy)]
pub struct NeedsTargeting;

#[derive(Component, Debug, Clone, Copy)]
pub struct NeedsFiltering;

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
        targeting_type: CreatureSel<targeting::Or<SingleTarget, MultiTarget>>,
        amount: ValueSource,
    },
    HealCreature {
        targeting_type: CreatureSel<targeting::Or<SingleTarget, MultiTarget>>,
        amount: ValueSource,
    },
    DrawCards {
        count: ValueSource,
        player_selector: PlayerSel<targeting::Or<SingleTarget, MultiTarget>>,
    },
    AddGold {
        amount: ValueSource,
        player_selector: PlayerSel<targeting::Or<SingleTarget, MultiTarget>>,
    },
    ApplyEffect {
        effect: EffectType,
        duration: ValueSource,
        targeting_type: TileSel<targeting::Or<SingleTarget, MultiTarget>>,
    },
    SummonCreature {
        creature_id: CardID,
        position: TileSel<SingleTarget>,
    },
    DestroyCreature {
        targeting_type: CreatureSel<targeting::Or<SingleTarget, MultiTarget>>,
    },
    ModifyStats {
        targeting_type: CreatureSel<targeting::Or<SingleTarget, MultiTarget>>,
        stat_modifier: StatModifier,
    },
    DiscardCards {
        count: ValueSource,
        random: bool,
    },
    ReturnToHand {
        targeting_type: CreatureSel<targeting::Or<SingleTarget, MultiTarget>>,
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

impl GameAction {
    pub fn builder() -> ActionPrototypeBuilder {
        ActionPrototypeBuilder::new()
    }
}
