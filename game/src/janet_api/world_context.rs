use bevy::ecs::{component::Component, entity::Entity, event::Event, world::World};
use janet_bindings::{
    bindings::JanetAbstractType,
    types::{function::JFunction, janetabstract::IsAbstract},
};

#[repr(C)]
pub struct ScriptCtx<'w> {
    world: &'w mut World,
    caller: Entity,
}

impl<'w> ScriptCtx<'w> {
    pub fn new(world: &'w mut bevy::prelude::World, caller: Entity) -> Self {
        Self { world, caller }
    }

    pub fn trigger<'a, E>(&mut self, event: E)
    where
        E: Event<Trigger<'a>: Default>,
    {
        self.world.trigger(event);
    }
}

impl<'w> IsAbstract for ScriptCtx<'w> {
    fn type_info() -> &'static janet_bindings::bindings::JanetAbstractType {
        const CONDITION_ATYPE: JanetAbstractType =
            JanetAbstractType::new(c"main/script-cxt", ScriptCtx::gc);
        &CONDITION_ATYPE
    }
}
