use std::cmp;

use macroquad::rand::ChooseRandom;

use super::card::card_id::CardID;

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
        Self((self.0 + 1) % 2)
    }
}

#[derive(Debug)]
pub struct Player {
    pub id: PlayerID,
    money: i64,
    deck: Vec<CardID>,
    hand: Vec<CardID>,
    discard_pile: Vec<CardID>,
}

impl Player {
    pub fn new(id: PlayerID) -> Self {
        Self {
            id,
            money: 0,
            deck: Vec::new(),
            hand: Vec::new(),
            discard_pile: Vec::new(),
        }
    }

    pub fn add_to_hand(&mut self, card: CardID) {
        self.hand.push(card)
    }

    pub fn shuffle_deck(&mut self) {
        self.deck.shuffle();
    }

    pub fn add_to_deck_top(&mut self, card: CardID) {
        self.hand.push(card)
    }
    pub fn draw_from_deck(&mut self) -> Option<CardID> {
        self.deck.pop()
    }

    pub fn discard_card(&mut self) {
        self.hand.shuffle();
        let Some(card) = self.hand.pop() else {
            return;
        };
        self.discard_pile.push(card);
    }
    pub fn add_gold(&mut self, amount: i64) {
        self.money = cmp::max(self.money + amount, 0)
    }

    pub fn get_gold(&self) -> i64 {
        self.money
    }
}
