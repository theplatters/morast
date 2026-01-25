use std::slice::Iter;

use bevy::{
    ecs::{bundle::Bundle, component::Component, name::Name},
    log::warn,
    math::{I16Vec2, U16Vec2},
};
use derive_more::From;

use crate::{
    actions::GameAction,
    board::tile::Position,
    card::{
        Card, CardBehavior, Cost, CreatureCard, CurrentAttack, CurrentDefense,
        CurrentMovementPoints, FromRegistry, Playable, abilities::Abilities, card_id::CardID,
        card_registry::CardRegistry,
    },
};

#[derive(Debug)]
pub struct Creature {
    pub name: String,
    pub movement: Vec<I16Vec2>,
    pub movement_points: u16,
    pub attack: Vec<I16Vec2>,
    pub attack_strength: u16,
    pub defense: u16,
    pub cost: u16,
    on_play_action: Option<GameAction>,
    pub turn_begin_action: Option<GameAction>,
    pub turn_end_action: Option<GameAction>,
    pub draw_action: Option<GameAction>,
    pub discard_action: Option<GameAction>,
    pub abilities: Vec<Abilities>,
    pub description: String,
    pub display_image_asset_string: String,
}

impl CardBehavior for Creature {
    fn name(&self) -> &str {
        &self.name
    }

    fn cost(&self) -> u16 {
        self.cost
    }

    fn description(&self) -> &str {
        &self.description
    }

    fn display_image_asset_string(&self) -> &str {
        &self.display_image_asset_string
    }
}

impl Creature {
    pub fn new(
        name: String,
        movement: Vec<I16Vec2>,
        movement_points: u16,
        attack: Vec<I16Vec2>,
        attack_strength: u16,
        defense: u16,
        cost: u16,
        play_action: Option<GameAction>,
        turn_begin_action: Option<GameAction>,
        turn_end_action: Option<GameAction>,
        draw_action: Option<GameAction>,
        discard_action: Option<GameAction>,
        abilities: Vec<Abilities>,
        description: String,
        display_image_asset_string: String,
    ) -> Self {
        Self {
            name,
            movement,
            movement_points,
            attack,
            attack_strength,
            defense,
            cost,
            on_play_action: play_action,
            turn_begin_action,
            turn_end_action,
            draw_action,
            discard_action,
            abilities,
            description,
            display_image_asset_string,
        }
    }

    // Helper method to reduce code duplication

    // Only keep getters for computed properties or when you need different return types
    pub fn total_stats(&self) -> u16 {
        self.attack_strength + self.defense
    }

    pub fn is_powerful(&self) -> bool {
        self.attack_strength > 5 || self.defense > 5
    }
}

impl Playable for Creature {
    fn on_play_action(&self) -> Option<&GameAction> {
        self.on_play_action.as_ref()
    }
}

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
    pub base_movement_points: BaseMovementPoints,
    pub current_movement_points: CurrentMovementPoints,
    pub attack_pattern: AttackPattern,
    pub movement_pattern: MovementPattern,
    pub type_identifier: CreatureCard,
}

impl FromRegistry for CreatureBundle {
    fn from_registry(card_registry: &CardRegistry, card_id: CardID) -> Option<Self> {
        let Some(Card::Creature(card)) = card_registry.get(&card_id) else {
            warn!("Card Id {} not found", card_id);
            return None;
        };

        Some(Self {
            card_id,
            name: card.name().into(),
            cost: card.cost().into(),
            base_attack: card.attack_strength.into(),
            base_defense: card.defense.into(),
            current_attack: card.attack_strength.into(),
            current_defense: card.defense.into(),
            base_movement_points: card.movement_points.into(),
            current_movement_points: card.movement_points.into(),
            attack_pattern: card.attack.clone().into(),
            movement_pattern: card.movement.clone().into(),
            type_identifier: CreatureCard,
        })
    }
}
