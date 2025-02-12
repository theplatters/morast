use super::card_id::CardID;

#[derive(Debug)]
pub struct CardHolder {
    cards: Vec<CardID>,
}

impl CardHolder {
    pub fn new() -> Self {
        Self { cards: Vec::new() }
    }

    pub fn add_card(&mut self, card: CardID) {
        self.cards.push(card);
    }

    pub fn remove_card(&mut self) -> Option<CardID> {
        self.cards.pop()
    }
}
