use bevy::ecs::{
    component::Component,
    entity::Entity,
    error::Result,
    event::EntityEvent,
    observer::On,
    query::{With, Without},
    system::{Commands, Query},
    world::World,
};
use janet_bindings::{error::JanetError, types::janetenum::JanetEnum};

use crate::{
    actions::{Action, ActionEffect, Condition, Execute},
    janet_api::world_context::ScriptCtx,
};

#[derive(Debug, Clone, Copy, Component)]
pub struct CanExecute;

fn check_executable(
    condition: &Condition,
    caller: Entity,
    caster: Entity,
    commands: &mut Commands,
) -> Result<bool, JanetError> {
    let mut script_ctx = ScriptCtx::new(commands, caller, caster).into();
    let JanetEnum::Bool(is_executable) = condition
        .eval_function
        .eval(std::slice::from_mut(&mut script_ctx))?
    else {
        return Err(JanetError::Type("Expected bool".into()));
    };
    Ok(is_executable)
}

pub fn eval_conditions(
    q_actions: Query<(Entity, &Condition, &Action, Option<&CanExecute>)>,
    mut commands: Commands,
) -> Result {
    for (caller, condition, action, can_execute) in q_actions {
        let should_execute = check_executable(condition, caller, action.caster, &mut commands)?;

        match (can_execute.is_some(), should_execute) {
            (false, true) => {
                commands.entity(caller).insert(CanExecute);
            }
            (true, false) => {
                commands.entity(caller).remove::<CanExecute>();
            }
            _ => {}
        }
    }
    Ok(())
}

pub fn execute_action(
    m: On<Execute>,
    q_actions: Query<(Entity, &ActionEffect, &Action)>,
    mut commands: Commands,
) -> Result {
    let (caller, ActionEffect { action }, &Action { caster }) = q_actions.get(m.event_target())?;
    let script_ctx = ScriptCtx::new(&mut commands, caller, caster);
    let argv = [script_ctx.into()];
    action.eval(&argv)?;
    Ok(())
}
