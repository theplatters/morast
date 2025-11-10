use std::sync::Arc;

use macroquad::{
    math::{I16Vec2, Vec2},
    shapes::draw_rectangle_lines,
    text::draw_text,
};

mod card_render;
pub mod render_config;

use crate::{
    engine::{
        asset_loader::AssetLoader,
        renderer::{card_render::CardRenderer, render_config::RenderConfig},
    },
    game::{
        board::Board,
        card::{card_registry::CardRegistry, Card},
        error::Error,
        game_context::GameContext,
    },
};

pub struct Renderer {
    cards_to_draw: Vec<CardRenderer>,
    render_config: Arc<RenderConfig>,
}

impl<'a> Renderer {
    pub fn new(render_config: Arc<RenderConfig>) -> Self {
        Self {
            cards_to_draw: Vec::new(),
            render_config,
        }
    }
    pub fn update_cards(&mut self, game_cards: &[&Card], assets: &AssetLoader) {
        self.cards_to_draw.clear();

        for (i, card) in game_cards.iter().enumerate() {
            let pos_x =
                i as f32 * (self.render_config.card_width + self.render_config.card_padding);
            let card = CardRenderer::new(
                Vec2::new(pos_x, self.render_config.hand_y),
                card.cost,
                card.attack_strength,
                card.defense,
                card.name.clone(),
                self.render_config.clone(),
            );

            self.cards_to_draw.push(card)
        }
    }

    pub fn draw_board(&self, board: &Board, _asset_loader: &AssetLoader) {
        let tile_size: f32 = self.render_config.tile_size;

        for x in 0i16..board.width() {
            for y in 0i16..board.height() {
                let pos = I16Vec2::new(x, y);
                let tile = board.get_tile(&pos).unwrap();

                // Determine tile color
                let color = if tile.has_effects() {
                    macroquad::color::GREEN
                } else {
                    macroquad::color::WHITE
                };

                // Calculate screen position
                let screen_x = x as f32 * tile_size;
                let screen_y = y as f32 * tile_size;

                // Draw tile background
                macroquad::shapes::draw_rectangle(screen_x, screen_y, tile_size, tile_size, color);
                draw_rectangle_lines(
                    screen_x,
                    screen_y,
                    tile_size,
                    tile_size,
                    1.0,
                    macroquad::color::BLACK,
                );

                // Draw attack values
                let attack_x = tile.attack_on_tile.x;
                let attack_y = tile.attack_on_tile.y;
                let attack_text = format!("{:}, {:}", attack_x, attack_y);
                if attack_x != 0 || attack_y != 0 {
                    draw_text(
                        &attack_text,
                        screen_x + 8.0,
                        screen_y + 20.0,
                        14.0,
                        macroquad::color::BLACK,
                    );
                }

                // Draw X if occupied
                if tile.ontile.is_some() {
                    let thickness = 2.0;
                    let padding = 4.0;

                    // Draw two crossing lines for X
                    macroquad::shapes::draw_line(
                        screen_x + padding,
                        screen_y + padding,
                        screen_x + tile_size - padding,
                        screen_y + tile_size - padding,
                        thickness,
                        macroquad::color::BLACK,
                    );
                    macroquad::shapes::draw_line(
                        screen_x + padding,
                        screen_y + tile_size - padding,
                        screen_x + tile_size - padding,
                        screen_y + padding,
                        thickness,
                        macroquad::color::BLACK,
                    );
                }
            }
        }
    }

    fn draw_hand(&self) {
        for card in self.cards_to_draw.iter() {
            card.draw_card();
        }
    }

    pub(crate) fn render(
        &mut self,
        context: &GameContext,
        asset_loader: &'a AssetLoader,
        card_registry: &CardRegistry,
    ) -> Result<(), Error> {
        let player = context.get_turn_player().ok_or(Error::PlayerNotFound)?;
        let hand = player.get_hand();
        let cards: Vec<_> = hand
            .iter()
            .filter_map(|card| card_registry.get(card))
            .collect();

        self.update_cards(cards.as_slice(), asset_loader);
        self.draw_board(context.get_board(), asset_loader);
        self.draw_hand();
        Ok(())
    }
}
