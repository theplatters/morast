use bevy::ecs::{
    component::Component,
    entity::Entity,
    error::Result,
    hierarchy::ChildOf,
    query::{With, Without},
    system::{Commands, Query},
    world::World,
};
use janet_bindings::{error::JanetError, types::janetenum::JanetEnum};

use crate::{
    actions::{Action, Condition, GameAction},
    janet_api::world_context::ScriptCtx,
};

#[derive(Debug, Clone, Copy, Component)]
pub struct CanExecute;

fn can_execute(
    condition: &Condition,
    caller: Entity,
    caster: Entity,
    world: &mut World,
) -> Result<bool, JanetError> {
    let mut script_ctx = ScriptCtx::new(world, caller, caster).into();
    let JanetEnum::Bool(is_executable) = condition
        .eval_function
        .eval(std::slice::from_mut(&mut script_ctx))?
    else {
        return Err(JanetError::Type("Expected bool".into()));
    };
    Ok(is_executable)
}

pub fn eval_inactive_condition(
    q_actions: Query<(Entity, &Condition, &Action), Without<CanExecute>>,
    world: &mut World,
    mut commands: Commands,
) -> Result {
    for (caller, action, caster) in q_actions {
        if can_execute(action, caller, caster.caster, world)? {
            commands.entity(caller).insert(CanExecute);
        }
    }
    Ok(())
}

pub fn eval_active_condition(
    q_actions: Query<(Entity, &Condition, &Action), With<CanExecute>>,
    world: &mut World,
    mut commands: Commands,
) -> Result {
    for (caller, action, caster) in q_actions {
        if !can_execute(action, caller, caster.caster, world)? {
            commands.entity(caller).remove::<CanExecute>();
        }
    }
    Ok(())
}
