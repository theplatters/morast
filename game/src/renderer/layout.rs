use bevy::{
    ecs::{
        message::MessageReader,
        resource::Resource,
        system::{Res, ResMut, Single},
    },
    math::{U16Vec2, Vec2, Vec3},
    window::{Window, WindowResized},
};

use crate::board::BoardRes;

/// Tunable layout parameters. Changing these and re-running recompute
/// (or resizing the window) re-derives the whole screen layout.
#[derive(Resource, Debug, Clone)]
pub struct LayoutConfig {
    pub stats_bar_height: f32,
    pub hand_height: f32,
    pub board_margin: f32,
    pub card_width: f32,
    pub card_height: f32,
    pub card_gap: f32,
    pub end_turn_size: Vec2,
    pub end_turn_margin: f32,
    pub stats_margin: f32,
}

impl Default for LayoutConfig {
    fn default() -> Self {
        Self {
            stats_bar_height: 72.0,
            hand_height: 240.0,
            board_margin: 16.0,
            card_width: 170.0,
            card_height: 200.0,
            card_gap: 20.0,
            end_turn_size: Vec2::new(160.0, 48.0),
            end_turn_margin: 16.0,
            stats_margin: 16.0,
        }
    }
}

/// A rectangular screen region in window-space pixels.
/// Origin is the TOP-LEFT of the window, +y points down.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Region {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Region {
    pub const fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    pub fn center(&self) -> Vec2 {
        Vec2::new(self.x + self.width * 0.5, self.y + self.height * 0.5)
    }
}

/// The fully derived screen layout: three non-overlapping regions
/// (top stats bar, board, bottom hand) plus the concrete sizes derived
/// from the current window and [`LayoutConfig`].
#[derive(Resource, Debug, Clone)]
pub struct ScreenLayout {
    pub window: Vec2,
    pub stats: Region,
    pub board: Region,
    pub hand: Region,
    pub tile_size: f32,
    pub board_size: Vec2,
    pub card_size: Vec2,
    pub card_gap: f32,
    pub end_turn_size: Vec2,
    pub end_turn_margin: f32,
    pub stats_margin: f32,
}

impl Default for ScreenLayout {
    fn default() -> Self {
        Self {
            window: Vec2::ZERO,
            stats: Region::new(0.0, 0.0, 0.0, 0.0),
            board: Region::new(0.0, 0.0, 0.0, 0.0),
            hand: Region::new(0.0, 0.0, 0.0, 0.0),
            tile_size: 1.0,
            board_size: Vec2::ZERO,
            card_size: Vec2::ZERO,
            card_gap: 0.0,
            end_turn_size: Vec2::ZERO,
            end_turn_margin: 0.0,
            stats_margin: 0.0,
        }
    }
}

impl ScreenLayout {
    /// Recomputes all regions and derived sizes from the current window size.
    pub fn recompute(&mut self, config: &LayoutConfig, window: Vec2) {
        self.window = window;
        self.card_size = Vec2::new(config.card_width, config.card_height);
        self.card_gap = config.card_gap;
        self.end_turn_size = config.end_turn_size;
        self.end_turn_margin = config.end_turn_margin;
        self.stats_margin = config.stats_margin;

        // Top stats bar spans the full window width at the top.
        self.stats = Region::new(0.0, 0.0, window.x, config.stats_bar_height);

        // Bottom hand spans the full window width at the bottom.
        self.hand = Region::new(
            0.0,
            window.y - config.hand_height,
            window.x,
            config.hand_height,
        );

        // Board fills everything in between, with a margin on all sides.
        let board_top = config.stats_bar_height + config.board_margin;
        let board_bottom = window.y - config.hand_height - config.board_margin;
        self.board = Region::new(
            config.board_margin,
            board_top,
            (window.x - 2.0 * config.board_margin).max(1.0),
            (board_bottom - board_top).max(1.0),
        );

        // Largest tile size that still fits the whole grid inside the board region.
        let grid = Vec2::new(BoardRes::XSIZE as f32, BoardRes::YSIZE as f32);
        self.tile_size = (self.board.width / grid.x)
            .min(self.board.height / grid.y)
            .max(1.0);
        self.board_size = grid * self.tile_size;
    }

    /// Converts a window-space point (top-left origin, +y down) to world space
    /// (center origin, +y up).
    pub fn window_to_world(&self, p: Vec2) -> Vec2 {
        Vec2::new(p.x - self.window.x * 0.5, self.window.y * 0.5 - p.y)
    }

    /// World position of the board's top-left corner (the grid may be smaller
    /// than the region, so the grid is centered inside the region first).
    pub fn board_top_left_world(&self) -> Vec2 {
        let top_left = Vec2::new(
            self.board.x + (self.board.width - self.board_size.x) * 0.5,
            self.board.y + (self.board.height - self.board_size.y) * 0.5,
        );
        self.window_to_world(top_left)
    }

    /// Board-local position of a tile. Origin is the board's top-left, +y down.
    pub fn tile_local_position(&self, pos: U16Vec2) -> Vec3 {
        Vec3::new(
            pos.x as f32 * self.tile_size,
            -(pos.y as f32 * self.tile_size),
            1.0,
        )
    }

    /// World position of a hand card's top-center anchor point.
    pub fn hand_card_position(&self, index: usize, count: usize) -> Vec2 {
        let n = count.max(1) as f32;
        let total = n * self.card_size.x + (n - 1.0) * self.card_gap;
        let start_x = self.hand.x + (self.hand.width - total) * 0.5;
        let top_center_x =
            start_x + self.card_size.x * 0.5 + index as f32 * (self.card_size.x + self.card_gap);
        let top_center_y = self.hand.y + (self.hand.height - self.card_size.y) * 0.5;
        self.window_to_world(Vec2::new(top_center_x, top_center_y))
    }

    /// World position of the end-turn button center (right side of the stats bar).
    pub fn end_turn_center_world(&self) -> Vec2 {
        let center = Vec2::new(
            self.stats.x + self.stats.width - self.end_turn_margin - self.end_turn_size.x * 0.5,
            self.stats.center().y,
        );
        self.window_to_world(center)
    }

    /// World position of the stats text anchor (left side of the stats bar).
    pub fn stats_left_world(&self) -> Vec2 {
        let point = Vec2::new(self.stats.x + self.stats_margin, self.stats.center().y);
        self.window_to_world(point)
    }
}

// ============================================================================
// Systems
// ============================================================================

/// Computes the initial layout from the window size at startup.
pub fn compute_screen_layout_startup(
    mut layout: ResMut<ScreenLayout>,
    config: Res<LayoutConfig>,
    window: Single<&Window>,
) {
    layout.recompute(&config, Vec2::new(window.width(), window.height()));
}

/// Recomputes the layout whenever the window is resized.
pub fn compute_screen_layout_on_resize(
    mut layout: ResMut<ScreenLayout>,
    config: Res<LayoutConfig>,
    mut events: MessageReader<WindowResized>,
) {
    for e in events.read() {
        layout.recompute(&config, Vec2::new(e.width, e.height));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn default_layout() -> ScreenLayout {
        let config = LayoutConfig::default();
        let mut layout = ScreenLayout::default();
        layout.recompute(&config, Vec2::new(1600.0, 900.0));
        layout
    }

    #[test]
    fn regions_do_not_overlap() {
        let layout = default_layout();
        assert!(
            layout.stats.y + layout.stats.height <= layout.board.y,
            "stats bar and board overlap"
        );
        assert!(
            layout.board.y + layout.board.height <= layout.hand.y,
            "board and hand overlap"
        );
    }

    #[test]
    fn board_fits_in_region() {
        let layout = default_layout();
        assert!(
            layout.board_size.x <= layout.board.width + 0.001,
            "board wider than its region: {} > {}",
            layout.board_size.x,
            layout.board.width
        );
        assert!(
            layout.board_size.y <= layout.board.height + 0.001,
            "board taller than its region: {} > {}",
            layout.board_size.y,
            layout.board.height
        );
    }

    #[test]
    fn board_fills_at_least_one_dimension() {
        let layout = default_layout();
        let fills_width = layout.board_size.x >= layout.board.width - 1.0
            && layout.board_size.x <= layout.board.width + 0.001;
        let fills_height = layout.board_size.y >= layout.board.height - 1.0
            && layout.board_size.y <= layout.board.height + 0.001;
        assert!(
            fills_width || fills_height,
            "board ({:?}) fills neither dimension of its region {:?}",
            layout.board_size,
            layout.board
        );
    }

    #[test]
    fn hand_cards_are_inside_hand_region() {
        let layout = default_layout();
        for index in [0, 4] {
            let world = layout.hand_card_position(index, 5);
            // Invert window_to_world: window = (world.x + window.x/2, window.y/2 - world.y)
            let window = Vec2::new(
                world.x + layout.window.x * 0.5,
                layout.window.y * 0.5 - world.y,
            );
            assert!(
                window.x >= layout.hand.x && window.x <= layout.hand.x + layout.hand.width,
                "card {index} x out of hand region: {window:?} not in {:?}",
                layout.hand
            );
            assert!(
                window.y >= layout.hand.y && window.y <= layout.hand.y + layout.hand.height,
                "card {index} y out of hand region: {window:?} not in {:?}",
                layout.hand
            );
        }
    }

    #[test]
    fn window_to_world_maps_window_center_to_origin() {
        let layout = default_layout();
        assert_eq!(layout.window_to_world(Vec2::new(800.0, 450.0)), Vec2::ZERO);
    }
}
