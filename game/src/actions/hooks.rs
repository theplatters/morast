use bevy::ecs::{component::Component, event::EntityEvent, observer::On, system::Commands};

use crate::actions::Execute;

#[derive(Component)]
pub struct Hook<T: HookEvent> {
    pd: std::marker::PhantomData<T>,
}

pub trait HookEvent: EntityEvent + Sized {}

pub fn run_hook<T: HookEvent>(hm: On<T>, mut commands: Commands) {
    commands.entity(hm.event_target()).trigger(Execute);
}
