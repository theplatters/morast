use std::sync::Arc;

use macroquad::math::{I16Vec2, Vec2};

mod board_renderer;
mod card_render;
pub mod render_config;

use crate::{
    engine::{
        asset_loader::AssetLoader,
        renderer::{
            board_renderer::BoardRenderer, card_render::CardRenderer, render_config::RenderConfig,
        },
    },
    game::{
        board::{place_error::BoardError, Board},
        card::{card_registry::CardRegistry, Card},
        error::Error,
        game_context::GameContext,
        turn_controller::TurnStep,
    },
};

pub struct Renderer {
    cards_to_draw: Vec<CardRenderer>,
    board_renderer: BoardRenderer,
    render_config: Arc<RenderConfig>,
}

impl Renderer {
    pub fn new(render_config: Arc<RenderConfig>) -> Self {
        Self {
            cards_to_draw: Vec::new(),
            board_renderer: BoardRenderer::new(render_config.clone()),
            render_config,
        }
    }
    pub fn update_cards(
        &mut self,
        game_cards: &[&Card],
        turn_step: &TurnStep,
        assets: &AssetLoader,
    ) {
        self.cards_to_draw.clear();

        for (i, card) in game_cards.iter().enumerate() {
            let pos_x =
                i as f32 * (self.render_config.card_width + self.render_config.card_padding);
            let highlighted = match turn_step {
                TurnStep::Cardchoosen(card_chosen_index) => *card_chosen_index == i,
                _ => false,
            };
            let card = CardRenderer::new(
                Vec2::new(pos_x, self.render_config.hand_y),
                card.cost,
                card.attack_strength,
                card.defense,
                card.name.clone(),
                highlighted,
                self.render_config.clone(),
            );

            self.cards_to_draw.push(card)
        }
    }

    pub fn draw_turn_state(
        &self,
        turn_step: &TurnStep,
        context: &GameContext,
        card_registy: &CardRegistry,
    ) -> Result<(), Error> {
        match turn_step {
            TurnStep::Figurechosen(pos) => {
                let board = context.get_board();
                let card_id = board
                    .get_tile(pos)
                    .ok_or(Error::PlaceError(BoardError::TileNotFound))?
                    .ontile
                    .ok_or(Error::TileEmpty)?
                    .card_id;
                let movement_pattern = card_registy
                    .get(&card_id)
                    .ok_or(Error::CardNotFound)?
                    .get_movement_pattern();

                let highlights: Vec<I16Vec2> =
                    movement_pattern.iter().map(|tile| *tile + *pos).collect();

                self.board_renderer.draw_highlights(&highlights);
            }
            TurnStep::Cardchoosen(_) => {
                self.board_renderer.draw_available_place_positions(context);
            }
            _ => {}
        };
        Ok(())
    }

    fn draw_hand(&self) {
        for card in self.cards_to_draw.iter() {
            card.draw_card();
        }
    }

    pub(crate) fn render(
        &mut self,
        context: &GameContext,
        turn_step: &TurnStep,
        asset_loader: &AssetLoader,
        card_registry: &CardRegistry,
    ) -> Result<(), Error> {
        let player = context.get_turn_player().ok_or(Error::PlayerNotFound)?;
        let hand = player.get_hand();
        let cards: Vec<_> = hand
            .iter()
            .filter_map(|card| card_registry.get(card))
            .collect();

        self.update_cards(cards.as_slice(), turn_step, asset_loader);
        self.board_renderer
            .draw_board(context.get_board(), asset_loader);
        self.draw_turn_state(turn_step, context, card_registry)?;
        self.draw_hand();
        Ok(())
    }
}
