use crate::engine;

use super::{
    card::Card,
    events::{
        actions::CardAction,
        event::Event,
        event_handler::{self, EventHandler},
        event_manager::EventManager,
    },
    hand::Hand,
};

pub struct Deck {
    pub player: u16,
    pub cards: Vec<Card>,
}
// Implement the draw function
impl Deck {
    pub fn add_card(&mut self, card: Card) {
        self.cards.push(card);
    }

    pub fn remove_card(&mut self) -> Option<Card> {
        self.cards.pop()
    }
}

impl EventHandler for Deck {
    fn handle_event(&mut self, event: &Event) -> Vec<Event> {
        match event {
            Event::DrawCard(player) if *player == self.player => {
                if let Some(card) = self.remove_card() {
                    vec![Event::SendCardToHand(CardAction {
                        card,
                        player: self.player,
                    })]
                } else {
                    vec![Event::DeckEmpty(*player)]
                }
            }
            _ => panic!("Event not implemented for Deck"),
        }
    }
}
