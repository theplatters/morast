use std::collections::HashMap;

use bevy::ecs::{resource::Resource, world::FromWorld};

use crate::{
    engine::{asset_loader::AssetLoader, janet_handler::controller::Environment},
    game::{
        card::{
            card_reader::{read_spell, read_trap},
            card_type::CardTypes,
            creature::Creature,
            Card, CardBehavior,
        },
        error::Error,
    },
};

use super::{
    card_id::CardID,
    card_reader::{get_card_list, read_creature},
};

#[derive(Debug, Resource)]
pub struct CardRegistry {
    cards: HashMap<CardID, Card>,
    id_counter: CardID,
    names: HashMap<String, CardID>,
}

impl CardRegistry {
    pub fn new() -> Self {
        Self {
            cards: HashMap::new(),
            id_counter: CardID::new(0),
            names: HashMap::new(),
        }
    }

    pub fn registered_ids(&self) -> std::collections::hash_map::Keys<'_, CardID, Card> {
        self.cards.keys()
    }

    pub async fn init(&mut self, env: &mut Environment, asset_loader: &mut AssetLoader) {
        let (creature_names, spell_names, trap_names) =
            get_card_list(env).expect("Could not get list of cards");

        for card in creature_names {
            self.add_card_from_janet(env, card.as_str(), asset_loader, CardTypes::Creature)
                .await
                .expect("Could not read card");
        }

        for card in spell_names {
            self.add_card_from_janet(env, card.as_str(), asset_loader, CardTypes::Spell)
                .await
                .expect("Could not read card");
        }

        for card in trap_names {
            self.add_card_from_janet(env, card.as_str(), asset_loader, CardTypes::Trap)
                .await
                .expect("Could not read card");
        }
    }

    pub async fn add_card_from_janet(
        &mut self,
        env: &Environment,
        name: &str,
        asset_loader: &mut AssetLoader,
        card_type: CardTypes,
    ) -> Result<CardID, Error> {
        let card = match card_type {
            CardTypes::Creature => read_creature(env, name, asset_loader).await?,
            CardTypes::Spell => read_spell(env, name, asset_loader).await?,
            CardTypes::Trap => read_trap(env, name, asset_loader).await?,
        };
        self.names.insert(card.name().to_owned(), self.id_counter);
        self.cards.insert(self.id_counter, card);
        let current_id = self.id_counter;
        self.id_counter = self.id_counter.next();
        Ok(current_id)
    }

    pub fn get(&self, card_id: &CardID) -> Option<&Card> {
        self.cards.get(card_id)
    }

    pub fn get_creature(&self, card_id: &CardID) -> Option<&Creature> {
        let Some(Card::Creature(card)) = self.cards.get(card_id) else {
            return None;
        };
        Some(card)
    }
}
