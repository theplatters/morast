use std::sync::Arc;

use macroquad::math::I16Vec2;

use crate::{
    engine::{input_handler::InputHandler, renderer::render_config::RenderConfig},
    game::{
        card::{card_registry::CardRegistry, Card},
        error::Error,
        events::{
            action::Action, action_context::ActionContext, action_effect::TargetingType,
            action_prototype::ActionPrototype, event::Event,
        },
        game_context::GameContext,
        turn_controller::{
            play_command::{PlayCommand, PlayCommandEffect},
            play_command_builder::PlayCommandBuilder,
        },
    },
};

pub mod play_command;
pub mod play_command_builder;

#[derive(Clone)]
pub enum TurnState {
    Idle,
    CardSelected {
        card_index: usize,
    },
    AwaitingTargets {
        targeting_type: TargetingType,
        selected_targets: Vec<I16Vec2>,
        remaining_targets: u8,
    },
    FigureSelected {
        position: I16Vec2,
    },
    EndTurn,
}

impl PartialEq for TurnState {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (
                Self::CardSelected {
                    card_index: l_card_index,
                },
                Self::CardSelected {
                    card_index: r_card_index,
                },
            ) => l_card_index == r_card_index,
            (
                Self::AwaitingTargets {
                    targeting_type: l_targeting_type,
                    selected_targets: l_selected_targets,
                    remaining_targets: l_remaining_targets,
                },
                Self::AwaitingTargets {
                    targeting_type: r_targeting_type,
                    selected_targets: r_selected_targets,
                    remaining_targets: r_remaining_targets,
                },
            ) => {
                l_targeting_type == r_targeting_type
                    && l_selected_targets == r_selected_targets
                    && l_remaining_targets == r_remaining_targets
            }
            (
                Self::FigureSelected {
                    position: l_position,
                },
                Self::FigureSelected {
                    position: r_position,
                },
            ) => l_position == r_position,
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}

pub struct TurnController {
    pub state: TurnState,
    input_handler: InputHandler,
    pending_action: Option<Box<ActionPrototype>>,
    action_context: ActionContext,
}

impl TurnController {
    pub fn new(render_config: Arc<RenderConfig>) -> Self {
        let input_handler = InputHandler::new(render_config);
        Self {
            state: TurnState::Idle,
            input_handler,
            pending_action: None,
            action_context: ActionContext::new(),
        }
    }

    pub fn update(
        &mut self,
        context: &mut GameContext,
        card_registry: &CardRegistry,
    ) -> Result<Option<PlayCommand>, Error> {
        // Handle input based on current state
        match self.state.clone() {
            TurnState::Idle => self.handle_idle_state(context),
            TurnState::CardSelected { card_index } => {
                self.handle_card_selected(card_index, context, card_registry)
            }
            TurnState::AwaitingTargets {
                targeting_type,
                selected_targets,
                remaining_targets,
            } => self.handle_targeting(
                &targeting_type,
                &selected_targets,
                remaining_targets,
                context,
            ),
            TurnState::FigureSelected { position } => {
                self.handle_figure_selected(&position, context, card_registry)
            }
            TurnState::EndTurn => Ok(None),
        }
    }

    pub(crate) fn turn_over(&self) -> bool {
        self.state == TurnState::EndTurn
    }

    pub(crate) fn reset_state(&mut self) {
        self.pending_action = None;
        self.state = TurnState::Idle;
    }

    pub(crate) fn process_event(&mut self, event: Event) -> Result<Option<PlayCommand>, Error> {
        todo!()
    }
}

impl TurnController {
    fn handle_targeting(
        &mut self,
        targeting_type: &TargetingType,
        selected_targets: &[I16Vec2],
        remaining_targets: u8,
        context: &GameContext,
    ) -> Result<Option<PlayCommand>, Error> {
        let Some(next_target) = self.input_handler.get_board_click() else {
            return Ok(None);
        };

        let mut targets = selected_targets.to_owned();
        targets.push(next_target);

        if remaining_targets > 1 {
            self.state = TurnState::AwaitingTargets {
                targeting_type: *targeting_type,
                selected_targets: targets,
                remaining_targets: remaining_targets - 1,
            };
            Ok(None)
        } else {
            let action = self.pending_action.take().expect("Expected pending action");
            let current_player = context.turn_player_id();

            let command = PlayCommandBuilder::new()
                .with_effect(PlayCommandEffect::BuildAction {
                    action,
                    action_context: self.action_context,
                })
                .with_owner(current_player)
                .build();

            Ok(Some(command))
        }
    }

    fn handle_figure_selected(
        &mut self,
        position: &I16Vec2,
        context: &GameContext,
        card_registry: &CardRegistry,
    ) -> Result<Option<PlayCommand>, Error> {
        let Some(next_position) = self.input_handler.get_board_click() else {
            return Ok(None);
        };

        print!(
            "Sending move command from {} to {}",
            position, next_position
        );

        self.state = TurnState::Idle;
        let current_player = context.turn_player_id();

        let command = PlayCommandBuilder::new()
            .with_effect(PlayCommandEffect::MoveCreature {
                from: *position,
                to: next_position,
            })
            .with_owner(current_player)
            .build();

        Ok(Some(command))
    }

    fn handle_idle_state(&mut self, context: &GameContext) -> Result<Option<PlayCommand>, Error> {
        // Check for card selection
        if let Some(card_index) = self.input_handler.get_card_left_click(
            context
                .get_turn_player()
                .ok_or(Error::PlayerNotFound)?
                .hand_size(),
        ) {
            self.state = TurnState::CardSelected { card_index };
            return Ok(None);
        }

        // Check for figure selection
        if let Some(pos) = self.input_handler.get_board_click() {
            if context
                .get_board()
                .get_tile(&pos)
                .ok_or(Error::PlaceError(
                    super::board::place_error::BoardError::TileNotFound,
                ))?
                .is_occupied()
            {
                self.state = TurnState::FigureSelected { position: pos };
            }
        }
        Ok(None)
    }

    fn handle_card_selected(
        &mut self,
        card_index: usize,
        context: &GameContext,
        card_registry: &CardRegistry,
    ) -> Result<Option<PlayCommand>, Error> {
        let Some(pos) = self.input_handler.get_board_click() else {
            return Ok(None);
        };

        self.state = TurnState::Idle;

        let card_id = context
            .get_turn_player()
            .ok_or(Error::PlayerNotFound)?
            .get_card_in_hand(card_index)
            .ok_or(Error::InvalidHandPosition(card_index))?;
        let card = card_registry.get(&card_id).ok_or(Error::CardNotFound)?;
        let current_player = context.turn_player_id();

        let effect = match card {
            Card::Creature(_) => PlayCommandEffect::PlaceCreature {
                card_index,
                position: pos,
            },
            Card::Spell(_) => PlayCommandEffect::CastSpell { card_index },
            Card::Trap(_) => PlayCommandEffect::PlaceTrap {
                card_index,
                position: pos,
            },
        };

        let command = PlayCommandBuilder::new()
            .with_effect(effect)
            .with_owner(current_player)
            .build();

        Ok(Some(command))
    }
}
