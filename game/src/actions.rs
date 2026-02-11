use bevy::{
    app::{FixedUpdate, Plugin},
    ecs::{
        bundle::Bundle, component::Component, entity::Entity, event::EntityEvent, message::Message,
    },
    math::I16Vec2,
};
use janet_bindings::types::function::JFunction;

use crate::{
    actions::{
        hooks::{Hook, HookEvent},
        spell_speed::SpellSpeed,
        systems::{eval_conditions, execute_action},
        value_source::StatModifier,
    },
    board::effect::EffectType,
};

pub mod action_builder;
pub mod action_parser;
pub mod conditions;
pub mod hooks;
pub mod spell_speed;
pub mod systems;
pub mod targeting;
pub mod timing;
pub mod value_source;

#[derive(Component, Debug, Clone)]
#[relationship_target(relationship = Action)]
pub struct Actions(Vec<Entity>);

#[derive(Component, Debug, Clone, Copy)]
#[relationship(relationship_target = Actions)]
pub struct Action {
    #[relationship]
    pub caster: Entity,
}

#[derive(Component, Debug, Clone, Copy)]
pub struct Pending;

#[derive(Component, Debug, Clone, Copy)]
pub struct NeedsTargeting;

#[derive(Component, Debug, Clone, Copy)]
pub struct NeedsFiltering;

#[derive(EntityEvent)]
pub struct Execute(pub Entity);

impl From<Entity> for Execute {
    fn from(entity: Entity) -> Self {
        Execute(entity)
    }
}

#[derive(Component, Debug, Clone)]
pub struct Condition {
    pub eval_function: JFunction,
}

impl Condition {
    fn new(eval_function: JFunction) -> Self {
        Self { eval_function }
    }
}

#[derive(Component, Debug, Clone)]
pub struct ActionEffect {
    pub action: JFunction,
}

impl From<JFunction> for ActionEffect {
    fn from(value: JFunction) -> Self {
        ActionEffect { action: value }
    }
}

#[derive(Bundle, Debug, Clone)]
pub struct GameAction {
    pub condition: Condition,
    pub speed: SpellSpeed,
    pub action: ActionEffect,
}

impl GameAction {
    fn new(condition: Condition, speed: SpellSpeed, action: ActionEffect) -> Self {
        Self {
            condition,
            speed,
            action,
        }
    }
}

// ============================================================================
// Core Action Types
// ============================================================================

#[derive(EntityEvent)]
pub struct MoveCreature {
    direction: I16Vec2,
    absolute: bool,
    entity: Entity,
}

impl HookEvent for MoveCreature {}

#[derive(EntityEvent)]
pub struct EndTurn(Entity);

impl HookEvent for EndTurn {}

// Atomic effects
#[derive(EntityEvent)]
pub struct DealDamage {
    pub amount: u16,
    pub entity: Entity,
}

impl HookEvent for DealDamage {}

#[derive(EntityEvent)]
pub struct HealCreature {
    pub amount: u16,
    pub entity: Entity,
}

impl HookEvent for HealCreature {}

#[derive(EntityEvent)]
pub struct DrawCards {
    amount: u16,
    entity: Entity,
}

impl HookEvent for DrawCards {}

#[derive(EntityEvent)]
pub struct AddGold {
    amount: u16,
    entity: Entity,
}

impl HookEvent for AddGold {}
#[derive(EntityEvent)]
pub struct ApplyEffect {
    effect: EffectType,
    duration: u16,
    entity: Entity,
}

impl HookEvent for ApplyEffect {}

#[derive(EntityEvent)]
pub struct DestroyCreature {
    entity: Entity,
}
impl HookEvent for DestroyCreature {}

#[derive(EntityEvent)]
pub struct ModifyStats {
    entity: Entity,
    stat_modifier: StatModifier,
}

impl HookEvent for ModifyStats {}

#[derive(EntityEvent)]
pub struct DiscardCards {
    amount: u16,
    entity: Entity,
}

impl HookEvent for DiscardCards {}

#[derive(EntityEvent)]
pub struct ReturnToHand {
    entity: Entity,
}

impl HookEvent for ReturnToHand {}

#[derive(EntityEvent)]
pub struct Mill {
    amount: u16,
    entity: Entity,
}

impl HookEvent for Mill {}

pub struct ActionPlugin;

impl Plugin for ActionPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_observer(execute_action)
            .add_systems(FixedUpdate, eval_conditions);
    }
}
