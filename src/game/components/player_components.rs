use bevy::ecs::{component::Component, entity::Entity};

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

#[derive(Component)]
pub struct Graveyard(pub Vec<Entity>);
