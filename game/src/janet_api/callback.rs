use bevy::ecs::{
    component::Component,
    entity::Entity,
    event::{EntityEvent, Event},
    hierarchy::ChildOf,
    observer::On,
    system::Commands,
    world::World,
};
use janet_bindings::types::{function::JFunction, janetenum::JanetEnum};

use crate::janet_api::world_context::ScriptCtx;

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
            move |e: On<Completed<T>>, world: &mut World| {
                let caster = world.get::<ChildOf>(self.trigger.entity).unwrap();
                let mut ctx = ScriptCtx::new(world, e.entity, caster.0);
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
