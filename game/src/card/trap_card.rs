use bevy::{
    ecs::{bundle::Bundle, name::Name},
    log::warn,
};

use crate::{
    actions::GameAction,
    card::{Card, CardBehavior, Cost, FromRegistry, card_id::CardID, card_registry::CardRegistry},
};

#[derive(Debug)]
pub struct Trap {
    name: String,
    description: String,
    cost: u16,
    display_image_asset_string: String,
    actions: Vec<GameAction>,
    on_reveal_action: GameAction,
}

impl CardBehavior for Trap {
    fn cost(&self) -> u16 {
        self.cost
    }

    fn description(&self) -> &str {
        &self.description
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn display_image_asset_string(&self) -> &str {
        &self.display_image_asset_string
    }
}

impl Trap {
    pub fn new(
        name: String,
        cost: u16,
        description: String,
        display_image_asset_string: String,
        actions: Vec<GameAction>,
        on_reveal_action: GameAction,
    ) -> Self {
        Self {
            name,
            description,
            cost,
            display_image_asset_string,
            actions,
            on_reveal_action,
        }
    }
}

#[derive(Bundle, Clone)]
pub struct TrapBundle {
    pub card_id: CardID,
    pub name: Name,
    pub cost: Cost,
}

impl FromRegistry for TrapBundle {
    fn from_registry(card_registry: &CardRegistry, card_id: CardID) -> Option<Self> {
        let Some(Card::Trap(card)): Option<&super::Card> = card_registry.get(&card_id) else {
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
