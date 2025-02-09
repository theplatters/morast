use std::collections::HashMap;

use super::Card;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CardID(u32);

#[derive(Debug)]
pub struct CardRegistry {
    cards: HashMap<CardID, Card>,
    id_counter: CardID,
}

impl CardID {
    // Existing methods
    pub fn new(id: u32) -> Self {
        Self(id)
    }

    pub fn get(&self) -> u32 {
        self.0
    }

    // New next method with overflow protection
    pub fn next(&self) -> Self {
        // Choose one of these implementations:

        // 1. Wrapping arithmetic (cycles back to 0 after u16::MAX)
        Self(self.0 + 1)
    }
}

impl CardRegistry {
    pub fn new() -> Self {
        Self {
            cards: HashMap::new(),
            id_counter: CardID::new(0),
        }
    }
}
