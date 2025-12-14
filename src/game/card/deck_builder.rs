use macroquad::rand::ChooseRandom;

use crate::game::card::{card_id::CardID, card_registry::CardRegistry};

pub struct DeckBuilder;

impl DeckBuilder {
    pub fn standard_deck(card_registry: &CardRegistry) -> Vec<CardID> {
        let mut deck = Vec::new();
        for key in card_registry.registered_ids() {
            deck.extend(std::iter::repeat_n(*key, 4));
        }
        deck.shuffle();
        deck
    }
}

