use bevy::{
    ecs::{
        component::Component,
        entity::Entity,
        hierarchy::ChildOf,
        query::With,
        system::{Commands, Query, Res},
    },
};
use derive_more::From;

use crate::{
    actions::{AbilityData, Action},
    board::tile::Occupant,
    card::{
        card_id::CardID,
        card_registry::CardRegistry,
        creature::{CreatureBundle},
        deck_builder::DeckBuilder,
        spell_card::{SpellBundle},
        trap_card::{TrapBundle},
    },
    components::Owner,
    player::{Deck, Hand, Player},
};

pub mod abilities;
pub mod card_id;
pub mod card_registry;
pub mod card_type;
pub mod creature;
pub mod deck_builder;
pub mod in_play_id;
pub mod spell_card;
pub mod trap_card;

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
#[relationship(relationship_target = crate::player::Graveyard)]
pub struct InGraveyard {
    #[relationship]
    pub owner: Entity,
}

#[derive(Component, Default)]
pub struct Selected;

// ============================================
// MUTABLE INSTANCE STATE (what changes during play)
// ============================================

#[derive(Component, From, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct CurrentAttack(pub u16);

#[derive(Component, From, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct CurrentDefense(pub u16);

#[derive(Component, From, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
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
// Bundles
// ============================================

#[derive(Clone)]
pub enum CardBundle {
    Creature { bundle: CreatureBundle },
    Spell { bundle: SpellBundle },
    Trap { bundle: TrapBundle },
}

impl CardBundle {
    pub fn card_id(&self) -> CardID {
        match self {
            CardBundle::Creature { bundle } => bundle.card_id,
            CardBundle::Spell { bundle } => bundle.card_id,
            CardBundle::Trap { bundle } => bundle.card_id,
        }
    }
}

pub trait FromRegistry: Sized {
    fn from_registry(card_registry: &CardRegistry, card_id: CardID) -> Option<Self>;
}

impl FromRegistry for CardBundle {
    fn from_registry(card_registry: &CardRegistry, card_id: CardID) -> Option<Self> {
        let bundle = match card_registry.get(&card_id)?.kind {
            crate::def::card::CardKindDef::Creature { .. } => CardBundle::Creature {
                bundle: CreatureBundle::from_registry(card_registry, card_id)?,
            },
            crate::def::card::CardKindDef::Spell => CardBundle::Spell {
                bundle: SpellBundle::from_registry(card_registry, card_id)?,
            },
            crate::def::card::CardKindDef::Trap => CardBundle::Trap {
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
            let card_id = bundle.card_id();
            let card_entity = match bundle {
                CardBundle::Creature { bundle } => commands.spawn((bundle, Owner(player), InDeck { parent: player })).id(),
                CardBundle::Spell { bundle } => commands.spawn((bundle, Owner(player), InDeck { parent: player })).id(),
                CardBundle::Trap { bundle } => commands.spawn((bundle, Owner(player), InDeck { parent: player })).id(),
            };

            let Some(def) = card_registry.get(&card_id) else {
                continue;
            };
            // Spawn ability child entities for every triggered ability on the card.
            for ability in &def.abilities {
                commands.spawn((
                    AbilityData(ability.clone()),
                    Action { caster: card_entity },
                    ChildOf(card_entity),
                ));
            }
        }
    }
}
