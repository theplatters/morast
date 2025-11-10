use std::sync::Arc;

use macroquad::{
    input::*,
    math::{I16Vec2, IVec2},
};

use crate::engine::renderer::render_config::RenderConfig;

#[derive(Debug)]
pub struct InputHandler {
    render_config: Arc<RenderConfig>,
}

impl InputHandler {
    pub fn new(render_config: Arc<RenderConfig>) -> Self {
        InputHandler { render_config }
    }
    /// Checks if a card was clicked in the player's hand.
    /// Returns `Some(index)` for the clicked card, or `None` if none clicked.
    pub fn get_card_click(&self, hand_size: usize) -> Option<usize> {
        if !is_mouse_button_pressed(MouseButton::Left) {
            return None;
        }
        let (mouse_x, mouse_y) = mouse_position();

        let card_width = self.render_config.card_width;
        let card_height = self.render_config.card_height;
        let card_y = self.render_config.hand_y;

        for pos in 0..hand_size {
            let screen_x = (card_width + self.render_config.card_padding) * pos as f32;

            let clicked = mouse_x >= screen_x
                && mouse_x <= screen_x + card_width
                && mouse_y >= card_y
                && mouse_y <= card_y + card_height;

            if clicked {
                return Some(pos);
            }
        }

        None
    }

    pub(crate) fn get_board_click(&self) -> Option<I16Vec2> {
        if !is_mouse_button_pressed(MouseButton::Left) {
            return None;
        }

        let (mouse_x, mouse_y) = mouse_position();

        let board_width = self.render_config.board_width;
        let board_height = self.render_config.board_height;

        if mouse_x > board_width || mouse_y > board_height {
            return None;
        }

        let tile_size = self.render_config.tile_size;
        Some(I16Vec2::new(
            (mouse_x / tile_size).floor() as i16,
            (mouse_y / tile_size).floor() as i16,
        ))
    }

    pub(crate) fn get_key_press(&self) -> Option<KeyCode> {
        if is_key_released(KeyCode::Enter) {
            return Some(KeyCode::Enter);
        }

        None
    }
}
