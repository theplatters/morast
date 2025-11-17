use std::sync::Arc;

use macroquad::{input::KeyCode, math::I16Vec2};

use crate::{
    engine::{input_handler::InputHandler, renderer::render_config::RenderConfig},
    game::{
        board::{place_error::BoardError, tile::Tile},
        card::card_registry::CardRegistry,
        error::Error,
        events::event_scheduler::GameScheduler,
        game_context::GameContext,
    },
};

#[derive(Clone, Copy)]
pub enum TurnStep {
    Blank,
    Cardchoosen(usize),
    Figurechosen(I16Vec2),
    EndTurn,
}

impl PartialEq for TurnStep {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Cardchoosen(l0), Self::Cardchoosen(r0)) => l0 == r0,
            (Self::Figurechosen(l0), Self::Figurechosen(r0)) => l0 == r0,
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}
pub struct TurnController {
    step: TurnStep,
    input_handler: InputHandler,
}

//helper methods for updating board
impl TurnController {
    fn handle_card_placement(
        &self,
        context: &mut GameContext,
        card_pos: usize,
        target_pos: I16Vec2,
        card_registry: &CardRegistry,
        scheduler: &mut GameScheduler,
    ) -> Result<TurnStep, Error> {
        match context.play_card_from_hand(
            context.turn_player_id(),
            card_pos,
            target_pos,
            card_registry,
            scheduler,
        ) {
            Ok(_) => Ok(TurnStep::Blank),
            Err(Error::InsufficientGold | Error::InvalidMove) => Ok(TurnStep::Blank),
            Err(err) => Err(err),
        }
    }

    fn handle_blank_state(&self, tile: &Tile, pos: I16Vec2) -> Result<TurnStep, Error> {
        if tile.is_occupied() {
            Ok(TurnStep::Figurechosen(pos))
        } else {
            Ok(TurnStep::Blank)
        }
    }

    fn handle_figure_movement(
        &self,
        context: &mut GameContext,
        card_on_board: I16Vec2,
        target_pos: I16Vec2,
        card_registry: &CardRegistry,
    ) -> Result<TurnStep, Error> {
        match context.move_card(card_on_board, target_pos, card_registry) {
            Ok(_) => Ok(TurnStep::Blank),
            Err(Error::InsufficientGold)
            | Err(Error::InvalidMove)
            | Err(Error::PlaceError(BoardError::NoMovementPoints)) => Ok(TurnStep::Blank),
            Err(err) => Err(err),
        }
    }
}

impl TurnController {
    pub fn new(render_config: Arc<RenderConfig>) -> Self {
        let input_handler = InputHandler::new(render_config);
        Self {
            step: TurnStep::Blank,
            input_handler,
        }
    }

    fn update_board(
        &self,
        context: &mut GameContext,
        card_registry: &CardRegistry,
        scheduler: &mut GameScheduler,
    ) -> Result<TurnStep, Error> {
        let pos = match self.input_handler.get_board_click() {
            Some(pos) => pos,
            None => return Ok(self.step), // No click, return current step
        };

        // Validate the clicked position exists
        let tile = context
            .get_board()
            .get_tile(&pos)
            .ok_or(Error::PlaceError(BoardError::TileNotFound))?;

        match self.step {
            TurnStep::Cardchoosen(card_pos) => {
                self.handle_card_placement(context, card_pos, pos, card_registry, scheduler)
            }
            TurnStep::Blank => self.handle_blank_state(tile, pos),
            TurnStep::Figurechosen(card_on_board) => {
                self.handle_figure_movement(context, card_on_board, pos, card_registry)
            }
            TurnStep::EndTurn => Ok(TurnStep::EndTurn),
        }
    }

    pub fn update(
        &mut self,
        context: &mut GameContext,
        card_registry: &CardRegistry,
        scheduler: &mut GameScheduler,
    ) -> Result<TurnStep, Error> {
        if let Some(key) = self.input_handler.get_key_press() {
            if key == KeyCode::Enter {
                self.step = TurnStep::EndTurn;
            }
        }
        let turn_player = context.get_turn_player().ok_or(Error::PlayerNotFound)?;

        let hand_size = turn_player.hand_size();
        if let Some(pos) = self.input_handler.get_card_left_click(hand_size) {
            self.step = TurnStep::Cardchoosen(pos);
        };

        if let Some(pos) = self.input_handler.get_card_right_click(hand_size) {
            context
                .get_turn_player_mut()
                .ok_or(Error::PlayerNotFound)?
                .sell_card(pos, card_registry)?;
        };

        self.step = self.update_board(context, card_registry, scheduler)?;

        Ok(self.step)
    }

    pub(crate) fn turn_over(&self) -> bool {
        self.step == TurnStep::EndTurn
    }

    pub(crate) fn reset_state(&mut self) {
        self.step = TurnStep::Blank;
    }
}
