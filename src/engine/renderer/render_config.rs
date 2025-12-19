use bevy::{
    color::Color,
    ecs::resource::Resource,
    math::{U16Vec2, Vec2, Vec3},
};

#[derive(Resource, Debug)]
pub struct RenderConfig {
    pub card_width: f32,
    pub card_height: f32,
    pub hand_y: f32,
    pub card_padding: f32,
    pub tile_size: f32,
    pub board_width: f32,
    pub board_height: f32,
    pub select_offset: Vec2,
    pub sprite_color: Color,
}

impl Default for RenderConfig {
    fn default() -> Self {
        Self {
            card_width: 200.0,
            card_height: 220.0,
            hand_y: 700.0,
            card_padding: 10.0,
            tile_size: 50.0,
            board_width: 1200.0,
            board_height: 350.0,
            select_offset: Vec2::new(0.0, -10.0),
            sprite_color: Color::WHITE,
        }
    }
}

impl RenderConfig {
    #[inline]
    pub fn to_absolute_position(&self, pos: U16Vec2) -> Vec3 {
        (Vec2::new(1.0, -1.0) * pos.as_vec2() * self.tile_size).extend(0.0)
    }
}
