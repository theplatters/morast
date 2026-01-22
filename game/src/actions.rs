use bevy::{
    ecs::{bundle::Bundle, component::Component, entity::Entity},
    math::I16Vec2,
    ui::Val,
};

use crate::{
    actions::{
        action_builder::ActionPrototypeBuilder,
        conditions::Condition,
        spell_speed::SpellSpeed,
        targeting::{
            AnyTargetSelector, CreatureSel, MultiTarget, PlayerSel, SingleTarget, TileSel,
        },
        timing::ActionTiming,
        value_source::{ChoiceSource, StatModifier, ValueSource},
    },
    board::effect::EffectType,
    card::card_id::CardID,
};

pub mod action_builder;
pub mod action_systems;
pub mod conditions;
pub mod spell_speed;
pub mod systems;
pub mod targeting;
pub mod timing;
pub mod value_source;
#[derive(Component, Debug, Clone, Copy)]
pub struct Pending;

#[derive(Component, Debug, Clone, Copy)]
pub struct NeedsTargeting;

#[derive(Component, Debug, Clone, Copy)]
pub struct NeedsFiltering;

#[derive(Component)]
#[relationship_target(relationship = RequiredForCompletion, linked_spawn)]
pub struct WaitForEntities {
    remaining: Vec<Entity>,
}

#[derive(Component)]
#[relationship(relationship_target = WaitForEntities)]
pub struct RequiredForCompletion(pub Entity);

#[derive(Component)]
struct Done; // added to any entity when it finishes

#[derive(Component)]
struct ReadyToProceed;

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
        direction_x: ValueSource,
        direction_y: ValueSource,
        absolute: bool,
        target: CreatureSel<SingleTarget>,
    },
    EndTurn,

    // Atomic effects
    DealDamage {
        target_selector: CreatureSel<targeting::Or<SingleTarget, MultiTarget>>,
        amount: ValueSource,
    },
    HealCreature {
        target_selector: CreatureSel<targeting::Or<SingleTarget, MultiTarget>>,
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
        player_selector: PlayerSel<targeting::Or<SingleTarget, MultiTarget>>,
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

pub enum RequirementRef<'a> {
    Target(&'a AnyTargetSelector),
    Value(&'a ValueSource),
    Cond(&'a Condition),
}

impl UnitAction {
    pub fn requirements(&self) -> Vec<RequirementRef<'_>> {
        match self {
            UnitAction::MoveCreature { target, .. } => vec![RequirementRef::Target(target)],

            UnitAction::DealDamage {
                target_selector,
                amount,
            } => vec![
                RequirementRef::Target(target_selector),
                RequirementRef::Value(amount),
            ],

            UnitAction::HealCreature {
                target_selector,
                amount,
            } => vec![
                RequirementRef::Target(target_selector),
                RequirementRef::Value(amount),
            ],

            UnitAction::Conditional { condition, .. } => vec![RequirementRef::Cond(condition)],

            _ => smallvec![],
        }
    }
}
