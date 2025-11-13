use std::fmt::Display;

use macroquad::math::U16Vec2;

use crate::game::{
    card::in_play_id::InPlayID, game_objects::player_base::PlayerBase, player::PlayerID,
};

use super::effect::Effect;

#[derive(Debug)]
pub struct Tile {
    pub ontile: Option<InPlayID>,
    player_base: Option<PlayerBase>,
    effects: Vec<Effect>,
    pub attack_on_tile: U16Vec2,
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Tile {{ ")?;

        // Display ontile
        match &self.ontile {
            Some(id) => write!(f, "ontile: Some({}), ", id)?,
            None => write!(f, "ontile: None, ")?,
        }

        // Display player_base
        match &self.player_base {
            Some(base) => write!(f, "has player base")?,
            None => write!(f, "player_base: None, ")?,
        }

        // Display attack_on_tile
        write!(f, "attack_on_tile: {} }}", self.attack_on_tile)
    }
}

impl Tile {
    pub fn new() -> Self {
        Self {
            ontile: None,
            player_base: None,
            effects: Vec::new(),
            attack_on_tile: U16Vec2::ZERO,
        }
    }
    pub fn with_player_base(mut self, player_base: PlayerBase) -> Self {
        self.player_base = Some(player_base);
        self
    }

    pub fn process_effects(&mut self, turn_player: PlayerID) {
        self.effects.retain(|effect| effect.duration() > 0);
        self.effects
            .iter_mut()
            .filter(|effect| effect.get_owner() == turn_player)
            .for_each(|effect| effect.decrease_duration());
    }

    pub fn place(&mut self, card: InPlayID) {
        self.ontile = Some(card);
    }

    pub fn is_occupied(&self) -> bool {
        self.ontile.is_some() || self.player_base.is_some()
    }

    pub fn has_player_base(&self) -> bool {
        self.player_base.is_some()
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
