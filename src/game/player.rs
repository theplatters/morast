use bevy::{
    asset::AssetServer,
    ecs::{
        bundle::Bundle,
        component::Component,
        entity::Entity,
        hierarchy::ChildOf,
        query::With,
        system::{Commands, Query, Res},
    },
    sprite::Sprite,
    state::commands,
    transform::components::Transform,
};

use crate::game::{
    card::{Card, InDeck, InHand},
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
pub struct Deck(pub Vec<Entity>);

impl Deck {
    fn empty() -> Deck {
        Self(Vec::new())
    }
}

#[derive(Component)]
pub struct Hand(pub Vec<Entity>);

impl Hand {
    fn empty() -> Hand {
        Self(Vec::new())
    }

    pub(crate) fn get_card(&self, card_index: usize) -> Option<Entity> {
        self.0.get(card_index).cloned()
    }
}
#[derive(Component)]
pub struct Graveyard(pub Vec<Entity>);
impl Graveyard {
    fn empty() -> Graveyard {
        Self(Vec::new())
    }
}

#[derive(Bundle)]
pub struct PlayerBundle {
    resources: PlayerResources,
    hand: Hand,
    deck: Deck,
    graveyard: Graveyard,
}

impl Default for PlayerBundle {
    fn default() -> Self {
        Self {
            resources: Default::default(),
            hand: Hand::empty(),
            deck: Deck::empty(),
            graveyard: Graveyard::empty(),
        }
    }
}

pub fn add_player(mut commands: Commands) {
    let player1 = commands
        .spawn((Player { number: 0 }, PlayerBundle::default(), TurnPlayer))
        .id();
    let player2 = commands
        .spawn((Player { number: 1 }, PlayerBundle::default()))
        .id();
}

pub fn draw_card(
    deck: &mut Deck,
    hand: &mut Hand,
    commands: &mut Commands,
    asset_server: &AssetServer,
) {
    let Some(card) = deck.0.pop() else {
        return;
    };

    hand.0.push(card);

    commands
        .entity(card)
        .remove::<InDeck>()
        .insert(InHand)
        .insert((
            Sprite::from_image(asset_server.load("card_frame.png")),
            Transform::from_xyz(200., 200., 200.),
        ));
    commands.write_message(CardsDrawn { card });
}

pub fn draw_starting_cards(
    mut players: Query<(&mut Deck, &mut Hand)>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    for (mut deck, mut hand) in &mut players {
        draw_card(deck.as_mut(), hand.as_mut(), &mut commands, &asset_server);
    }
}
