use bevy::ecs::{
    component::Component, entity::Entity, event::Event, query::With, system::Single, world::World,
};
use janet_bindings::{
    bindings::JanetAbstractType,
    types::{function::JFunction, janetabstract::IsAbstract},
};

use crate::player::{Player, TurnPlayer};

#[repr(C)]
pub struct ScriptCtx {
    world: *mut World,
    caller: Entity,
    caster: Entity,
}

impl ScriptCtx {
    pub fn new(world: &mut bevy::prelude::World, caller: Entity, caster: Entity) -> Self {
        Self {
            world,
            caller,
            caster,
        }
    }

    pub fn trigger<'a, E>(&mut self, event: E)
    where
        E: Event<Trigger<'a>: Default>,
    {
        unsafe {
            (*self.world).trigger(event);
        }
    }

    pub fn turn_player(&mut self) -> Entity {
        unsafe {
            (*self.world)
                .query_filtered::<Entity, With<TurnPlayer>>()
                .single(self.world.as_ref().unwrap())
                .unwrap()
        }
    }
}

impl IsAbstract for ScriptCtx {
    fn type_info() -> &'static janet_bindings::bindings::JanetAbstractType {
        const CONDITION_ATYPE: JanetAbstractType =
            JanetAbstractType::new(c"main/script-cxt", ScriptCtx::gc);
        &CONDITION_ATYPE
    }
}
