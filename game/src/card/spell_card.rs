use bevy::{
    ecs::{bundle::Bundle, name::Name},
    log::warn,
};

use crate::{
    actions::GameAction,
    card::{Card, CardBehavior, Cost, FromRegistry, card_id::CardID, card_registry::CardRegistry},
};

#[derive(Debug)]
pub struct Spell {
    name: String,
    cost: u16,
    description: String,
    on_play_action: GameAction,
    actions: Vec<GameAction>,
    display_image_asset_string: String,
}

impl CardBehavior for Spell {
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

impl Spell {
    pub fn cost(&self) -> u16 {
        self.cost
    }

    pub fn new(
        name: String,
        description: String,
        cost: u16,
        on_play_action: GameAction,
        actions: Vec<GameAction>,
        display_image_asset_string: String,
    ) -> Self {
        Self {
            name: name.to_owned(),
            description,
            cost,
            on_play_action,
            actions,
            display_image_asset_string,
        }
    }
}

#[derive(Bundle, Clone)]
pub struct SpellBundle {
    pub card_id: CardID,
    pub name: Name,
    pub cost: Cost,
}

impl FromRegistry for SpellBundle {
    fn from_registry(card_registry: &CardRegistry, card_id: CardID) -> Option<Self> {
        let Some(Card::Spell(card)): Option<&super::Card> = card_registry.get(&card_id) else {
            warn!("Card Id {} not found", card_id);
            return None;
        };

        Some(Self {
            card_id,
            name: card.name().into(),
            cost: card.cost().into(),
        })
    }
}
