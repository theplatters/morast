use std::collections::HashMap;

use bevy::ecs::{
    resource::Resource,
    system::{NonSend, NonSendMut, ResMut},
};

use super::{
    Card, CardBehavior,
    card_reader::{read_spell, read_trap},
    card_type::CardTypes,
    creature::Creature,
};
use crate::{
    error::GameError,
    janet_api::{core_constants::CORE_CONSTANTS, core_functions::CORE_FUNCTIONS},
};
use janet_bindings::controller::Environment;

use super::{
    card_id::CardID,
    card_reader::{get_card_list, read_creature},
};

#[derive(Debug, Resource)]
pub struct CardRegistry {
    cards: HashMap<CardID, Card>,
    types: HashMap<CardID, CardTypes>,
    id_counter: CardID,
    names: HashMap<String, CardID>,
}

impl CardRegistry {
    pub fn new() -> Self {
        Self {
            cards: HashMap::new(),
            id_counter: CardID::new(0),
            names: HashMap::new(),
            types: HashMap::new(),
        }
    }

    pub fn registered_ids(&self) -> std::collections::hash_map::Keys<'_, CardID, Card> {
        self.cards.keys()
    }

    pub async fn init(&mut self, env: &mut Environment) {}

    pub fn add_card_from_janet(
        &mut self,
        env: &Environment,
        name: &str,
        card_type: CardTypes,
    ) -> Result<CardID, GameError> {
        let card = match card_type {
            CardTypes::Creature => read_creature(env, name)?,
            CardTypes::Spell => read_spell(env, name)?,
            CardTypes::Trap => read_trap(env, name)?,
        };
        self.names.insert(card.name().to_owned(), self.id_counter);
        self.types.insert(self.id_counter, card_type);
        self.cards.insert(self.id_counter, card);
        let current_id = self.id_counter;
        self.id_counter = self.id_counter.next();
        Ok(current_id)
    }

    pub fn get(&self, card_id: &CardID) -> Option<&Card> {
        self.cards.get(card_id)
    }

    pub fn get_type(&self, card_id: &CardID) -> Option<CardTypes> {
        self.types.get(card_id).cloned()
    }

    pub fn get_creature(&self, card_id: &CardID) -> Option<&Creature> {
        let Some(Card::Creature(card)) = self.cards.get(card_id) else {
            return None;
        };
        Some(card)
    }
}

pub fn init_card_registry(
    mut card_registry: ResMut<CardRegistry>,
    environment: NonSendMut<Environment>,
) {
    let (creature_names, spell_names, trap_names) =
        get_card_list(&environment).expect("Could not get list of cards");

    for card in creature_names {
        card_registry
            .add_card_from_janet(&environment, card.as_str(), CardTypes::Creature)
            .expect("Could not read card");
    }

    for card in spell_names {
        card_registry
            .add_card_from_janet(&environment, card.as_str(), CardTypes::Spell)
            .expect("Could not read card");
    }

    for card in trap_names {
        card_registry
            .add_card_from_janet(&environment, card.as_str(), CardTypes::Trap)
            .expect("Could not read card");
    }
}
