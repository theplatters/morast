use bevy::{
    color::Color,
    ecs::resource::Resource,
    math::{U16Vec2, Vec2, Vec3},
};

use crate::board::BoardRes;

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
    pub hand_from_board_margin: f32,
}

impl Default for RenderConfig {
    fn default() -> Self {
        let tile_size: f32 = 55.0;
        Self {
            card_width: 200.0,
            card_height: 220.0,
            hand_y: 700.0,
            card_padding: 50.0,
            tile_size,
            hand_from_board_margin: 5.0,
            board_width: BoardRes::XSIZE as f32 * tile_size,
            board_height: BoardRes::YSIZE as f32 * tile_size,
            select_offset: Vec2::new(0.0, -10.0),
            sprite_color: Color::WHITE,
        }
    }
}

impl RenderConfig {
    #[inline]
    pub fn to_absolute_position(&self, pos: U16Vec2) -> Vec3 {
        let base_x = pos.x as f32 * self.tile_size;
        let base_y = -(pos.y as f32 * self.tile_size); // Y increases downwards

        // Offset from the top-left corner to the center of the tile
        let center_x = base_x;
        let center_y = base_y;

        Vec3::new(center_x, center_y, 0.0)
    }
}
