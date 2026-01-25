use bevy::ecs::world::World;
use janet_bindings::{
    bindings::{Janet, JanetAbstractType},
    types::{function::JFunction, janetabstract::IsAbstract},
};

pub struct ScriptCtx<'w> {
    world: &'w mut World,
}

pub enum Callback<'w, Res, Ret> {
    RustFun(Box<dyn FnMut(&'w World, Res) -> Ret>),
    JanetFun(JFunction),
}

impl<'w> ScriptCtx<'w> {
    pub fn new(world: &'w mut bevy::prelude::World) -> Self {
        Self { world }
    }
}

impl<'w> IsAbstract for ScriptCtx<'w> {
    fn type_info() -> &'static janet_bindings::bindings::JanetAbstractType {
        const CONDITION_ATYPE: JanetAbstractType =
            JanetAbstractType::new(c"main/script-cxt", ScriptCtx::gc);
        &CONDITION_ATYPE
    }
}
