use std::cmp;

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
        Self(self.0.wrapping_add(1))
    }
}

#[derive(Debug)]
pub struct Player {
    pub id: PlayerID,
    money: i32,
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
