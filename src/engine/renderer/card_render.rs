use std::sync::Arc;

use macroquad::{
    color::*,
    math::Vec2,
    shapes::{draw_rectangle, draw_rectangle_lines},
    text::{draw_multiline_text, draw_text, measure_text},
};

use crate::engine::renderer::render_config::RenderConfig;

pub struct CardRenderer {
    position: Vec2,
    cost: u16,
    attack: u16,
    defense: u16,
    name: String,
    description: String,
    highlighted: bool,
    render_config: Arc<RenderConfig>,
}

impl CardRenderer {
    pub fn new(
        position: Vec2,
        cost: u16,
        attack: u16,
        defense: u16,
        name: String,
        description: String,
        highlighted: bool,
        render_config: Arc<RenderConfig>,
    ) -> Self {
        Self {
            position,
            cost,
            highlighted,
            attack,
            defense,
            description,
            name,
            render_config,
        }
    }
    fn wrap_text_to_width(&self, text: &str, font_size: f32, max_width: f32) -> String {
        let char_width = font_size * 0.6; // Approximate character width
        let chars_per_line = (max_width / char_width) as usize;

        if chars_per_line == 0 {
            return text.to_string();
        }

        text.split_whitespace()
            .fold((String::new(), 0), |(mut result, mut line_len), word| {
                if line_len + word.len() + 1 > chars_per_line && line_len > 0 {
                    result.push('\n');
                    result.push_str(word);
                    (result, word.len())
                } else {
                    if line_len > 0 {
                        result.push(' ');
                        line_len += 1;
                    }
                    result.push_str(word);
                    (result, line_len + word.len())
                }
            })
            .0
    }

    pub fn draw_card(&self) {
        let offset = if self.highlighted {
            self.render_config.select_offset
        } else {
            Vec2::ZERO
        };
        // Card background
        //
        let position = self.position + offset;
        draw_rectangle(
            position.x,
            position.y,
            self.render_config.card_width,
            self.render_config.card_height,
            RED,
        );

        // Card border
        draw_rectangle_lines(
            position.x,
            position.y,
            self.render_config.card_width,
            self.render_config.card_height,
            2.0,
            BLACK,
        );

        // Name at the top center
        let name_x = position.x + self.render_config.card_width
            - measure_text(&self.name, None, 24, 1.0).width * 2.0;
        draw_text(&self.name, name_x, position.y + 30.0, 24.0, WHITE);

        // Cost in top-left
        draw_text(
            &format!("Cost: {}", self.cost),
            position.x + 10.0,
            position.y + 60.0,
            20.0,
            YELLOW,
        );

        // Attack in bottom-left
        draw_text(
            &format!("Atk: {}", self.attack),
            position.x + 10.0,
            position.y + self.render_config.card_height - 20.0,
            20.0,
            GREEN,
        );

        // Defense in bottom-right
        let defense_text = format!("Def: {}", self.defense);
        let def_x = position.x + self.render_config.card_width
            - measure_text(&defense_text, None, 20, 1.0).width
            - 10.0;
        draw_text(
            &defense_text,
            def_x,
            position.y + self.render_config.card_height - 20.0,
            20.0,
            BLUE,
        );

        let description_x = position.x + 20.0;
        let description_y = position.y + self.render_config.card_height / 2.0;

        // Usage:
        let wrapped_description = self.wrap_text_to_width(
            &self.description,
            20.0,
            self.render_config.card_width - 20.0, // Leave some padding
        );

        draw_multiline_text(
            &wrapped_description,
            description_x,
            description_y,
            20.0,
            Some(1.0),
            YELLOW,
        );
    }
}
