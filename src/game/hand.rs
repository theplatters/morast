use crate::game::events::actions::CardAction;

use super::{
    card::card_holder::CardHolder,
    events::{event::Event, event_handler::EventHandler},
    player::PlayerID,
};

#[derive(Debug)]
pub struct Hand {
    pub player: PlayerID,
    pub cards: CardHolder,
}

impl Hand {
    pub fn new(player: PlayerID) -> Self {
        Self {
            player,
            cards: CardHolder::new(),
        }
    }
}

impl crate::engine::janet_handler::types::janetenum::ToVoidPointer for Hand {}
impl EventHandler for Hand {
    fn handle_event(&mut self, event: &Event) -> Vec<Event> {
        match event {
            Event::SendCardToHand(action) => {
                if action.player == self.player {
                    self.cards.add_card(action.card.to_owned());
                }
                vec![]
            }
            Event::DiscardCard(player) if *player == self.player => {
                if let Some(card) = self.cards.remove_card() {
                    vec![Event::SendCardToDiscard(CardAction {
                        card,
                        player: *player,
                    })]
                } else {
                    vec![Event::HandEmpty(*player)]
                }
            }
            _ => Vec::new(),
        }
    }
}
