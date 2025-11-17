use crate::game::{card::Named, game_action::GameAction};

#[derive(Debug)]
pub struct Trap {
    pub name: String,
    pub on_place_action: Vec<GameAction>,
    pub on_reveal_action: Vec<GameAction>,
}

impl Named for Trap {
    fn name(&self) -> &str {
        &self.name
    }
}
