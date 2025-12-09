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

pub enum TurnState {
    Idle,
    CardSelected {
        card_index: usize,
    },
    AwaitingInputs {
        targeting_type: TargetingType,
        selected_targets: Vec<I16Vec2>,
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
                },
                Self::AwaitingInputs {
                    targeting_type: r_targeting_type,
                    selected_targets: r_selected_targets,
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

    pub fn update<'a>(
        &'a mut self,
        context: &mut GameContext,
        card_registry: &CardRegistry,
    ) -> Result<Option<PlayCommand<'a>>, Error> {
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
            TurnState::AwaitingInputs { .. } => self.handle_awaiting_inpput(context),
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

    pub async fn request_action_context(
        &mut self,
        action: &ActionPrototype,
    ) -> Result<ActionContext, Error> {
        match action.targeting_type() {
            TargetingType::SingleTile | TargetingType::Area { .. } => {
                let target = self.collect_position(action.targeting_type()).await?;
                Ok(ActionContext::new().with_position(target))
            }
            TargetingType::Tiles { .. } | TargetingType::Line { .. } => {
                let targets = self.collect_targets(action.targeting_type()).await?;
                Ok(ActionContext::new().with_targets(targets))
            }
            TargetingType::Caster
            | TargetingType::AreaAroundCaster { .. }
            | TargetingType::AllEnemies
            | TargetingType::None => Ok(ActionContext::new()),
        }
    }
    // Async method to collect targets from user input
    async fn collect_targets(
        &mut self,
        targeting_type: TargetingType,
    ) -> Result<Vec<I16Vec2>, Error> {
        let (sender, receiver) = oneshot::channel();

        self.state = TurnState::AwaitingInputs {
            targeting_type,
            selected_targets: Vec::new(),
        };

        self.sender = Some(sender);
        // Wait for the input to be collected
        receiver.await.map_err(|_| Error::InputCancelled)
    }

    // Async method to collect a single position
    async fn collect_position(&mut self, targeting_type: TargetingType) -> Result<I16Vec2, Error> {
        let (sender, receiver) = oneshot::channel();

        self.state = TurnState::AwaitingInputs {
            targeting_type,
            selected_targets: Vec::new(),
        };

        self.sender = Some(sender);
        // Wait for position and take the first one
        let positions = receiver.await.map_err(|_| Error::InputCancelled)?;
        positions.into_iter().next().ok_or(Error::NoInputReceived)
    }
}

impl TurnController {
    fn handle_awaiting_inpput<'a>(
        &'a mut self,
        context: &GameContext,
    ) -> Result<Option<PlayCommand<'a>>, Error> {
        let Some(click_pos) = self.input_handler.get_board_click() else {
            return Ok(None);
        };
        let sender = &mut self.sender;
        let TurnState::AwaitingInputs {
            targeting_type,
            selected_targets,
        } = &mut self.state
        else {
            panic!("Selecting called, when state is something else")
        };

        match *targeting_type {
            TargetingType::SingleTile | TargetingType::Area { .. } => {
                if let Some(sender) = sender.take() {
                    let _ = sender.send(vec![click_pos]);
                    self.state = TurnState::Idle;
                }
            }
            TargetingType::Tiles { amount } => {
                selected_targets.push(click_pos);
                if selected_targets.len() == amount.into() {
                    if let Some(sender) = sender.take() {
                        let _ = sender.send(selected_targets.clone());
                        self.state = TurnState::Idle;
                    }
                }
            }
            TargetingType::Line { .. } => {
                selected_targets.push(click_pos);
                if selected_targets.len() == 2 {
                    if let Some(sender) = sender.take() {
                        let _ = sender.send(selected_targets.clone());
                        self.state = TurnState::Idle;
                    }
                }
            }
            _ => {}
        }

        Ok(None)
    }

    fn handle_figure_selected<'a>(
        &'a mut self,
        position: &I16Vec2,
        context: &GameContext,
        card_registry: &CardRegistry,
    ) -> Result<Option<PlayCommand<'a>>, Error> {
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

    fn handle_idle_state<'a>(
        &'a mut self,
        context: &GameContext,
    ) -> Result<Option<PlayCommand<'a>>, Error> {
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

    fn handle_card_selected<'a>(
        &'a mut self,
        card_index: usize,
        context: &GameContext,
        card_registry: &CardRegistry,
    ) -> Result<Option<PlayCommand<'a>>, Error> {
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
