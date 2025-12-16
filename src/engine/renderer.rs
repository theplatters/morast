use std::sync::Arc;

use bevy::{
    app::{Plugin, Startup},
    asset::AssetServer,
    camera::Camera2d,
    ecs::{
        resource::Resource,
        schedule::graph::Direction,
        system::{Commands, Res},
    },
    sprite::Sprite,
    transform::components::Transform,
};

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
        card::{card_registry::CardRegistry, Card, CardBehavior},
        error::GameError,
        turn_controller::TurnState,
    },
};

#[derive(Resource)]
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
        turn_step: &TurnState,
        assets: &AssetLoader,
    ) {
        self.cards_to_draw.clear();

        for (i, card) in game_cards.iter().enumerate() {
            let pos_x =
                i as f32 * (self.render_config.card_width + self.render_config.card_padding);
            let highlighted = match turn_step {
                TurnState::CardSelected { card_index } => *card_index == i,
                _ => false,
            };
            let mut card_builder = CardRenderer::builder()
                .cost(card.cost())
                .position(Vec2::new(pos_x, self.render_config.hand_y))
                .name(card.name())
                .description(card.description())
                .highlighted(highlighted)
                .render_config(self.render_config.clone());

            card_builder = match card {
                Card::Creature(c) => card_builder.creature(c.attack_strength, c.defense),
                Card::Spell(c) => card_builder.spell(),
                Card::Trap(c) => card_builder.trap(),
            };
            self.cards_to_draw
                .push(card_builder.build().expect("Could not build card"))
        }
    }

    pub fn draw_turn_state(
        &self,
        turn_step: &TurnState,
        context: &GameContext,
        card_registy: &CardRegistry,
    ) -> Result<(), GameError> {
        match turn_step {
            TurnState::FigureSelected { position } => {
                let board = context.get_board();
                let card_id = board.get_card_on_tile(position)?;
                let movement_pattern = &card_registy
                    .get_creature(&card_id.card_id)
                    .ok_or(GameError::CardNotFound)?
                    .movement;

                let highlights: Vec<I16Vec2> = movement_pattern
                    .iter()
                    .map(|tile| *tile + *position)
                    .collect();
                if context
                    .get_board()
                    .can_card_move(context.turn_player_id(), position)
                {
                    self.board_renderer.draw_highlights(&highlights);
                }
            }
            TurnState::CardSelected { card_index: _ } => {
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
        turn_step: &TurnState,
        asset_loader: &AssetLoader,
        card_registry: &CardRegistry,
    ) -> Result<(), GameError> {
        let player = context.get_turn_player().ok_or(GameError::PlayerNotFound)?;
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

pub struct RendererPlugin;

impl Plugin for RendererPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.init_resource::<RenderConfig>()
            .add_systems(Startup, setup_renderer);
    }
}

pub fn setup_renderer(mut commands: Commands) {
    commands.spawn(Camera2d);
}

pub fn draw_board() {}

pub fn draw_card(cards: Query<&) {}
