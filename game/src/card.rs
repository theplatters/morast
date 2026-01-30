use bevy::ecs::component::Component;
use bevy::ecs::entity::Entity;
use bevy::ecs::query::With;
use bevy::ecs::system::{Commands, Query, Res};
use derive_more::From;

use crate::actions::GameAction;
use crate::board::tile::Occupant;
use crate::card::card_id::CardID;
use crate::card::card_registry::CardRegistry;
use crate::card::creature::{Creature, CreatureBundle};
use crate::card::deck_builder::DeckBuilder;
use crate::card::spell_card::{Spell, SpellBundle};
use crate::card::trap_card::{Trap, TrapBundle};
use crate::components::Owner;
use crate::player::{Deck, Graveyard, Hand, Player};

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

#[derive(Component, Clone, Copy, Debug, PartialEq, Eq)]
pub struct CreatureCard;

#[derive(Component, Clone, Copy, Debug, PartialEq, Eq)]
pub struct SpellCard;

#[derive(Component, Clone, Copy, Debug, PartialEq, Eq)]
pub struct TrapCard;

// ============================================
// LOCATION COMPONENTS (instance-specific)
// ============================================

#[derive(Component, Debug, Clone)]
#[relationship(relationship_target = Deck)]
pub struct InDeck {
    #[relationship]
    pub parent: Entity,
}

#[derive(Component, Debug, Clone)]
#[relationship(relationship_target = Hand)]
pub struct InHand {
    #[relationship]
    pub parent: Entity,
}

#[derive(Component, Debug, Clone)]
#[relationship(relationship_target = Occupant)]
pub struct OnBoard {
    #[relationship]
    pub position: Entity,
}

#[derive(Component, Debug, Clone)]
#[relationship(relationship_target = Graveyard)]
pub struct InGraveyard {
    #[relationship]
    pub owner: Entity,
}

#[derive(Component, Default)]
pub struct Selected;

// ============================================
// MUTABLE INSTANCE STATE (what changes during play)
// ============================================

#[derive(Component, Clone, Copy, From, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct CurrentAttack(pub u16);

#[derive(Component, Clone, Copy, From, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct CurrentDefense(pub u16);
/// Current movement state
#[derive(Component, Clone, Copy, From, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct CurrentMovementPoints(pub u16);

#[derive(Component, Clone, Debug)]
pub struct Cost {
    pub value: u16,
}

impl From<u16> for Cost {
    fn from(value: u16) -> Self {
        Self { value }
    }
}

// ============================================
// Card Traits
// ============================================

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

// ============================================
// Bundles
// ============================================

#[derive(Clone)]
pub enum CardBundle {
    Creature { bundle: CreatureBundle },
    Spell { bundle: SpellBundle },
    Trap { bundle: TrapBundle },
}

pub trait FromRegistry: Sized {
    fn from_registry(card_registry: &CardRegistry, card_id: CardID) -> Option<Self>;
}

impl FromRegistry for CardBundle {
    fn from_registry(card_registry: &CardRegistry, card_id: CardID) -> Option<Self> {
        let bundle = match card_registry.get_type(&card_id)? {
            card_type::CardTypes::Creature => CardBundle::Creature {
                bundle: CreatureBundle::from_registry(card_registry, card_id)?,
            },
            card_type::CardTypes::Spell => CardBundle::Spell {
                bundle: SpellBundle::from_registry(card_registry, card_id)?,
            },
            card_type::CardTypes::Trap => CardBundle::Trap {
                bundle: TrapBundle::from_registry(card_registry, card_id)?,
            },
        };
        Some(bundle)
    }
}

pub fn add_cards(
    card_registry: Res<CardRegistry>,
    players: Query<Entity, With<Player>>,
    mut commands: Commands,
) {
    for player in players {
        for bundle in DeckBuilder::standard_deck(&card_registry) {
            match bundle {
                CardBundle::Creature { bundle } => {
                    commands.spawn((bundle, Owner(player), InDeck { parent: player }));
                }
                CardBundle::Spell { bundle } => {
                    commands.spawn((bundle, Owner(player), InDeck { parent: player }));
                }
                CardBundle::Trap { bundle } => {
                    commands.spawn((bundle, Owner(player), InDeck { parent: player }));
                }
            }
        }
    }
}
