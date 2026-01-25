use bevy::ecs::{bundle::Bundle, component::Component, entity::Entity};

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

impl Requirement {
    #[inline]
    fn target<T: Into<AnyTargetSelector>>(t: T) -> Self {
        Requirement::Target(t.into())
    }

    #[inline]
    fn value(v: ValueSource) -> Self {
        Requirement::Value(v.into())
    }

    #[inline]
    fn cond(c: Condition) -> Self {
        Requirement::Cond(c)
    }
}

pub trait IsWaiter {
    fn emit_requirements(&self, f: &mut dyn FnMut(Requirement));
}

impl IsWaiter for UnitAction {
    fn emit_requirements(&self, f: &mut dyn FnMut(Requirement)) {
        match self {
            UnitAction::MoveCreature { target, .. } => {
                f(Requirement::target(target.clone()));
            }

            UnitAction::DealDamage {
                target_selector,
                amount,
            }
            | UnitAction::HealCreature {
                target_selector,
                amount,
            } => {
                f(Requirement::target(target_selector.clone()));
                f(Requirement::value(amount.clone()));
            }

            UnitAction::DrawCards {
                count,
                player_selector,
            }
            | UnitAction::AddGold {
                amount: count,
                player_selector,
            } => {
                f(Requirement::target(player_selector.clone()));
                f(Requirement::value(count.clone()));
            }

            UnitAction::ApplyEffect {
                duration,
                targeting_type,
                ..
            } => {
                f(Requirement::target(targeting_type.clone()));
                f(Requirement::value(duration.clone()));
            }

            UnitAction::SummonCreature { position, .. } => {
                f(Requirement::target(position.clone()));
            }

            UnitAction::DestroyCreature { targeting_type }
            | UnitAction::ModifyStats { targeting_type, .. }
            | UnitAction::ReturnToHand { targeting_type } => {
                f(Requirement::target(targeting_type.clone()));
            }

            UnitAction::DiscardCards { count, .. } => {
                f(Requirement::value(count.clone()));
            }

            UnitAction::Mill {
                count,
                player_selector,
            } => {
                f(Requirement::value(count.clone()));
                f(Requirement::target(player_selector.clone()));
            }

            UnitAction::EndTurn => {}
        }
    }
}
