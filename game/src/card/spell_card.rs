use bevy::{
    ecs::{bundle::Bundle, name::Name},
    log::warn,
};

use crate::{
    card::{Cost, FromRegistry, SpellCard, card_id::CardID, card_registry::CardRegistry},
    def::card::CardKindDef,
};

#[derive(Bundle, Clone)]
pub struct SpellBundle {
    pub card_id: CardID,
    pub name: Name,
    pub cost: Cost,
    pub type_identifier: SpellCard,
}

impl FromRegistry for SpellBundle {
    fn from_registry(card_registry: &CardRegistry, card_id: CardID) -> Option<Self> {
        let Some(def) = card_registry.get(&card_id) else {
            warn!("Card Id {} not found", card_id);
            return None;
        };
        let CardKindDef::Spell = &def.kind else {
            warn!("Card Id {} is not a spell", card_id);
            return None;
        };

        Some(Self {
            card_id,
            name: def.name.as_str().into(),
            cost: def.cost.into(),
            type_identifier: SpellCard,
        })
    }
}
