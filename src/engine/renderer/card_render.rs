use std::sync::Arc;

use macroquad::{
    color::*,
    math::Vec2,
    shapes::{draw_rectangle, draw_rectangle_lines},
    text::{draw_multiline_text, draw_text, measure_text},
};

use crate::{
    engine::renderer::{self, render_config::RenderConfig},
    game::card::Card,
};

pub struct CreatureCardRenderer {
    attack: u16,
    defense: u16,
}

impl CreatureCardRenderer {
    fn draw(&self, position: Vec2, render_config: &Arc<RenderConfig>) {
        // Attack in bottom-left
        draw_text(
            &format!("Atk: {}", self.attack),
            position.x + 10.0,
            position.y + render_config.card_height - 20.0,
            20.0,
            GREEN,
        );

        // Defense in bottom-right
        let defense_text = format!("Def: {}", self.defense);
        let def_x = position.x + render_config.card_width
            - measure_text(&defense_text, None, 20, 1.0).width
            - 10.0;
        draw_text(
            &defense_text,
            def_x,
            position.y + render_config.card_height - 20.0,
            20.0,
            BLUE,
        );
    }
}

struct SpellCardRenderer {}

struct TrapCardRenderer {}

enum CardRendererType {
    Creature(CreatureCardRenderer),
    Spell(SpellCardRenderer),
    Trap(TrapCardRenderer),
}

pub struct CardRenderer {
    position: Vec2,
    cost: u16,
    name: String,
    description: String,
    highlighted: bool,
    render_config: Arc<RenderConfig>,
    card_type: CardRendererType,
}

pub struct CardRendererBuilder {
    position: Option<Vec2>,
    cost: Option<u16>,
    name: Option<String>,
    description: Option<String>,
    highlighted: Option<bool>,
    render_config: Option<Arc<RenderConfig>>,
    card_type: Option<CardRendererType>,
}

impl CardRendererBuilder {
    pub fn new() -> Self {
        Self {
            position: None,
            cost: None,
            name: None,
            description: None,
            highlighted: None,
            render_config: None,
            card_type: None,
        }
    }

    pub fn position(mut self, position: Vec2) -> Self {
        self.position = Some(position);
        self
    }

    pub fn cost(mut self, cost: u16) -> Self {
        self.cost = Some(cost);
        self
    }

    pub fn name<S: Into<String>>(mut self, name: S) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn description<S: Into<String>>(mut self, description: S) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn highlighted(mut self, highlighted: bool) -> Self {
        self.highlighted = Some(highlighted);
        self
    }

    pub fn render_config(mut self, render_config: Arc<RenderConfig>) -> Self {
        self.render_config = Some(render_config);
        self
    }

    pub fn card_type(mut self, card_type: CardRendererType) -> Self {
        self.card_type = Some(card_type);
        self
    }

    // Convenience methods for specific card types
    pub fn creature(mut self, attack: u16, defense: u16) -> Self {
        self.card_type = Some(CardRendererType::Creature(CreatureCardRenderer {
            attack,
            defense,
        }));
        self
    }

    pub fn spell(mut self) -> Self {
        self.card_type = Some(CardRendererType::Spell(SpellCardRenderer {}));
        self
    }

    pub fn trap(mut self) -> Self {
        self.card_type = Some(CardRendererType::Trap(TrapCardRenderer {}));
        self
    }

    pub fn build(self) -> Result<CardRenderer, &'static str> {
        Ok(CardRenderer {
            position: self.position.ok_or("Position is required")?,
            cost: self.cost.ok_or("Cost is required")?,
            name: self.name.ok_or("Name is required")?,
            description: self.description.ok_or("Description is required")?,
            highlighted: self.highlighted.unwrap_or(false),
            render_config: self.render_config.ok_or("RenderConfig is required")?,
            card_type: self.card_type.ok_or("Card type is required")?,
        })
    }
}

impl Default for CardRendererBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl CardRenderer {
    pub fn new(
        position: Vec2,
        cost: u16,
        card_type: CardRendererType,
        name: String,
        description: String,
        highlighted: bool,
        render_config: Arc<RenderConfig>,
    ) -> Self {
        Self {
            position,
            cost,
            highlighted,
            card_type,
            description,
            name,
            render_config,
        }
    }

    pub fn builder() -> CardRendererBuilder {
        CardRendererBuilder::new()
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

        match &self.card_type {
            CardRendererType::Creature(c) => c.draw(position, &self.render_config),
            _ => {}
        };
        let description_x = position.x + 20.0;
        let description_y = position.y + self.render_config.card_height / 2.0;
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
