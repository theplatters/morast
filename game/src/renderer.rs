use bevy::{
    app::{Plugin, Startup, Update},
    asset::AssetServer,
    camera::Camera2d,
    color::{Color, Srgba},
    ecs::{
        component::Component,
        entity::{ContainsEntity, Entity},
        error::Result,
        hierarchy::{ChildOf, Children},
        lifecycle::{Insert, Remove},
        message::MessageWriter,
        name::Name,
        observer::On,
        query::{Added, Changed, With, Without},
        relationship::{RelatedSpawnerCommands, RelationshipTarget},
        schedule::{
            common_conditions::resource_changed,
            IntoScheduleConfigs,
        },
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
};

use crate::{
    board::{
        Board, BoardRes,
        tile::{EffectsOnTile, Position, Tile},
    },
    card::{Cost, InHand, OnBoard, card_id::CardID},
    player::{Hand, TurnPlayer},
    renderer::layout::{
        LayoutConfig, ScreenLayout, compute_screen_layout_on_resize,
        compute_screen_layout_startup,
    },
    turn_controller::{CardClicked, EndTurnPressed},
};

pub mod layout;

pub struct RendererPlugin;

impl Plugin for RendererPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.init_resource::<LayoutConfig>()
            .init_resource::<ScreenLayout>()
            .add_systems(
                Startup,
                (
                    setup_camera,
                    compute_screen_layout_startup,
                    render_board
                        .after(compute_screen_layout_startup)
                        .after(BoardRes::setup_board),
                    render_tiles
                        .after(compute_screen_layout_startup)
                        .after(BoardRes::setup_board),
                    spawn_end_turn_button.after(compute_screen_layout_startup),
                ),
            )
            .add_systems(
                Update,
                (
                    compute_screen_layout_on_resize,
                    apply_board_layout.run_if(resource_changed::<ScreenLayout>),
                    apply_tiles_layout.run_if(resource_changed::<ScreenLayout>),
                    apply_end_turn_layout.run_if(resource_changed::<ScreenLayout>),
                    apply_creature_layout.run_if(resource_changed::<ScreenLayout>),
                    spawn_hand_card_visuals,
                    position_hand_cards,
                    render_effects_on_tile,
                ),
            );
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
    layout: Res<ScreenLayout>,
) {
    let board_color = Color::Srgba(Srgba::rgba_u8(10, 20, 30, 255));
    let board_top_left = layout.board_top_left_world();

    commands.entity(board.entity()).insert((
        Sprite {
            color: board_color,
            custom_size: Some(layout.board_size),
            ..Default::default()
        },
        Transform::from_xyz(board_top_left.x, board_top_left.y, 0.0),
        Anchor::TOP_LEFT,
    ));
}

// ============================================================================
// Tile Rendering
// ============================================================================

pub fn render_tiles(
    mut commands: Commands,
    tiles: Query<(&Position, Entity), Added<Tile>>,
    layout: Res<ScreenLayout>,
    asset_server: Res<AssetServer>,
) {
    for (&Position(U16Vec2 { x, y }), entity) in tiles.iter() {
        commands.entity(entity).insert((
            Sprite {
                image: asset_server.load("tile.png"),
                custom_size: Some(layout.tile_size * Vec2::ONE),
                ..Default::default()
            },
            Transform::from_translation(layout.tile_local_position(U16Vec2::new(x, y))),
            Anchor::TOP_LEFT,
            Pickable::default(),
        ));
    }
}

pub fn render_effects_on_tile(
    tiles_with_effect: Query<&EffectsOnTile, With<Tile>>,
    mut commands: Commands,
    layout: Res<ScreenLayout>,
    asset_server: Res<AssetServer>,
) {
    for tile_effects in tiles_with_effect {
        for effect in tile_effects.iter() {
            commands.entity(effect).insert((
                Sprite {
                    image: asset_server.load("effect.png"),
                    custom_size: Some(layout.tile_size * Vec2::ONE),
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
    layout: Res<ScreenLayout>,
) -> Result {
    let on_board = creatures.get(event.entity)?;
    let (tile_entity, &Position(pos), _global_transform) = tiles.get(on_board.position)?;

    info!("Rendering creature on board at position {}", pos);

    commands.entity(event.entity).insert((
        Sprite {
            image: asset_server.load("knight.png"),
            custom_size: Some(layout.tile_size * Vec2::ONE),
            ..Default::default()
        },
        Transform::from_xyz(
            layout.tile_size / 2.0,
            -layout.tile_size / 2.0,
            2.0,
        ),
        ChildOf(tile_entity),
    ));

    Ok(())
}

// ============================================================================
// Card Rendering
// ============================================================================

/// Marker for cards that already have their hand visual spawned.
#[derive(Component)]
struct HandCardVisual;

/// Marker for the text labels spawned as part of a hand card's visual.
#[derive(Component)]
struct HandCardLabel;

fn spawn_hand_card_visuals(
    hand: Single<&Hand, (With<TurnPlayer>, Changed<Hand>)>,
    cards: Query<(&Name, &Cost), (With<InHand>, Without<HandCardVisual>)>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    layout: Res<ScreenLayout>,
) {
    let count = hand.iter().len();

    for (pos, card_entity) in hand.iter().enumerate() {
        // Get the card's name and cost
        let Ok((name, cost)) = cards.get(card_entity) else {
            warn!("Card {} not found in query", card_entity);
            continue;
        };

        info!("Rendering card '{}' in hand at position {}", name, pos);

        commands
            .entity(card_entity)
            .insert((
                Sprite {
                    image: asset_server.load("card_frame.png"),
                    custom_size: Some(layout.card_size),
                    ..Default::default()
                },
                Transform::from_translation(layout.hand_card_position(pos, count).extend(2.0)),
                Pickable::default(),
                Anchor::TOP_CENTER,
                HandCardVisual,
            ))
            .with_children(|parent| {
                spawn_card_ui(parent, name, cost.value, &layout, &asset_server);
            })
            .observe(on_card_clicked)
            .observe(on_card_removed_from_hand);
    }
}

// ============================================================================
// End Turn Button
// ============================================================================

#[derive(Component)]
struct EndTurnButton;

fn spawn_end_turn_button(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    layout: Res<ScreenLayout>,
) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let button_center = layout.end_turn_center_world();

    commands
        .spawn((
            Sprite {
                color: Color::srgb(0.15, 0.35, 0.15),
                custom_size: Some(layout.end_turn_size),
                ..Default::default()
            },
            Transform::from_xyz(button_center.x, button_center.y, 3.0),
            Pickable::default(),
            Name::new("EndTurnButton"),
            EndTurnButton,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text2d::new("End Turn"),
                TextFont {
                    font,
                    font_size: 24.0,
                    ..Default::default()
                },
                TextColor(Color::WHITE),
                // The sprite uses the default CENTER anchor, so the text centers
                // on the button.
                Transform::from_xyz(0.0, 0.0, 0.1),
            ));
        })
        .observe(on_end_turn_clicked);
}

// ============================================================================
// Layout Apply Systems (run when the ScreenLayout resource changes)
// ============================================================================

fn apply_board_layout(
    board: Single<(&mut Sprite, &mut Transform), With<Board>>,
    layout: Res<ScreenLayout>,
) {
    let (mut sprite, mut transform) = board.into_inner();
    let board_top_left = layout.board_top_left_world();
    sprite.custom_size = Some(layout.board_size);
    transform.translation = board_top_left.extend(0.0);
}

fn apply_tiles_layout(
    mut tiles: Query<(&mut Sprite, &mut Transform, &Position), With<Tile>>,
    layout: Res<ScreenLayout>,
) {
    for (mut sprite, mut transform, &Position(pos)) in &mut tiles {
        sprite.custom_size = Some(layout.tile_size * Vec2::ONE);
        transform.translation = layout.tile_local_position(pos);
    }
}

fn apply_end_turn_layout(
    mut button: Single<&mut Transform, With<EndTurnButton>>,
    layout: Res<ScreenLayout>,
) {
    button.translation = layout.end_turn_center_world().extend(3.0);
}

fn apply_creature_layout(
    mut creatures: Query<(&mut Sprite, &mut Transform), With<OnBoard>>,
    layout: Res<ScreenLayout>,
) {
    let offset = Vec2::new(layout.tile_size / 2.0, -layout.tile_size / 2.0);
    for (mut sprite, mut transform) in &mut creatures {
        sprite.custom_size = Some(layout.tile_size * Vec2::ONE);
        transform.translation = offset.extend(2.0);
    }
}

fn position_hand_cards(
    hand: Single<&Hand, With<TurnPlayer>>,
    layout: Res<ScreenLayout>,
    mut transforms: Query<&mut Transform>,
) {
    let count = hand.iter().len();

    for (i, card) in hand.iter().enumerate() {
        if let Ok(mut tf) = transforms.get_mut(card) {
            tf.translation = layout.hand_card_position(i, count).extend(2.0);
        }
    }
}

// ============================================================================
// Helper Functions
// ============================================================================

fn spawn_card_ui(
    parent: &mut RelatedSpawnerCommands<'_, bevy::prelude::ChildOf>,
    name: &str,
    cost: u16,
    layout: &ScreenLayout,
    asset_server: &AssetServer,
) {
    let card_height = layout.card_size.y;
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
        HandCardLabel,
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
        HandCardLabel,
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

fn on_end_turn_clicked(
    _click: On<Pointer<Release>>,
    mut writer: MessageWriter<EndTurnPressed>,
) {
    info!("End turn button clicked");
    writer.write(EndTurnPressed);
}

fn on_card_removed_from_hand(
    trigger: On<Remove, InHand>,
    mut commands: Commands,
    children_query: Query<&Children>,
    labels: Query<(), With<HandCardLabel>>,
) {
    let entity = trigger.entity;
    info!(
        "InHand component removed from card {}, cleaning up rendering",
        entity
    );

    if let Ok(mut entity_commands) = commands.get_entity(entity) {
        // Remove the rendering components
        entity_commands.remove::<(Sprite, Transform, Anchor, Pickable, HandCardVisual)>();
    }

    // Despawn only the visual labels (name/cost); keep ability children alive.
    if let Ok(children) = children_query.get(entity) {
        for child in children.iter() {
            if labels.get(child).is_ok() {
                commands.entity(child).despawn();
            }
        }
    }
}
