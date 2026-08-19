use bevy::{
    app::Plugin,
    ecs::{component::Component, entity::Entity, event::EntityEvent},
    math::I16Vec2,
    state::state::OnEnter,
};
use crate::{
    actions::{
        appliers::{
            apply_add_gold, apply_apply_effect, apply_deal_damage, apply_destroy_creature,
            apply_discard_cards, apply_draw_cards, apply_heal, apply_mill, apply_modify_stats,
            apply_move_creature, apply_return_to_hand,
        },
        execute::{drive_abilities, on_card_played, on_turn_end},
        hooks::HookEvent,
        value_source::StatModifier,
    },
    board::effect::EffectType,
    def::trigger::AbilityDef,
};

pub mod appliers;
pub mod conditions;
pub mod execute;
pub mod hooks;
pub mod spell_speed;
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

/// Component storing a data-driven ability definition on an ability entity.
#[derive(Component, Debug, Clone)]
pub struct AbilityData(pub AbilityDef);

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

// ============================================================================
// Core Action Types
// ============================================================================

#[derive(EntityEvent)]
pub struct MoveCreature {
    pub direction: I16Vec2,
    pub absolute: bool,
    pub entity: Entity,
}

impl MoveCreature {
    pub fn new(direction: I16Vec2, absolute: bool, entity: Entity) -> Self {
        Self {
            direction,
            absolute,
            entity,
        }
    }
}

impl HookEvent for MoveCreature {}

#[derive(EntityEvent)]
pub struct EndTurn(pub Entity);

impl HookEvent for EndTurn {}

// Atomic effects
#[derive(EntityEvent)]
pub struct DealDamage {
    pub amount: u16,
    pub entity: Entity,
}

impl DealDamage {
    pub fn new(amount: u16, entity: Entity) -> Self {
        Self { amount, entity }
    }
}

impl HookEvent for DealDamage {}

#[derive(EntityEvent)]
pub struct HealCreature {
    pub amount: u16,
    pub entity: Entity,
}

impl HealCreature {
    pub fn new(amount: u16, entity: Entity) -> Self {
        Self { amount, entity }
    }
}

impl HookEvent for HealCreature {}

#[derive(EntityEvent)]
pub struct DrawCards {
    pub amount: u16,
    pub entity: Entity,
}

impl DrawCards {
    pub fn new(amount: u16, entity: Entity) -> Self {
        Self { amount, entity }
    }
}

impl HookEvent for DrawCards {}

#[derive(EntityEvent)]
pub struct AddGold {
    pub amount: u16,
    pub entity: Entity,
}

impl AddGold {
    pub fn new(amount: u16, entity: Entity) -> Self {
        Self { amount, entity }
    }
}

impl HookEvent for AddGold {}

#[derive(EntityEvent)]
pub struct ApplyEffect {
    pub effect: EffectType,
    pub duration: u16,
    pub entity: Entity,
}

impl ApplyEffect {
    pub fn new(effect: EffectType, duration: u16, entity: Entity) -> Self {
        Self {
            effect,
            duration,
            entity,
        }
    }
}

impl HookEvent for ApplyEffect {}

#[derive(EntityEvent)]
pub struct DestroyCreature {
    pub entity: Entity,
}

impl DestroyCreature {
    pub fn new(entity: Entity) -> Self {
        Self { entity }
    }
}

impl HookEvent for DestroyCreature {}

#[derive(EntityEvent)]
pub struct ModifyStats {
    pub entity: Entity,
    pub stat_modifier: StatModifier,
}

impl ModifyStats {
    pub fn new(entity: Entity, stat_modifier: StatModifier) -> Self {
        Self {
            entity,
            stat_modifier,
        }
    }
}

impl HookEvent for ModifyStats {}

#[derive(EntityEvent)]
pub struct DiscardCards {
    pub amount: u16,
    pub entity: Entity,
}

impl DiscardCards {
    pub fn new(amount: u16, entity: Entity) -> Self {
        Self { amount, entity }
    }
}

impl HookEvent for DiscardCards {}

#[derive(EntityEvent)]
pub struct ReturnToHand {
    pub entity: Entity,
}

impl ReturnToHand {
    pub fn new(entity: Entity) -> Self {
        Self { entity }
    }
}

impl HookEvent for ReturnToHand {}

#[derive(EntityEvent)]
pub struct Mill {
    pub amount: u16,
    pub entity: Entity,
}

impl Mill {
    pub fn new(amount: u16, entity: Entity) -> Self {
        Self { amount, entity }
    }
}

impl HookEvent for Mill {}

pub struct ActionPlugin;

impl Plugin for ActionPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_message::<crate::actions::execute::ChoiceRequested>()
            .add_message::<crate::board::placement::CardPlayed>()
            .add_observer(apply_deal_damage)
            .add_observer(apply_heal)
            .add_observer(apply_draw_cards)
            .add_observer(apply_add_gold)
            .add_observer(apply_apply_effect)
            .add_observer(apply_destroy_creature)
            .add_observer(apply_modify_stats)
            .add_observer(apply_move_creature)
            .add_observer(apply_discard_cards)
            .add_observer(apply_mill)
            .add_observer(apply_return_to_hand)
            .add_systems(bevy::app::Update, drive_abilities)
            .add_systems(bevy::app::Update, on_card_played)
            .add_systems(
                OnEnter(crate::turn_controller::TurnState::EndTurn),
                on_turn_end,
            );
    }
}
