use super::{CardBundle, FromRegistry, card_registry::CardRegistry};
use rand::seq::SliceRandom;

pub struct DeckBuilder;

impl DeckBuilder {
    pub fn standard_deck(card_registry: &CardRegistry) -> Vec<CardBundle> {
        let mut deck = Vec::new();
        for card_id in card_registry.registered_ids() {
            let el = CardBundle::from_registry(card_registry, *card_id).unwrap();
            deck.extend(std::iter::repeat_n(el, 4));
        }

        let mut rng = rand::rng();
        deck.shuffle(&mut rng);
        deck
    }
}
