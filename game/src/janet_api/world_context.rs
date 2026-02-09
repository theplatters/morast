use bevy::ecs::{
    entity::Entity,
    event::Event,
    query::With,
    system::{Commands, Query, SystemParam},
    world::EntityRef,
};
use janet_bindings::{bindings::JanetAbstractType, types::janetabstract::IsAbstract};

use crate::{
    board::tile::Tile,
    card::{CreatureCard, SpellCard, TrapCard},
    player::{Player, TurnPlayer},
};

#[derive(SystemParam)]
pub struct ScriptCache<'w, 's> {
    pub creatures: Query<'w, 's, EntityRef<'static>, With<CreatureCard>>,
    pub traps: Query<'w, 's, EntityRef<'static>, With<TrapCard>>,
    pub spells: Query<'w, 's, EntityRef<'static>, With<SpellCard>>,
    pub tiles: Query<'w, 's, EntityRef<'static>, With<Tile>>,
    pub players: Query<'w, 's, EntityRef<'static>, With<Player>>,
}

#[repr(C)]
pub struct ScriptCtx<'w, 's, 'c> {
    cache: &'c ScriptCache<'w, 's>,
    commands: &'c mut Commands<'w, 's>,
}

impl<'w, 's, 'c> ScriptCtx<'w, 's, 'c> {
    pub fn new(commands: &'c mut Commands<'w, 's>, cache: &'c ScriptCache<'w, 's>) -> Self {
        Self { cache, commands }
    }

    pub fn trigger<'e, E>(&mut self, event: E)
    where
        E: Event<Trigger<'e>: Default>,
    {
        self.commands.trigger(event);
    }

    pub(crate) fn turn_player(&self) -> Entity {}
}

impl<'a, 'b, 'c> IsAbstract for ScriptCtx<'a, 'b, 'c> {
    fn type_info() -> &'static janet_bindings::bindings::JanetAbstractType {
        const CONDITION_ATYPE: JanetAbstractType =
            JanetAbstractType::new(c"main/script-cxt", ScriptCtx::gc);
        &CONDITION_ATYPE
    }
}
