use bevy::ecs::{
    bundle::Bundle,
    component::Component,
    entity::Entity,
    relationship::RelationshipTarget,
    system::{Commands, Query},
};

use crate::card::{InDeck, InGraveyard, InHand};

#[derive(Component, Clone, Copy, Debug, PartialEq, Eq)]
pub struct Player {
    pub number: u8,
}

/// Player's resources
#[derive(Component, Debug)]
pub struct PlayerResources {
    pub health: u16,
    pub max_health: u16,
    pub gold: u16,
}

impl Default for PlayerResources {
    fn default() -> Self {
        Self {
            health: 10,
            max_health: 10,
            gold: 10,
        }
    }
}

/// Tracks whose turn it is
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct TurnPlayer;

#[derive(Component, Debug, Clone, PartialEq, Eq, Default)]
#[relationship_target(relationship = InDeck)]
pub struct Deck(Vec<Entity>);

#[derive(Component, Debug, Clone, PartialEq, Eq, Default)]
#[relationship_target(relationship = InHand)]
pub struct Hand(Vec<Entity>);

impl Hand {
    pub(crate) fn get_card(&self, card_index: usize) -> Option<Entity> {
        self.0.get(card_index).cloned()
    }
}

#[derive(Component, Debug, Clone, PartialEq, Eq, Default)]
#[relationship_target(relationship = InGraveyard)]
pub struct Graveyard(Vec<Entity>);

#[derive(Bundle, Default)]
pub struct PlayerBundle {
    resources: PlayerResources,
    deck: Deck,
    hand: Hand,
    graveyard: Graveyard,
}

pub fn add_player(mut commands: Commands) {
    commands.spawn((Player { number: 0 }, PlayerBundle::default(), TurnPlayer));
    commands.spawn((Player { number: 1 }, PlayerBundle::default()));
}

pub fn draw_starting_cards(mut players: Query<(&mut Deck, Entity)>, mut commands: Commands) {
    for (deck, player) in &mut players {
        for card in deck.iter().take(5) {
            commands
                .entity(card)
                .remove::<InDeck>()
                .insert(InHand { parent: player });
        }
    }
}
