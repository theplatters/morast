use std::iter;

use bevy::{
    asset::AssetServer,
    ecs::{
        bundle::Bundle,
        component::Component,
        entity::{self, Entity},
        hierarchy::ChildOf,
        query::With,
        relationship::{OrderedRelationshipSourceCollection, RelationshipTarget},
        system::{Commands, Query, Res},
    },
    log::info,
    sprite::Sprite,
    state::commands,
    transform::components::Transform,
};

use crate::game::{
    card::{Card, InDeck, InGraveyard, InHand},
    events::CardsDrawn,
};

#[derive(Component, Clone, Copy, Debug, PartialEq, Eq)]
pub struct Player {
    pub number: u8,
}

/// Player's resources
#[derive(Component)]
pub struct PlayerResources {
    pub health: u16,
    pub max_health: u16,
    pub gold: u16,
}

#[derive(Component)]
pub struct DeckSize(pub usize);

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
#[derive(Component)]
pub struct TurnPlayer;

#[derive(Component)]
#[relationship_target(relationship = InDeck)]
pub struct Deck(Vec<Entity>);

impl Deck {
    fn empty() -> Deck {
        Self(Vec::new())
    }
}

#[derive(Component)]
#[relationship_target(relationship = InHand)]
pub struct Hand(Vec<Entity>);

impl Hand {
    fn empty() -> Hand {
        Self(Vec::new())
    }

    pub(crate) fn get_card(&self, card_index: usize) -> Option<Entity> {
        self.0.get(card_index).cloned()
    }
}

#[derive(Component)]
#[relationship_target(relationship = InGraveyard)]
pub struct Graveyard(Vec<Entity>);

impl Graveyard {
    fn empty() -> Graveyard {
        Self(Vec::new())
    }
}

#[derive(Bundle, Default)]
pub struct PlayerBundle {
    resources: PlayerResources,
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
