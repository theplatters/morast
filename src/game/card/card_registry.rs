use std::collections::HashMap;

use crate::engine::{asset_loader::AssetLoader, janet_handler::controller::Environment};

use super::{
    card_id::CardID,
    card_reader::{get_card_list, read_card},
    Card,
};

#[derive(Debug)]
pub struct CardRegistry {
    cards: HashMap<CardID, Card>,
    id_counter: CardID,
    names: HashMap<String, CardID>,
}

impl CardRegistry {
    pub async fn new(env: &mut Environment, asset_loader: &mut AssetLoader) -> Self {
        let mut s = Self {
            cards: HashMap::new(),
            id_counter: CardID::new(0),
            names: HashMap::new(),
        };
        s.init(env, asset_loader).await;
        s
    }

    pub async fn init(&mut self, env: &mut Environment, asset_loader: &mut AssetLoader) {
        let cards = get_card_list(env).expect("Could not get list of cards");
        for card in cards {
            self.add_janet_card(env, card.as_str(), asset_loader)
                .await
                .expect("Could not read card");
        }
    }

    pub async fn add_janet_card(
        &mut self,
        env: &Environment,
        name: &str,
        asset_loader: &mut AssetLoader,
    ) -> Result<CardID, &'static str> {
        let card = read_card(env, name, asset_loader).await?;
        self.names.insert(card.name.clone(), self.id_counter);
        self.cards.insert(self.id_counter, card);
        let current_id = self.id_counter;
        self.id_counter = self.id_counter.next();
        Ok(current_id)
    }

    pub fn get(&self, card_id: &CardID) -> Option<&Card> {
        self.cards.get(card_id)
    }
}
