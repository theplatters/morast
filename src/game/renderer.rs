use macroquad::{math::I16Vec2, text::draw_text};

use crate::{
    engine::asset_loader::AssetLoader,
    game::{board::Board, player::Player},
};

pub struct Renderer {}

impl Renderer {
    pub fn draw_board(&self, board: Board) {
        const TILE_SIZE: f32 = 64.0;

        for x in 0i16..=board.width() {
            for y in 0i16..=board.height() {
                let pos = I16Vec2::new(x, y);
                let tile = board.get_tile(&pos).unwrap();

                // Determine tile color
                let color = if tile.has_effects() {
                    macroquad::color::GREEN
                } else {
                    macroquad::color::WHITE
                };

                // Calculate screen position
                let screen_x = x as f32 * TILE_SIZE;
                let screen_y = y as f32 * TILE_SIZE + 200.0;

                // Draw tile background
                macroquad::shapes::draw_rectangle(screen_x, screen_y, TILE_SIZE, TILE_SIZE, color);

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
                        screen_x + TILE_SIZE - padding,
                        screen_y + TILE_SIZE - padding,
                        thickness,
                        macroquad::color::BLACK,
                    );
                    macroquad::shapes::draw_line(
                        screen_x + padding,
                        screen_y + TILE_SIZE - padding,
                        screen_x + TILE_SIZE - padding,
                        screen_y + padding,
                        thickness,
                        macroquad::color::BLACK,
                    );
                }
            }
        }
    }

    fn draw_hand(self, player: &Player) {}
}

