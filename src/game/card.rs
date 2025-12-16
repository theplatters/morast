use bevy::ecs::component::Component;
use bevy::ecs::entity::Entity;
use bevy::math::{I16Vec2, U16Vec2};
use std::slice::Iter;

use crate::game::card::creature::Creature;
use crate::game::card::spell_card::Spell;
use crate::game::card::trap_card::Trap;

pub mod abilities;
pub mod card_builder;
pub mod card_id;
pub mod card_reader;
pub mod card_registry;
pub mod card_type;
pub mod creature;
pub mod deck_builder;
pub mod in_play_id;
pub mod spell_card;
pub mod trap_card;

#[derive(Debug)]
pub enum Card {
    Creature(Creature),
    Spell(Spell),
    Trap(Trap),
}

#[derive(Component)]
pub struct CreatureCard;

#[derive(Component)]
pub struct SpellCard;

#[derive(Component)]
pub struct TrapCard;

// ============================================
// LOCATION COMPONENTS (instance-specific)
// ============================================

#[derive(Component)]
pub struct InDeck;

#[derive(Component)]
pub struct InHand {
    pub owner: Entity,
    pub hand_index: usize,
}

#[derive(Component)]
pub struct OnBoard {
    pub position: U16Vec2,
}

#[derive(Component)]
pub struct InGraveyard {
    pub owner: Entity,
    pub order: usize,
}

#[derive(Component, Default)]
pub struct Selected;

// ============================================
// IMMUTABLE INSTANCE STATE (what changes during play)
// ============================================
#[derive(Component)]
pub struct BaseAttack(pub u16);

#[derive(Component)]
pub struct BaseDefense(pub u16);

#[derive(Component)]
pub struct BaseMovement(pub u16);

#[derive(Component)]
pub struct AttackPattern(pub Vec<I16Vec2>);

impl<'a> IntoIterator for &'a AttackPattern {
    type Item = &'a I16Vec2;
    type IntoIter = Iter<'a, I16Vec2>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

#[derive(Component)]
pub struct MovementPattern(pub Vec<I16Vec2>);

impl<'a> IntoIterator for &'a MovementPattern {
    type Item = &'a I16Vec2;
    type IntoIter = Iter<'a, I16Vec2>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

// ============================================
// MUTABLE INSTANCE STATE (what changes during play)
// ============================================

#[derive(Component)]
pub struct CurrentAttack {
    pub value: u16,
}

#[derive(Component)]
pub struct CurrentDefense {
    pub value: u16,
}
/// Current movement state
#[derive(Component)]
pub struct CurrentMovementPoints {
    pub remaining_points: u16,
}

pub trait CardBehavior {
    fn cost(&self) -> u16;
    fn description(&self) -> &str;
    fn name(&self) -> &str;
    fn display_image_asset_string(&self) -> &str;
    // Add other common methods here
}

impl Card {
    fn can_be_placed(&self) -> bool {
        matches!(self, Card::Creature(_) | Card::Trap(_))
    }
}

impl CardBehavior for Card {
    fn cost(&self) -> u16 {
        match self {
            Card::Creature(c) => c.cost(),
            Card::Spell(c) => c.cost(),
            Card::Trap(c) => c.cost(),
        }
    }

    fn description(&self) -> &str {
        match self {
            Card::Creature(c) => c.description(),
            Card::Spell(c) => c.description(),
            Card::Trap(c) => c.description(),
        }
    }
    fn name(&self) -> &str {
        match self {
            Card::Creature(c) => c.name(),
            Card::Spell(c) => c.name(),
            Card::Trap(c) => c.name(),
        }
    }

    fn display_image_asset_string(&self) -> &str {
        match self {
            Card::Creature(c) => c.display_image_asset_string(),
            Card::Spell(c) => c.display_image_asset_string(),
            Card::Trap(c) => c.display_image_asset_string(),
        }
    }
}
