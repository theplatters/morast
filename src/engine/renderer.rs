use bevy::{
    app::{Plugin, Startup},
    camera::Camera2d,
    ecs::system::Commands,
};

use crate::engine::renderer::render_config::RenderConfig;

pub mod render_config;

pub struct RendererPlugin;

impl Plugin for RendererPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.init_resource::<RenderConfig>()
            .add_systems(Startup, setup_renderer);
    }
}

pub fn setup_renderer(mut commands: Commands) {
    commands.spawn(Camera2d);
}
