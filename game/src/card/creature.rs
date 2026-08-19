use std::slice::Iter;

use bevy::{
    ecs::{bundle::Bundle, component::Component, name::Name},
    log::warn,
    math::{I16Vec2, U16Vec2},
};
use derive_more::From;

use crate::{
    board::tile::Position,
    card::{
        Cost, CreatureCard, CurrentAttack, CurrentDefense, CurrentMovementPoints, FromRegistry,
        abilities::CardAbilities, card_id::CardID, card_registry::CardRegistry,
    },
    components::Health,
    def::card::CardKindDef,
};

#[derive(Component, From, Clone, Copy, Debug)]
pub struct BaseAttack(pub u16);

#[derive(Component, From, Clone, Copy, Debug)]
pub struct BaseDefense(pub u16);

#[derive(Component, From, Clone, Copy, Debug)]
pub struct BaseMovementPoints(pub u16);

#[derive(Component, From, Clone, Debug)]
pub struct AttackPattern(pub Vec<I16Vec2>);

impl AttackPattern {
    pub(crate) fn into_tiles(&self, pos: &Position) -> Vec<U16Vec2> {
        let mut tiles = Vec::new();
        for rel_pos in &self.0 {
            if let Some(tile) = pos.0.checked_add_signed(*rel_pos) {
                tiles.push(tile);
            };
        }

        tiles
    }
}

#[derive(Component)]
pub struct Attacks(pub Vec<U16Vec2>);

impl<'a> IntoIterator for &'a AttackPattern {
    type Item = &'a I16Vec2;
    type IntoIter = Iter<'a, I16Vec2>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

#[derive(Component, From, Clone, Debug)]
pub struct MovementPattern(pub Vec<I16Vec2>);

impl<'a> IntoIterator for &'a MovementPattern {
    type Item = &'a I16Vec2;
    type IntoIter = Iter<'a, I16Vec2>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

#[derive(Bundle, Clone)]
pub struct CreatureBundle {
    pub card_id: CardID,
    pub name: Name,
    pub cost: Cost,
    pub base_attack: BaseAttack,
    pub base_defense: BaseDefense,
    pub current_attack: CurrentAttack,
    pub current_defense: CurrentDefense,
    pub health: Health,
    pub base_movement_points: BaseMovementPoints,
    pub current_movement_points: CurrentMovementPoints,
    pub attack_pattern: AttackPattern,
    pub movement_pattern: MovementPattern,
    pub type_identifier: CreatureCard,
    pub abilities: CardAbilities,
}

impl FromRegistry for CreatureBundle {
    fn from_registry(card_registry: &CardRegistry, card_id: CardID) -> Option<Self> {
        let Some(def) = card_registry.get(&card_id) else {
            warn!("Card Id {} not found", card_id);
            return None;
        };
        let CardKindDef::Creature(stats) = &def.kind else {
            warn!("Card Id {} is not a creature", card_id);
            return None;
        };

        Some(Self {
            card_id,
            name: def.name.as_str().into(),
            cost: def.cost.into(),
            base_attack: stats.attack.into(),
            base_defense: stats.defense.into(),
            current_attack: stats.attack.into(),
            current_defense: stats.defense.into(),
            health: Health(stats.defense),
            base_movement_points: stats.movement_points.into(),
            current_movement_points: stats.movement_points.into(),
            attack_pattern: Vec::<I16Vec2>::from(&stats.attack_pattern).into(),
            movement_pattern: Vec::<I16Vec2>::from(&stats.movement).into(),
            type_identifier: CreatureCard,
            abilities: CardAbilities(stats.abilities.clone()),
        })
    }
}
