use bevy::{
    app::{Plugin, Startup, Update},
    asset::AssetServer,
    camera::Camera2d,
    color::Color,
    ecs::{
        component::Component,
        entity::Entity,
        error::Result,
        hierarchy::Children,
        lifecycle::{Insert, Remove, RemovedComponents, Replace},
        message::MessageWriter,
        name::Name,
        observer::On,
        query::{Added, Changed, With},
        relationship::RelationshipTarget,
        system::{Commands, Query, Res},
    },
    log::{info, warn},
    math::{U16Vec2, Vec2},
    picking::{
        events::{Click, Pointer},
        Pickable,
    },
    sprite::{Anchor, Sprite, Text2d},
    text::{TextColor, TextFont},
    transform::components::Transform,
};

use crate::{
    engine::renderer::render_config::RenderConfig,
    game::{
        board::{
            tile::{Position, Tile},
            Board, BoardRes,
        },
        card::{creature::Attacks, Cost, InHand, OnBoard},
        player::{Hand, TurnPlayer},
        turn_controller::CardClicked,
    },
};

pub mod render_config;

pub struct RendererPlugin;

impl Plugin for RendererPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.init_resource::<RenderConfig>()
            .add_systems(Startup, setup_renderer)
            .add_systems(Update, (render_card_in_hand, render_tiles));
    }
}

pub fn setup_renderer(mut commands: Commands) {
    commands.spawn(Camera2d);
}

pub fn render_creature(
    event: On<Insert, OnBoard>,
    tiles: Query<(&Position, Entity)>,
    creatures: Query<(&OnBoard)>,
    board: Res<BoardRes>,
    mut commands: Commands,
) -> Result {
    let on_board = creatures.get(event.entity)?;

    Ok(())
}

pub fn render_tiles(
    mut commands: Commands,
    tiles: Query<(&Position, Entity), Added<Tile>>,
    render_config: Res<RenderConfig>,
    asset_server: Res<AssetServer>,
) {
    for (&Position(U16Vec2 { x, y }), entity) in tiles.iter() {
        commands.entity(entity).insert((
            Sprite {
                image: asset_server.load("tile.png"),
                custom_size: Some(render_config.tile_size * Vec2::ONE),
                ..Default::default()
            },
            Anchor::TOP_LEFT,
            Transform::from_translation(
                render_config
                    .to_absolute_position(U16Vec2::new(x, y))
                    .with_z(1.0),
            ),
            Pickable::default(),
        ));
    }
}

#[derive(Component)]
pub struct AttackOnTileText;

pub fn render_card_in_hand(
    hands: Query<&Hand, (With<TurnPlayer>, Changed<Hand>)>,
    cards: Query<(&Name, &Cost), With<InHand>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    render_config: Res<RenderConfig>,
) {
    let offset = render_config.board_height;
    let card_width = render_config.card_width;
    let card_height = render_config.card_height;

    for hand in &hands {
        for (pos, card) in hand.iter().enumerate() {
            info!(
                "Attaching rendering component to card {} with position {}",
                card, pos
            );

            // Get the card's name and cost
            let Ok((name, cost)) = cards.get(card) else {
                warn!("Card {} not found in query", card);
                continue;
            };

            commands
                .get_entity(card)
                .unwrap()
                .insert((
                    Sprite {
                        image: asset_server.load("card_frame.png"),
                        custom_size: Some(Vec2::new(card_width, card_height)),
                        ..Default::default()
                    },
                    Anchor::TOP_CENTER,
                    Transform::from_xyz(
                        pos as f32 * card_width - render_config.board_width / 2.,
                        -offset,
                        2.,
                    ),
                    Pickable::default(),
                ))
                .with_children(|parent| {
                    // Spawn card name text at the top
                    parent.spawn((
                        Text2d::new(name),
                        TextFont {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 16.0,
                            ..Default::default()
                        },
                        TextColor(Color::BLACK),
                        Transform::from_xyz(0.0, -card_height * 0.4, 0.1),
                    ));

                    // Spawn cost text at the top-right corner
                    parent.spawn((
                        Text2d::new(format!("Cost: {}", cost.value)),
                        TextFont {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 20.0,
                            ..Default::default()
                        },
                        TextColor(Color::srgb(1.0, 0.8, 0.0)), // Gold color for cost
                        Transform::from_xyz(0.0, -card_height * 0.2, 0.1),
                    ));
                })
                .observe(
                    |click: On<Pointer<Click>>,
                     mut event_writer: MessageWriter<CardClicked>,
                     hands: Query<&Hand, With<TurnPlayer>>| {
                        if let Some(pos) = hands
                            .single()
                            .unwrap()
                            .iter()
                            .position(|card_in_hand| card_in_hand == click.entity)
                        {
                            info!("Card clicked");
                            event_writer.write(CardClicked(pos));
                        }
                    },
                )
                .observe(
                    |trigger: On<Remove, InHand>,
                     mut commands: Commands,
                     children_query: Query<&Children>| {
                        let entity = trigger.entity;
                        info!(
                            "InHand component removed from card {}, cleaning up rendering",
                            entity
                        );

                        if let Ok(mut entity_commands) = commands.get_entity(entity) {
                            // Remove the rendering components
                            entity_commands.remove::<(Sprite, Transform, Anchor, Pickable)>();

                            // Despawn all children (text entities)
                            if let Ok(children) = children_query.get(entity) {
                                for child in children.iter() {
                                    commands.entity(child).despawn();
                                }
                            }
                        }
                    },
                );
        }
    }
}
