use std::cmp;

use macroquad::math::clamp;

use super::{
    card::card_id::CardID,
    deck::Deck,
    events::{actions::GoldAction, event::Event, event_handler::EventHandler},
    hand::Hand,
};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct PlayerID(u16);
impl PlayerID {
    // Existing methods
    pub fn new(id: u16) -> Self {
        Self(id)
    }

    pub fn get(&self) -> u16 {
        self.0
    }

    // New next method with overflow protection
    pub fn next(&self) -> Self {
        Self(self.0.wrapping_add(1))
    }
}

#[derive(Debug)]
pub struct Player {
    pub id: PlayerID,
    money: i32,
    deck: Vec<CardID>,
    hand: Vec<CardID>,
    discard_pile: Deck,
}

impl Player {
    pub fn new(id: PlayerID) -> Self {
        Self {
            id,
            money: 0,
            deck: Vec::new(),
            hand: Vec::new(),
            discard_pile: Deck::new(id),
        }
    }

    pub fn add_to_hand(&mut self, card: CardID) {
        self.hand.push(card)
    }

    pub fn add_to_deck_top(&mut self, card: CardID) {
        self.hand.push(card)
    }
    pub fn draw_from_deck(&mut self) -> Option<CardID> {
        self.deck.pop()
    }

    pub fn discard_card(&mut self) {
        self.hand.pop();
    }
    pub fn get_gold(&mut self, amount: i32) {
        self.money = cmp::max(self.money + amount, 0)
    }
}

impl EventHandler for Player {
    fn handle_event(
        &mut self,
        event: &super::events::event::Event,
    ) -> Vec<super::events::event::Event> {
        match event {
            Event::GetGold(GoldAction { player, amount }) => {
                if *player == self.id {
                    self.money += amount;
                }
                Vec::new()
            }
            Event::RequestPlace(place_on_board_action)
                if place_on_board_action.player == self.id =>
            {
                if place_on_board_action.cost > self.money {
                    vec![Event::PlaceRequestDenied]
                } else {
                    self.money -= place_on_board_action.cost;
                    vec![Event::PlaceCard(*place_on_board_action)]
                }
            }
            _ => Vec::new(),
        }
    }
}
