use crate::game::{card::Named, game_action::GameAction};

#[derive(Debug)]
pub struct Spell {
    name: String,
    cost: u16,
    on_play: Vec<GameAction>,
}

impl Named for Spell {
    fn name(&self) -> &str {
        &self.name
    }
}

impl Spell {
    pub fn cost(&self) -> u16 {
        self.cost
    }
}
