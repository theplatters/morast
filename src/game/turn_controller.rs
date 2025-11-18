use std::sync::Arc;

use macroquad::math::I16Vec2;

use crate::{
    engine::{input_handler::InputHandler, renderer::render_config::RenderConfig},
    game::{
        card::{card_registry::CardRegistry, Card},
        error::Error,
        events::event_scheduler::GameScheduler,
        game_action::TargetingType,
        game_context::GameContext,
        turn_controller::play_command::PlayCommand,
    },
};

pub mod play_command;

#[derive(Clone)]
pub enum TurnState {
    Idle,
    CardSelected {
        card_index: usize,
        targeting_type: TargetingType,
    },
    AwaitingTargets {
        card_index: usize,
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
                    targeting_type: l_targeting_type,
                },
                Self::CardSelected {
                    card_index: r_card_index,
                    targeting_type: r_targeting_type,
                },
            ) => l_card_index == r_card_index && l_targeting_type == r_targeting_type,
            (
                Self::AwaitingTargets {
                    card_index: l_card_index,
                    targeting_type: l_targeting_type,
                    selected_targets: l_selected_targets,
                    remaining_targets: l_remaining_targets,
                },
                Self::AwaitingTargets {
                    card_index: r_card_index,
                    targeting_type: r_targeting_type,
                    selected_targets: r_selected_targets,
                    remaining_targets: r_remaining_targets,
                },
            ) => {
                l_card_index == r_card_index
                    && l_targeting_type == r_targeting_type
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
}

impl TurnController {
    pub fn new(render_config: Arc<RenderConfig>) -> Self {
        let input_handler = InputHandler::new(render_config);
        Self {
            state: TurnState::Idle,
            input_handler,
        }
    }

    pub fn update(
        &mut self,
        context: &mut GameContext,
        card_registry: &CardRegistry,
        scheduler: &mut GameScheduler,
    ) -> Result<Option<PlayCommand>, Error> {
        // Handle input based on current state
        match self.state.clone() {
            TurnState::Idle => self.handle_idle_state(context, card_registry),
            TurnState::CardSelected {
                card_index,
                targeting_type,
            } => self.handle_card_selected(
                card_index,
                targeting_type.clone(),
                context,
                card_registry,
            ),
            TurnState::AwaitingTargets {
                card_index,
                targeting_type,
                selected_targets,
                remaining_targets,
            } => self.handle_targeting(
                card_index,
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
        self.state = TurnState::Idle;
    }
}

impl TurnController {
    fn handle_targeting(
        &mut self,
        card_index: usize,
        targeting_type: &TargetingType,
        selected_targets: &Vec<I16Vec2>,
        remaining_targets: u8,
        context: &GameContext,
    ) -> Result<Option<PlayCommand>, Error> {
        todo!()
    }

    fn handle_figure_selected(
        &mut self,
        position: &I16Vec2,
        context: &GameContext,
        card_registry: &CardRegistry,
    ) -> Result<Option<PlayCommand>, Error> {
        todo!()
    }

    fn handle_idle_state(
        &mut self,
        context: &GameContext,
        card_registry: &CardRegistry,
    ) -> Result<Option<PlayCommand>, Error> {
        // Check for card selection
        if let Some(card_index) = self.input_handler.get_card_left_click(
            context
                .get_turn_player()
                .ok_or(Error::PlayerNotFound)?
                .hand_size(),
        ) {
            let targeting_type =
                self.get_card_targeting_type(context, card_index, card_registry)?;

            self.state = TurnState::CardSelected {
                card_index,
                targeting_type,
            };
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
        targeting_type: TargetingType,
        context: &GameContext,
        card_registry: &CardRegistry,
    ) -> Result<Option<PlayCommand>, Error> {
        match targeting_type {
            TargetingType::None => {
                // Execute immediately (like instant spells)
                self.state = TurnState::Idle;
                Ok(Some(PlayCommand::CastSpell {
                    card_index,
                    targets: vec![],
                }))
            }
            TargetingType::SingleTile => {
                if let Some(pos) = self.input_handler.get_board_click() {
                    self.state = TurnState::Idle;

                    let card_id = context
                        .get_turn_player()
                        .ok_or(Error::PlayerNotFound)?
                        .get_card_in_hand(card_index)
                        .ok_or(Error::InvalidHandPosition(card_index))?;
                    let card = card_registry.get(&card_id).ok_or(Error::CardNotFound)?;
                    match card {
                        Card::Creature(_) => Ok(Some(PlayCommand::PlaceCreature {
                            card_index,
                            position: pos,
                        })),
                        Card::Spell(_) => Ok(Some(PlayCommand::CastSpell {
                            card_index,
                            targets: vec![pos],
                        })),
                        Card::Trap(_) => Ok(Some(PlayCommand::PlaceTrap {
                            card_index,
                            position: pos,
                        })),
                    }
                } else {
                    Ok(None)
                }
            }
            TargetingType::Area { radius } => {
                self.state = TurnState::AwaitingTargets {
                    card_index,
                    targeting_type,
                    selected_targets: vec![],
                    remaining_targets: 1, // Area spells typically need 1 center point
                };
                Ok(None)
            }
            // Handle other targeting types...
            _ => todo!("Implement other targeting types"),
        }
    }

    fn get_card_targeting_type(
        &self,
        context: &GameContext,
        card_index: usize,
        card_registry: &CardRegistry,
    ) -> Result<TargetingType, Error> {
        let player = context.get_turn_player().ok_or(Error::PlayerNotFound)?;
        let card_id = player
            .get_card_in_hand(card_index)
            .ok_or(Error::CardNotFound)?;
        let card = card_registry.get(&card_id).ok_or(Error::CardNotFound)?;

        match card {
            Card::Creature(_) | Card::Trap(_) => Ok(TargetingType::SingleTile),
            Card::Spell(spell) => {
                // Get targeting from spell's first action or card definition
                Ok(spell.get_targeting_type())
            }
        }
    }
}
