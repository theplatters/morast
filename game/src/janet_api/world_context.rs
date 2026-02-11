use bevy::ecs::{
    entity::{ContainsEntity, Entity},
    event::Event,
    query::With,
    system::{Commands, Query, Single, SystemParam},
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
    pub turn_player: Single<'w, 's, Entity, With<TurnPlayer>>,
}

#[repr(C)]
pub struct ScriptCtx<'w, 's, 'w1, 's1, 'c> {
    cache: &'c ScriptCache<'w, 's>,
    commands: &'c mut Commands<'w1, 's1>,
    caster: Entity,
    calling_action: Entity,
}

impl<'w, 's, 'w1, 's1, 'c> ScriptCtx<'w, 's, 'w1, 's1, 'c> {
    pub fn new(
        commands: &'c mut Commands<'w1, 's1>,
        cache: &'c ScriptCache<'w, 's>,
        calling_action: Entity,
        caster: Entity,
    ) -> Self {
        Self {
            cache,
            commands,
            calling_action,
            caster,
        }
    }

    pub fn trigger<'e, E>(&mut self, event: E)
    where
        E: Event<Trigger<'e>: Default>,
    {
        self.commands.trigger(event);
    }

    pub(crate) fn turn_player(&self) -> Entity {
        self.cache.turn_player.entity()
    }
}

impl<'w, 's, 'w1, 's1, 'c> IsAbstract for ScriptCtx<'w, 's, 'w1, 's1, 'c> {
    fn type_info() -> &'static janet_bindings::bindings::JanetAbstractType {
        const CONDITION_ATYPE: JanetAbstractType =
            JanetAbstractType::new(c"main/script-cxt", ScriptCtx::gc);
        &CONDITION_ATYPE
    }
}
