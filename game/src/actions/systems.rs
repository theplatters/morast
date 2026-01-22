use bevy::ecs::{
    entity::Entity,
    query::{Added, With},
    system::{Commands, Query},
};

use crate::actions::{Pending, RequiredForCompletion, UnitAction};

pub fn execute_action(
    q_actions: Query<(Entity, &UnitAction), Added<Pending>>,
    mut commands: Commands,
) {
    for (action_entity, action_effect) in q_actions {
        match action_effect {
            UnitAction::PlaceCreature => todo!(),
            UnitAction::CastSpell => todo!(),
            UnitAction::PlaceTrap => todo!(),
            UnitAction::EndTurn => todo!(),
            UnitAction::MoveCreature { target, .. } => {}
            UnitAction::DealDamage {
                target_selector: targeting_type,
                amount,
            } => todo!(),
            UnitAction::HealCreature {
                target_selector: targeting_type,
                amount,
            } => todo!(),
            UnitAction::DrawCards {
                count,
                player_selector,
            } => todo!(),
            UnitAction::AddGold {
                amount,
                player_selector,
            } => todo!(),
            UnitAction::ApplyEffect {
                effect,
                duration,
                targeting_type,
            } => todo!(),
            UnitAction::SummonCreature {
                creature_id,
                position,
            } => todo!(),
            UnitAction::DestroyCreature { targeting_type } => todo!(),
            UnitAction::ModifyStats {
                targeting_type,
                stat_modifier,
            } => todo!(),
            UnitAction::DiscardCards { count, random } => todo!(),
            UnitAction::ReturnToHand { targeting_type } => todo!(),
            UnitAction::Mill {
                count,
                player_selector,
            } => todo!(),
            UnitAction::Sequence(unit_actions) => todo!(),
            UnitAction::Parallel(unit_actions) => todo!(),
            UnitAction::Choice { options, chooser } => todo!(),
            UnitAction::Repeat { action, count } => todo!(),
            UnitAction::Conditional {
                condition,
                on_true,
                on_false,
            } => todo!(),
            UnitAction::ForEach { action } => todo!(),
        }
    }
}
