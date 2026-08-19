use bevy::{
    app::{Plugin, Startup, Update},
    asset::AssetServer,
    color::Color,
    ecs::{
        component::Component,
        query::With,
        system::{Commands, Query, Res, Single},
    },
    sprite::{Anchor, Text2d},
    text::{TextColor, TextFont},
    transform::components::Transform,
    window::Window,
};

use crate::player::{Player, PlayerResources, TurnPlayer};

#[derive(Component)]
pub struct StatsText;

pub struct StatsPlugin;

impl Plugin for StatsPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(Startup, spawn_stats_display)
            .add_systems(Update, update_stats_display);
    }
}

fn spawn_stats_display(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window: Single<&Window>,
) {
    commands.spawn((
        Text2d::new("Player 0  Health 10"),
        TextFont {
            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
            font_size: 20.0,
            ..Default::default()
        },
        TextColor(Color::WHITE),
        Transform::from_xyz(-window.width() / 2.0 + 10.0, window.height() / 2.0 - 10.0, 3.0),
        Anchor::TOP_LEFT,
        StatsText,
    ));
}

fn update_stats_display(
    mut text: Single<&mut Text2d, With<StatsText>>,
    player: Single<(&Player, &PlayerResources), With<TurnPlayer>>,
) {
    let (player, resources) = *player;
    text.0 = format!("Player {}  Health {}", player.number, resources.health);
}
