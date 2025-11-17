use crate::game::{card::Named, game_action::GameAction};

#[derive(Debug)]
pub struct Spell {
    name: String,
    on_play: Vec<GameAction>,
}

impl Named for Spell {
    fn name(&self) -> &str {
        &self.name
    }
}
