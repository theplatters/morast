use bevy::ecs::{component::Component, entity::Entity};
use macroquad::math::{I16Vec2, U16Vec2};
use std::ops::{Deref, DerefMut};
use std::slice::{Iter, IterMut};

// ============================================
// CARD TYPE MARKERS (still useful for queries)
// ============================================

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

// ============================================
// IMMUTABLE INSTANCE STATE (what changes during play)
// ============================================
#[derive(Component)]
pub struct BaseAttack(pub u16);

#[derive(Component)]
pub struct BaseDefense(pub u16);

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
pub struct MovementState {
    pub remaining_points: u16,
}
