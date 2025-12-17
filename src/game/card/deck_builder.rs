use crate::game::card::{card_id::CardID, card_registry::CardRegistry};
use rand::seq::SliceRandom;

pub struct DeckBuilder;

impl DeckBuilder {
    pub fn standard_deck(card_registry: &CardRegistry) -> Vec<CardID> {
        let mut deck = Vec::new();
        for key in card_registry.registered_ids() {
            deck.extend(std::iter::repeat_n(*key, 4));
        }

        let mut rng = rand::rng();
        deck.shuffle(&mut rng);
        deck
    }
}
