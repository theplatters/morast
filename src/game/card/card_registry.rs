use std::collections::HashMap;

use crate::engine::{
    asset_loader::{self, AssetLoader},
    janet_handler::controller::Environment,
};

use super::{
    card_id::CardID,
    card_reader::{get_card_list, read_card},
    Card,
};

#[derive(Debug)]
pub struct CardRegistry {
    cards: HashMap<CardID, Card>,
    id_counter: CardID,
}

impl CardRegistry {
    pub fn new() -> Self {
        Self {
            cards: HashMap::new(),
            id_counter: CardID::new(0),
        }
    }

    pub async fn add_janet_card(
        &mut self,
        env: &Environment,
        name: &str,
        asset_loader: &mut AssetLoader,
    ) -> Result<CardID, &'static str> {
        let card = read_card(env, name, asset_loader).await?;
        self.cards.insert(self.id_counter, card);
        let current_id = self.id_counter;
        self.id_counter = self.id_counter.next();
        Ok(current_id)
    }
}
