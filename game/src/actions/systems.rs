use bevy::ecs::{
    component::Component,
    entity::Entity,
    error::Result,
    event::EntityEvent,
    observer::On,
    query::With,
    system::{Commands, Query},
};
use janet_bindings::{error::JanetError, types::janetenum::JanetEnum};

use crate::{
    actions::{Action, ActionEffect, Condition, Execute},
    janet_api::world_context::{ScriptCache, ScriptCtx},
};

#[derive(Debug, Clone, Copy, Component)]
pub struct CanExecute;

fn check_executable(
    condition: &Condition,
    commands: &mut Commands,
    calling_action: Entity,
    caster: Entity,
    cache_q: &ScriptCache,
) -> Result<bool, JanetError> {
    let mut script_ctx = ScriptCtx::new(commands, cache_q, calling_action, caster).into();
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
    cache_q: ScriptCache,
) -> Result {
    for (caller, condition, action, can_execute) in q_actions {
        let should_execute =
            check_executable(condition, &mut commands, caller, action.caster, &cache_q)?;

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
    q_actions: Query<(Entity, &ActionEffect, &Action), With<CanExecute>>,
    mut commands: Commands,
    cache_q: ScriptCache,
) -> Result {
    let (caller, ActionEffect { action }, &Action { caster }) = q_actions.get(m.event_target())?;
    let script_ctx = ScriptCtx::new(&mut commands, &cache_q, caller, caster);
    let argv = [script_ctx.into()];
    action.eval(&argv)?;
    Ok(())
}
