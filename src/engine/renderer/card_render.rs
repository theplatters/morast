use std::sync::Arc;

use macroquad::{
    color::*,
    math::Vec2,
    shapes::{draw_rectangle, draw_rectangle_lines},
    text::{draw_text, measure_text},
};

use crate::engine::renderer::render_config::RenderConfig;

pub struct CardRenderer {
    position: Vec2,
    cost: u16,
    attack: u16,
    defense: u16,
    name: String,
    render_config: Arc<RenderConfig>,
}

impl CardRenderer {
    pub fn new(
        position: Vec2,
        cost: u16,
        attack: u16,
        defense: u16,
        name: String,
        render_config: Arc<RenderConfig>,
    ) -> Self {
        Self {
            position,
            cost,
            attack,
            defense,
            name,
            render_config,
        }
    }

    pub fn draw_card(&self) {
        // Card background
        draw_rectangle(
            self.position.x,
            self.position.y,
            self.render_config.card_width,
            self.render_config.card_height,
            RED,
        );

        // Card border
        draw_rectangle_lines(
            self.position.x,
            self.position.y,
            self.render_config.card_width,
            self.render_config.card_height,
            2.0,
            BLACK,
        );

        // Name at the top center
        let name_x = self.position.x + self.render_config.card_width
            - measure_text(&self.name, None, 24, 1.0).width * 2.0;
        draw_text(&self.name, name_x, self.position.y + 30.0, 24.0, WHITE);

        // Cost in top-left
        draw_text(
            &format!("Cost: {}", self.cost),
            self.position.x + 10.0,
            self.position.y + 60.0,
            20.0,
            YELLOW,
        );

        // Attack in bottom-left
        draw_text(
            &format!("Atk: {}", self.attack),
            self.position.x + 10.0,
            self.position.y + self.render_config.card_height - 20.0,
            20.0,
            GREEN,
        );

        // Defense in bottom-right
        let defense_text = format!("Def: {}", self.defense);
        let def_x = self.position.x + self.render_config.card_width
            - measure_text(&defense_text, None, 20, 1.0).width
            - 10.0;
        draw_text(
            &defense_text,
            def_x,
            self.position.y + self.render_config.card_height - 20.0,
            20.0,
            BLUE,
        );
    }
}
