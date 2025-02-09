use crate::game::events::actions::CardAction;

use super::{
    card::Card,
    deck::Deck,
    events::{event::Event, event_handler::EventHandler, event_manager::EventManager},
};

pub struct Hand {
    pub player: u16,
    pub cards: Vec<Card>,
}

impl Hand {
    pub fn add_card(&mut self, card: Card) {
        self.cards.push(card);
    }

    pub fn remove_card(&mut self) -> Option<Card> {
        self.cards.pop()
    }
}

impl crate::engine::janet_handler::types::janetenum::ToVoidPointer for Hand {}
impl EventHandler for Hand {
    fn handle_event(&mut self, event: &Event) -> Vec<Event> {
        match event {
            Event::SendCardToHand(action) => {
                if action.player == self.player {
                    self.add_card(action.card.to_owned());
                }
                vec![]
            }
            Event::DiscardCard(player) if *player == self.player => {
                if let Some(card) = self.remove_card() {
                    vec![Event::SendCardToDiscard(CardAction {
                        card,
                        player: *player,
                    })]
                } else {
                    vec![Event::HandEmpty(*player)]
                }
            }
            Event::DiscardCard(_)
            | Event::DrawCard(_)
            | Event::SendCardToDiscard(_)
            | Event::CardDrawn(_)
            | Event::DeckEmpty(_)
            | Event::CardDiscarded(_)
            | Event::HandEmpty(_) => Vec::new(),
        }
    }
}
