use bevy::ecs::{
    component::Component,
    entity::Entity,
    event::{EntityEvent, Event},
    hierarchy::ChildOf,
    lifecycle::HookContext,
    observer::On,
    system::{Commands, Query},
    world::{DeferredWorld, World},
};
use janet_bindings::types::{function::JFunction, janetenum::JanetEnum};

use crate::{actions::Action, janet_api::world_context::ScriptCtx};

pub trait Completable {
    type Res: Sync + Send + 'static + Into<JanetEnum> + Clone;
    fn on_complete() -> Self::Res;
}

#[derive(EntityEvent)]
pub struct Completed<T: Completable> {
    payload: T::Res,
    entity: Entity,
}

pub struct Callback<T: Completable, Ret> {
    trigger: Completed<T>,
    then: CallbackFunction<T::Res, Ret>,
}

pub enum CallbackFunction<Res, Ret> {
    RustFun(Box<dyn Fn(&mut ScriptCtx, &Res) -> Ret + Send + Sync + 'static>),
    JanetFun(JFunction),
}

impl<T: Completable> Callback<T, ()> {
    pub fn register(self, mut commands: Commands)
    where
        T: Completable + 'static,
        T::Res: Send + Sync + 'static,
    {
        commands.entity(self.trigger.entity).observe(
            move |e: On<Completed<T>>, mut commands: Commands, q_childs: Query<&Action>| {
                let &Action { caster } = q_childs.get(self.trigger.entity).unwrap();
                let mut ctx = ScriptCtx::new(&mut commands, e.entity, caster);
                match &self.then {
                    CallbackFunction::RustFun(f) => {
                        (f)(&mut ctx, &e.payload);
                    }
                    CallbackFunction::JanetFun(j) => {
                        let argv = [ctx.into(), e.payload.clone().into()];
                        j.eval(argv.as_slice());
                    }
                };
            },
        );
    }
}
