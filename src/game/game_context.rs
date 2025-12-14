use log::debug;
use macroquad::math::I16Vec2;

use crate::game::{
    board::card_on_board::CreatureOnBoard,
    card::{creature::Creature, deck_builder::DeckBuilder, trap_card::Trap, Card, CardBehavior},
    phases::Phase,
};

use super::{
    actions::action_manager::ActionManager,
    board::{effect::Effect, place_error::BoardError, Board},
    card::{card_id::CardID, card_registry::CardRegistry, in_play_id::InPlayID},
    error::Error,
    player::{Player, PlayerID},
};

const NUM_CARDS_AT_START: u16 = 2;

pub struct GameContext {}

impl GameContext {
    pub fn place_creature(
        &mut self,
        card_id: CardID,
        creature: &Creature,
        index: I16Vec2,
        card_registry: &CardRegistry,
    ) -> Result<InPlayID, Error> {
        todo!()
    }

    pub fn place_trap() -> Result<(), Error> {
        todo!()
    }

    pub(crate) fn execute_creature_placement() -> Result<CardID, Error> {
        todo!()
    }

    pub(crate) fn cast_spell_from_hand() -> Result<CardID, Error> {
        todo!()
    }

    pub(crate) fn execute_trap_placement() -> Result<CardID, Error> {
        todo!()
    }
}

impl GameContext {
    pub fn new() -> Self {
        Self {}
    }
}
