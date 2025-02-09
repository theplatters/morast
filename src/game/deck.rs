use crate::engine;

use super::{
    card::card_holder::CardHolder,
    events::{actions::CardAction, event::Event, event_handler::EventHandler},
    player::PlayerID,
};

#[derive(Debug)]
pub struct Deck {
    pub player: PlayerID,
    pub cards: CardHolder,
}
// Implement the draw function
impl Deck {
    pub fn new(player: PlayerID) -> Self {
        Self {
            player,
            cards: CardHolder::new(),
        }
    }
}

impl EventHandler for Deck {
    fn handle_event(&mut self, event: &Event) -> Vec<Event> {
        match event {
            Event::DrawCard(player) if *player == self.player => {
                if let Some(card) = self.cards.remove_card() {
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
