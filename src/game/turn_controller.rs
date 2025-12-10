use std::sync::Arc;
use tokio::sync::oneshot;

use macroquad::{input::KeyCode, math::I16Vec2};

use crate::{
    engine::{input_handler::InputHandler, renderer::render_config::RenderConfig},
    game::{
        actions::{
            action_context::ActionContext, action_prototype::ActionPrototype,
            targeting::TargetingType,
        },
        card::{card_registry::CardRegistry, Card},
        error::Error,
        game_context::GameContext,
        turn_controller::{
            play_command::{PlayCommand, PlayCommandEffect},
            play_command_builder::PlayCommandBuilder,
        },
    },
};

pub mod play_command;
pub mod play_command_builder;

#[derive(Default)]
pub enum TurnState {
    #[default]
    Idle,
    CardSelected {
        card_index: usize,
    },
    AwaitingInputs {
        targeting_type: TargetingType,
        selected_targets: Vec<I16Vec2>,
        pending_action: ActionPrototype,
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
                Self::AwaitingInputs {
                    targeting_type: l_targeting_type,
                    selected_targets: l_selected_targets,
                    ..
                },
                Self::AwaitingInputs {
                    targeting_type: r_targeting_type,
                    selected_targets: r_selected_targets,
                    ..
                },
            ) => l_targeting_type == r_targeting_type && l_selected_targets == r_selected_targets,
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
    pub action_context: ActionContext,
    sender: Option<oneshot::Sender<Vec<I16Vec2>>>,
}

impl TurnController {
    pub fn new(render_config: Arc<RenderConfig>) -> Self {
        let input_handler = InputHandler::new(render_config);
        Self {
            state: TurnState::Idle,
            input_handler,
            pending_action: None,
            action_context: ActionContext::new(),
            sender: None,
        }
    }

    pub fn update(
        &mut self,
        context: &mut GameContext,
        card_registry: &CardRegistry,
    ) -> Result<Option<PlayCommand>, Error> {
        if self
            .input_handler
            .get_key_press()
            .is_some_and(|key| key == KeyCode::Enter)
        {
            print!("Escape pressed");
            self.pending_action = None;
            self.state = TurnState::EndTurn
        }

        // Handle input based on current state
        match self.state {
            TurnState::Idle => self.handle_idle_state(context),
            TurnState::CardSelected { card_index } => {
                self.handle_card_selected(card_index, context, card_registry)
            }
            TurnState::AwaitingInputs { .. } => self.handle_awaiting_inputs(context),
            TurnState::FigureSelected { position } => {
                self.handle_figure_selected(&position, context, card_registry)
            }
            TurnState::EndTurn => Ok(None),
        }
    }

    fn cancel_input(&mut self) {
        if let TurnState::AwaitingInputs { .. } = &mut self.state {
            self.sender.take(); // Drop the sender, which will cause the receiver to get an error
        }
        self.state = TurnState::Idle;
    }
    pub(crate) fn turn_over(&self) -> bool {
        self.state == TurnState::EndTurn
    }

    pub(crate) fn reset_state(&mut self) {
        self.pending_action = None;
        self.state = TurnState::Idle;
    }

    pub fn request_action_context(&mut self, action: &ActionPrototype) {
        match action.targeting_type() {
            TargetingType::SingleTile
            | TargetingType::Area { .. }
            | TargetingType::Tiles { .. }
            | TargetingType::Line { .. } => {
                self.state = TurnState::AwaitingInputs {
                    targeting_type: action.targeting_type(),
                    selected_targets: Vec::new(),
                    pending_action: action.clone(),
                };
            }
            TargetingType::Caster
            | TargetingType::AreaAroundCaster { .. }
            | TargetingType::AllEnemies
            | TargetingType::None => {}
        }
    }
}

impl TurnController {
    fn handle_awaiting_inputs(
        &mut self,
        context: &GameContext,
    ) -> Result<Option<PlayCommand>, Error> {
        let Some(click_pos) = self.input_handler.get_board_click() else {
            return Ok(None);
        };

        let TurnState::AwaitingInputs {
            targeting_type,
            mut selected_targets,
            pending_action,
        } = std::mem::take(&mut self.state)
        else {
            unreachable!("handle_awaiting_inputs called in wrong state")
        };

        selected_targets.push(click_pos);

        let required_targets = match targeting_type {
            TargetingType::SingleTile | TargetingType::Area { .. } => 1,
            TargetingType::Tiles { amount } => amount.into(),
            TargetingType::Line { .. } => 2,
            _ => todo!(),
        };

        if selected_targets.len() < required_targets {
            self.state = TurnState::AwaitingInputs {
                targeting_type,
                selected_targets,
                pending_action,
            };
            return Ok(None);
        }

        let player = context.turn_player_id();
        let action_context = if selected_targets.len() == 1 {
            ActionContext::new()
                .with_player(player)
                .with_position(selected_targets[0])
        } else {
            ActionContext::new()
                .with_player(player)
                .with_targets(selected_targets)
        };

        let command = PlayCommandBuilder::new()
            .with_effect(PlayCommandEffect::BuildAction {
                action: pending_action,
                action_context,
            })
            .with_owner(player)
            .build();

        println!("Sending Play Command ");
        Ok(Some(command))
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
