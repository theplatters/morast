use std::collections::HashMap;

use bevy::ecs::resource::Resource;

use crate::def::card::CardDef;

use super::card_id::CardID;

#[derive(Debug, Resource)]
pub struct CardRegistry {
    cards: HashMap<CardID, CardDef>,
    names: HashMap<String, CardID>,
    id_counter: CardID,
}

impl CardRegistry {
    pub fn new() -> Self {
        Self {
            cards: HashMap::new(),
            names: HashMap::new(),
            id_counter: CardID::new(0),
        }
    }

    pub fn insert(&mut self, id: CardID, def: CardDef) {
        self.names.insert(def.name.clone(), id);
        self.cards.insert(id, def);
        if id.value() >= self.id_counter.value() {
            self.id_counter = CardID::new(id.value() + 1);
        }
    }

    pub fn get(&self, card_id: &CardID) -> Option<&CardDef> {
        self.cards.get(card_id)
    }

    pub fn id_of_name(&self, name: &str) -> Option<CardID> {
        self.names.get(name).copied()
    }

    pub fn next_id(&self) -> CardID {
        self.id_counter
    }

    pub fn all_ids(&self) -> Vec<CardID> {
        let mut ids: Vec<_> = self.cards.keys().copied().collect();
        ids.sort();
        ids
    }

    pub fn registered_ids(&self) -> std::collections::hash_map::Keys<'_, CardID, CardDef> {
        self.cards.keys()
    }
}

impl Default for CardRegistry {
    fn default() -> Self {
        Self::new()
    }
}
