use macroquad::math::U16Vec2;

use super::{card_on_board::CardOnBoard, effect::Effect};

#[derive(Debug)]
pub struct Tile {
    pub ontile: Option<CardOnBoard>,
    effects: Vec<Effect>,
    pub attack_on_tile: U16Vec2,
}

impl Tile {
    pub fn new() -> Self {
        Self {
            ontile: None,
            effects: Vec::new(),
            attack_on_tile: U16Vec2::ZERO,
        }
    }

    pub fn place(&mut self, card: CardOnBoard) {
        self.ontile = Some(card);
    }

    pub fn is_occupied(&self) -> bool {
        self.ontile != None
    }

    pub fn add_effect(&mut self, effect: Effect) {
        self.effects.push(effect);
    }

    pub fn has_effects(&self) -> bool {
        self.effects.is_empty()
    }

    pub(crate) fn remove_effect(&mut self, effect: Effect) {
        self.effects.retain(|&x| x != effect);
    }
}
