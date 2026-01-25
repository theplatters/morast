use bevy::ecs::query::With;
use bevy::ecs::{
    entity::Entity,
    query::{Added, AnyOf},
    system::{Commands, Query},
};

use crate::actions::{
    IsWaiter, NeedsTargeting, Pending, RequiredForCompletion, Requirement, UnitAction,
    targeting::AnyTargetSelector, value_source::ValueSource,
};

pub fn spawn_requirements(
    q_actions: Query<
        (
            Entity,
            AnyOf<(&UnitAction, &ValueSource, &AnyTargetSelector)>,
        ),
        Added<Pending>,
    >,
    mut commands: Commands,
) {
    for (action_e, a) in q_actions {
        let mut spawn_req = |requirement: Requirement| match requirement {
            Requirement::Target(any_target_selector) => {
                commands.spawn((
                    any_target_selector,
                    Pending,
                    RequiredForCompletion(action_e),
                    NeedsTargeting,
                ));
            }
            Requirement::Value(value_source) => {
                commands.spawn((value_source, Pending, RequiredForCompletion(action_e)));
            }
            Requirement::Cond(condition) => {
                commands.spawn((condition, Pending, RequiredForCompletion(action_e)));
            }
        };

        if let Some(unit_action) = a.0 {
            unit_action.emit_requirements(&mut spawn_req);
        }
        if let Some(value_source) = a.1 {
            value_source.emit_requirements(&mut spawn_req);
        }
        if let Some(target_selector) = a.2 {
            target_selector.emit_requirements(&mut spawn_req);
        }
    }
}
