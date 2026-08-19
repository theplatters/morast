use bevy::{
    app::{Plugin, Startup, Update},
    asset::AssetServer,
    color::Color,
    ecs::{
        component::Component,
        query::With,
        schedule::{common_conditions::resource_changed, IntoScheduleConfigs},
        system::{Commands, Query, Res, Single},
    },
    math::Vec3,
    sprite::{Anchor, Text2d},
    text::{TextColor, TextFont},
    transform::components::Transform,
};

use crate::{
    player::{Player, PlayerResources, TurnPlayer},
    renderer::layout::{ScreenLayout, compute_screen_layout_startup},
};

#[derive(Component)]
pub struct StatsText;

pub struct StatsPlugin;

impl Plugin for StatsPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(Startup, spawn_stats_display.after(compute_screen_layout_startup))
            .add_systems(
                Update,
                (
                    update_stats_display,
                    apply_stats_layout.run_if(resource_changed::<ScreenLayout>),
                ),
            );
    }
}

fn spawn_stats_display(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    layout: Res<ScreenLayout>,
) {
    let pos = layout.stats_left_world();

    commands.spawn((
        Text2d::new(""),
        TextFont {
            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
            font_size: 20.0,
            ..Default::default()
        },
        TextColor(Color::WHITE),
        Transform::from_xyz(pos.x, pos.y, 3.0),
        Anchor::CENTER_LEFT,
        StatsText,
    ));
}

fn update_stats_display(
    mut text: Single<&mut Text2d, With<StatsText>>,
    players: Query<(&Player, &PlayerResources)>,
    turn: Query<&Player, With<TurnPlayer>>,
) {
    let turn_number = turn.iter().next().map(|player| player.number);

    let mut players: Vec<(&Player, &PlayerResources)> = players.iter().collect();
    players.sort_by_key(|(player, _)| player.number);

    let mut parts = Vec::new();
    for (player, resources) in players {
        let prefix = if Some(player.number) == turn_number {
            "▶ "
        } else {
            ""
        };
        parts.push(format!(
            "{prefix}P{}  HP {}/{}  Gold {}",
            player.number, resources.health, resources.max_health, resources.gold
        ));
    }
    text.0 = parts.join("    ");
}

fn apply_stats_layout(
    mut text: Single<&mut Transform, With<StatsText>>,
    layout: Res<ScreenLayout>,
) {
    let pos = layout.stats_left_world();
    text.translation = Vec3::new(pos.x, pos.y, 3.0);
}
