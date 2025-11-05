use macroquad::math::U16Vec2;

use crate::game::game_objects::player_base::PlayerBase;

use super::{card_on_board::CardOnBoard, effect::Effect};

#[derive(Debug)]
pub struct Tile {
    pub ontile: Option<CardOnBoard>,
    has_gold_mine: bool,
    player_base: Option<PlayerBase>,
    effects: Vec<Effect>,
    pub attack_on_tile: U16Vec2,
}

impl Tile {
    pub fn new() -> Self {
        Self {
            ontile: None,
            has_gold_mine: false,
            player_base: None,
            effects: Vec::new(),
            attack_on_tile: U16Vec2::ZERO,
        }
    }

    pub fn process_effects(&mut self) {
        self.effects.retain(|effect| effect.duration() > 0);
        self.effects
            .iter_mut()
            .for_each(|effect| effect.decrease_duration());
    }

    pub fn place(&mut self, card: CardOnBoard) {
        self.ontile = Some(card);
    }

    pub fn is_occupied(&self) -> bool {
        self.ontile.is_some()
    }

    pub fn add_effect(&mut self, effect: Effect) {
        self.effects.push(effect);
    }

    pub fn has_effects(&self) -> bool {
        !self.effects.is_empty()
    }

    pub(crate) fn remove_effect(&mut self, effect: Effect) {
        self.effects.retain(|&x| x != effect);
    }
}
