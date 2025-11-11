use std::sync::Arc;

use macroquad::{input::KeyCode, math::I16Vec2};

use crate::{
    engine::{input_handler::InputHandler, renderer::render_config::RenderConfig},
    game::{
        board::place_error::BoardError, card::card_registry::CardRegistry, error::Error,
        events::event_scheduler::GameScheduler, game_context::GameContext,
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

impl TurnController {
    pub fn new(render_config: Arc<RenderConfig>) -> Self {
        let input_handler = InputHandler::new(render_config);
        Self {
            step: TurnStep::Blank,
            input_handler,
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
        if let Some(pos) = self.input_handler.get_card_click(hand_size) {
            self.step = TurnStep::Cardchoosen(pos);
        };

        if let Some(pos) = self.input_handler.get_board_click() {
            let tile = context
                .get_board()
                .get_tile(&pos)
                .ok_or(Error::PlaceError(BoardError::TileNotFound))?;
            match self.step {
                TurnStep::Cardchoosen(card_pos) => {
                    match context.place_card_from_hand(
                        context.turn_player_id(),
                        card_pos,
                        pos,
                        card_registry,
                        scheduler,
                    ) {
                        Err(Error::InsufficientGold) | Err(Error::InvalidMove) => {
                            self.step = TurnStep::Blank;
                        }

                        Ok(_) => {
                            self.step = TurnStep::Blank;
                        }

                        Err(err) => {
                            return Err(err);
                        }
                    };
                    self.step = TurnStep::Blank;
                }

                TurnStep::Blank if tile.is_occupied() => {
                    self.step = TurnStep::Figurechosen(pos);
                }
                TurnStep::Figurechosen(card_on_board) => {
                    match context.move_card(card_on_board, pos, card_registry) {
                        Err(Error::InsufficientGold) | Err(Error::InvalidMove) | Ok(_) => {
                            self.step = TurnStep::Blank;
                        }
                        Err(err) => return Err(err),
                    };
                }
                _ => {}
            }
        };

        Ok(self.step)
    }

    pub(crate) fn turn_over(&self) -> bool {
        self.step == TurnStep::EndTurn
    }

    pub(crate) fn reset_state(&mut self) {
        self.step = TurnStep::Blank;
    }
}
