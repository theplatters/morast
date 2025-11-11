use std::sync::Arc;

use macroquad::{math::I16Vec2, shapes::draw_rectangle_lines, text::draw_text};

use crate::{
    engine::{asset_loader::AssetLoader, renderer::render_config::RenderConfig},
    game::{
        board::{self, place_error::BoardError, Board},
        card::card_registry::CardRegistry,
        error::Error,
        turn_controller::TurnStep,
    },
};

pub struct BoardRenderer {
    render_config: Arc<RenderConfig>,
}
impl BoardRenderer {
    pub(crate) fn new(render_config: Arc<RenderConfig>) -> Self {
        Self { render_config }
    }

    pub fn draw_highlights(&self, tiles: &[I16Vec2]) {
        let tile_size = self.render_config.tile_size;

        for pos in tiles {
            let screen_x = pos.x as f32 * tile_size;
            let screen_y = pos.y as f32 * tile_size;

            // Draw a translucent yellow overlay
            macroquad::shapes::draw_rectangle(
                screen_x,
                screen_y,
                tile_size,
                tile_size,
                macroquad::color::Color::new(1.0, 1.0, 0.0, 0.5), // RGBA = semi-transparent yellow
            );
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

    pub(crate) fn draw_available_place_positions(
        &self,
        context: &crate::game::game_context::GameContext,
    ) {
        let turn_player = context.turn_player_id();
        let tiles: Vec<_> = context
            .get_board()
            .tile_pos_iter()
            .filter(|tile| context.is_on_player_side(*tile, turn_player))
            .collect();
        self.draw_highlights(&tiles);
    }
}
