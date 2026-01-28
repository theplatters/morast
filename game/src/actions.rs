use bevy::{
    ecs::{bundle::Bundle, component::Component, entity::Entity, event::EntityEvent},
    math::I16Vec2,
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

pub enum Expr {
    Expr(Box<Expr>),
    Logic(Logic),
    UnitAction(UnitAction),
}

pub enum Logic {
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
}

// ============================================================================
// Core Action Types
// ============================================================================

/// Main action effect that can be executed
#[derive(Component, Debug, Clone)]
pub enum UnitAction {
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
        choice: ChoiceSource,
    },
    ReturnToHand {
        targeting_type: CreatureSel<targeting::Or<SingleTarget, MultiTarget>>,
    },
    Mill {
        count: ValueSource,
        player_selector: PlayerSel<targeting::Or<SingleTarget, MultiTarget>>,
    },
}

impl GameAction {
    pub fn builder() -> ActionPrototypeBuilder {
        ActionPrototypeBuilder::new()
    }
}

pub enum Requirement {
    Target(AnyTargetSelector),
    Value(ValueSource),
    Cond(Condition),
}

impl From<AnyTargetSelector> for Requirement {
    fn from(value: AnyTargetSelector) -> Self {
        Requirement::Target(value)
    }
}

impl From<ValueSource> for Requirement {
    fn from(value: ValueSource) -> Self {
        Requirement::Value(value)
    }
}

impl From<Condition> for Requirement {
    fn from(value: Condition) -> Self {
        Requirement::Cond(value)
    }
}

#[derive(EntityEvent)]
pub struct MoveCreature {
    direction: I16Vec2,
    absolute: bool,
    entity: Entity,
}

#[derive(EntityEvent)]
pub struct EndTurn(Entity);

// Atomic effects
#[derive(EntityEvent)]
pub struct DealDamage {
    pub amount: u16,
    pub entity: Entity,
}

#[derive(EntityEvent)]
pub struct HealCreature {
    pub amount: u16,
    pub entity: Entity,
}
#[derive(EntityEvent)]
pub struct DrawCards {
    amount: u16,
    entity: Entity,
}
#[derive(EntityEvent)]
pub struct AddGold {
    amount: u16,
    entity: Entity,
}
#[derive(EntityEvent)]
pub struct ApplyEffect {
    effect: EffectType,
    duration: u16,
    entity: Entity,
}

#[derive(EntityEvent)]
pub struct DestroyCreature {
    entity: Entity,
}
#[derive(EntityEvent)]
pub struct ModifyStats {
    entity: Entity,
    stat_modifier: StatModifier,
}
#[derive(EntityEvent)]
pub struct DiscardCards {
    amount: u16,
    entity: Entity,
}
#[derive(EntityEvent)]
pub struct ReturnToHand {
    entity: Entity,
}
#[derive(EntityEvent)]
pub struct Mill {
    amount: u16,
    entity: Entity,
}
