use bevy::{
    app::{Plugin, Startup, Update},
    asset::AssetServer,
    camera::Camera2d,
    color::{Color, Srgba},
    ecs::{
        entity::{ContainsEntity, Entity},
        error::Result,
        hierarchy::{ChildOf, Children},
        lifecycle::{Insert, Remove},
        message::MessageWriter,
        name::Name,
        observer::On,
        query::{Added, Changed, With},
        relationship::{RelatedSpawnerCommands, RelationshipTarget},
        schedule::IntoScheduleConfigs,
        system::{Commands, Query, Res, Single},
    },
    log::{info, warn},
    math::{U16Vec2, Vec2},
    picking::{
        Pickable,
        events::{Pointer, Release},
    },
    sprite::{Anchor, Sprite, Text2d},
    text::{TextColor, TextFont},
    transform::components::{GlobalTransform, Transform},
    window::Window,
};

use crate::{
    board::{
        Board, BoardRes,
        tile::{EffectsOnTile, Position, Tile},
    },
    card::{Cost, InHand, OnBoard, add_cards, card_id::CardID},
    player::{Hand, TurnPlayer},
    renderer::render_config::RenderConfig,
    turn_controller::CardClicked,
};

pub mod render_config;

pub struct RendererPlugin;

impl Plugin for RendererPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.init_resource::<RenderConfig>()
            .add_systems(
                Startup,
                (
                    setup_camera,
                    setup_creature_on_board_renderer.after(add_cards),
                    render_board.after(BoardRes::setup_board),
                    render_tiles.after(BoardRes::setup_board),
                ),
            )
            .add_systems(Update, (render_card_in_hand, render_effects_on_tile));
    }
}

// ============================================================================
// Setup Systems
// ============================================================================

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

pub fn setup_creature_on_board_renderer(
    mut commands: Commands,
    creatures: Query<Entity, With<CardID>>,
) {
    for creature in creatures {
        commands.entity(creature).observe(render_creature_on_board);
    }
}

// ============================================================================
// Board Rendering
// ============================================================================

pub fn render_board(
    board: Single<Entity, With<Board>>,
    mut commands: Commands,
    render_config: Res<RenderConfig>,
    window: Single<&Window>,
) {
    let board_color = Color::Srgba(Srgba::rgba_u8(10, 20, 30, 255));

    commands.entity(board.entity()).insert((
        Sprite {
            color: board_color,
            custom_size: Some(Vec2::new(
                render_config.board_width,
                render_config.board_height,
            )),
            ..Default::default()
        },
        Transform::from_xyz(-window.width() / 2.0, render_config.board_height / 2.0, 0.0),
        Anchor::TOP_LEFT,
    ));
}

// ============================================================================
// Tile Rendering
// ============================================================================

pub fn render_tiles(
    mut commands: Commands,
    tiles: Query<(&Position, Entity), Added<Tile>>,
    render_config: Res<RenderConfig>,
    asset_server: Res<AssetServer>,
) {
    for (&Position(U16Vec2 { x, y }), entity) in tiles.iter() {
        let tile_pos = render_config
            .to_absolute_position(U16Vec2::new(x, y))
            .with_z(1.0);

        commands.entity(entity).insert((
            Sprite {
                image: asset_server.load("tile.png"),
                custom_size: Some(render_config.tile_size * Vec2::ONE),
                ..Default::default()
            },
            Transform::from_translation(tile_pos),
            Anchor::TOP_LEFT,
            Pickable::default(),
        ));
    }
}

pub fn render_effects_on_tile(
    tiles_with_effect: Query<&EffectsOnTile, With<Tile>>,
    mut commands: Commands,
    render_config: Res<RenderConfig>,
    asset_server: Res<AssetServer>,
) {
    for tile_effects in tiles_with_effect {
        for effect in tile_effects.iter() {
            commands.entity(effect).insert((
                Sprite {
                    image: asset_server.load("effect.png"),
                    custom_size: Some(render_config.tile_size * Vec2::ONE),
                    ..Default::default()
                },
                Transform::from_xyz(0.0, 0.0, 1.0),
            ));
        }
    }
}

// ============================================================================
// Creature Rendering
// ============================================================================

pub fn render_creature_on_board(
    event: On<Insert, OnBoard>,
    creatures: Query<&OnBoard>,
    tiles: Query<(Entity, &Position, &GlobalTransform), With<Tile>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    render_config: Res<RenderConfig>,
) -> Result {
    let on_board = creatures.get(event.entity)?;
    let (tile_entity, &Position(pos), _global_transform) = tiles.get(on_board.position)?;

    info!("Rendering creature on board at position {}", pos);

    commands.entity(event.entity).insert((
        Sprite {
            image: asset_server.load("knight.png"),
            custom_size: Some(render_config.tile_size * Vec2::ONE),
            ..Default::default()
        },
        Transform::from_xyz(
            render_config.tile_size / 2.0,
            -render_config.tile_size / 2.0,
            2.0,
        ),
        ChildOf(tile_entity),
    ));

    Ok(())
}

// ============================================================================
// Card Rendering
// ============================================================================

pub fn render_card_in_hand(
    hand: Single<&Hand, (With<TurnPlayer>, Changed<Hand>)>,
    cards: Query<(&Name, &Cost), With<InHand>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    render_config: Res<RenderConfig>,
) {
    for (pos, card_entity) in hand.iter().enumerate() {
        // Skip if already rendered

        // Get the card's name and cost
        let Ok((name, cost)) = cards.get(card_entity) else {
            warn!("Card {} not found in query", card_entity);
            continue;
        };

        info!("Rendering card '{}' in hand at position {}", name, pos);

        let card_transform = calculate_card_position(pos, &render_config);

        commands
            .entity(card_entity)
            .insert((
                Sprite {
                    image: asset_server.load("card_frame.png"),
                    custom_size: Some(Vec2::new(
                        render_config.card_width,
                        render_config.card_height,
                    )),
                    ..Default::default()
                },
                card_transform,
                Pickable::default(),
                Anchor::TOP_CENTER,
            ))
            .with_children(|parent| {
                spawn_card_ui(parent, name, cost.value, &render_config, &asset_server);
            })
            .observe(on_card_clicked)
            .observe(on_card_removed_from_hand);
    }
}

// ============================================================================
// Helper Functions
// ============================================================================

fn calculate_card_position(position: usize, render_config: &RenderConfig) -> Transform {
    let offset = render_config.board_height / 2.0 + render_config.hand_from_board_margin;
    let card_width = render_config.card_width;

    Transform::from_xyz(
        position as f32 * (card_width + render_config.card_padding)
            - render_config.board_width / 2.0,
        -offset,
        2.0,
    )
}

fn spawn_card_ui(
    parent: &mut RelatedSpawnerCommands<'_, bevy::prelude::ChildOf>,
    name: &str,
    cost: u16,
    render_config: &RenderConfig,
    asset_server: &AssetServer,
) {
    let card_height = render_config.card_height;
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");

    // Card name
    parent.spawn((
        Text2d::new(name),
        TextFont {
            font: font.clone(),
            font_size: 16.0,
            ..Default::default()
        },
        TextColor(Color::BLACK),
        Anchor::TOP_CENTER,
        Transform::from_xyz(0.0, -card_height * 0.4, 0.1),
    ));

    // Cost badge
    parent.spawn((
        Text2d::new(format!("{}", cost)),
        TextFont {
            font,
            font_size: 24.0,
            ..Default::default()
        },
        TextColor(Color::srgb(1.0, 0.8, 0.0)), // Gold color
        Anchor::TOP_CENTER,
        Transform::from_xyz(0.0, -card_height * 0.15, 0.1),
    ));
}

// ============================================================================
// Event Observers
// ============================================================================

fn on_card_clicked(
    click: On<Pointer<Release>>,
    mut event_writer: MessageWriter<CardClicked>,
    hands: Query<&Hand, With<TurnPlayer>>,
) {
    let Ok(hand) = hands.single() else {
        return;
    };

    if let Some(pos) = hand
        .iter()
        .position(|card_in_hand| card_in_hand == click.entity)
    {
        info!("Card at position {} clicked", pos);
        event_writer.write(CardClicked(pos));
    }
}

fn on_card_removed_from_hand(
    trigger: On<Remove, InHand>,
    mut commands: Commands,
    children_query: Query<&Children>,
) {
    let entity = trigger.entity;
    info!(
        "InHand component removed from card {}, cleaning up rendering",
        entity
    );

    if let Ok(mut entity_commands) = commands.get_entity(entity) {
        // Remove the rendering components
        info!("Removing rendering components for card in hand");
        entity_commands.remove::<(Sprite, Transform, Anchor, Pickable)>();

        // Despawn all children (text entities)
        if let Ok(children) = children_query.get(entity) {
            for child in children.iter() {
                commands.entity(child).despawn();
            }
        }
    }
}
