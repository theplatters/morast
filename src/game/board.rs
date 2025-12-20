use std::{collections::HashMap, fmt::Display};

use bevy::{
    math::{I16Vec2, U16Vec2},
    prelude::*,
    sprite::Anchor,
};
use effect::Effect;
use place_error::BoardError;

use crate::{
    engine::renderer::render_config::RenderConfig,
    game::{
        board::{
            effect::{EffectDuration, EffectType},
            tile::{Occupant, Position, Tile, TileBundel},
        },
        card::{
            creature::{AttackPattern, Attacks, BaseMovementPoints, MovementPattern},
            CreatureCard, CurrentAttack, CurrentMovementPoints, InHand, OnBoard,
        },
        components::{Health, Owner},
        events::{CardMoved, EffectAdded, EffectRemoved},
        player::{Player, TurnPlayer},
        turn_controller::{BoardClicked, CardPlayRequested, TurnPhase},
    },
};

pub mod effect;
pub mod place_error;
pub mod tile;

#[derive(Bundle)]
pub struct PlayerBaseBundle {
    player_base: PlayerBase,
    health: Health,
}

impl PlayerBaseBundle {
    fn new() -> Self {
        Self {
            player_base: PlayerBase,
            health: Health::player_base_health(),
        }
    }
}

#[derive(Component, Default)]
pub struct Board;

#[derive(Component, Default)]
pub struct PlayerBase;

#[derive(Debug, Resource)]
pub struct BoardRes {
    tiles: HashMap<U16Vec2, Entity>,
    size: U16Vec2,
    player_base_positions: [U16Vec2; 2],
}

impl BoardRes {
    pub fn setup_board(
        mut commands: Commands,
        render_config: Res<RenderConfig>,
        asset_server: Res<AssetServer>,
        window: Query<&Window>,
    ) {
        let window = window.single().expect("Could not find window");
        let x_size: u16 = 24;
        let y_size: u16 = 12;

        let mut tiles = HashMap::new();
        let player_base_positions = [
            U16Vec2::new(2, y_size / 2),
            U16Vec2::new(x_size - 3, y_size / 2),
        ];
        let board_id = commands
            .spawn((
                Board,
                Transform::from_xyz(window.width() / -2., window.height() / 2., 0.),
                Visibility::default(),
                Anchor::TOP_LEFT,
            ))
            .id();

        for x in 0..x_size {
            for y in 0..y_size {
                let position = U16Vec2::new(x, y);
                let tile_id = commands
                    .spawn((TileBundel::default(), ChildOf(board_id), Position(position)))
                    .observe(
                        |click: On<Pointer<Release>>,
                         mut board_clicked: MessageWriter<BoardClicked>,
                         tiles: Query<&Position, With<Tile>>| {
                            info!("Tile clicked");
                            let &Position(position) = tiles
                                .get(click.entity)
                                .expect("Clicked thing is somehow not a tile");
                            board_clicked.write(BoardClicked {
                                position,
                                entity: click.entity,
                            });
                        },
                    )
                    .id();
                tiles.insert(position, tile_id);
            }
        }

        commands.insert_resource(BoardRes {
            tiles,
            size: U16Vec2::new(x_size, y_size),
            player_base_positions,
        });
    }

    pub fn setup_player_bases(
        mut commands: Commands,
        board: Res<BoardRes>,
        players: Query<Entity, With<Player>>,
    ) {
        for (player_entity, pos) in players.iter().zip(board.player_base_positions.into_iter()) {
            let tile = board
                .get_tile(&pos)
                .ok_or(BoardError::TileNotFound)
                .expect("This is a setup error and should never happen");
            let base_entity = commands
                .spawn((
                    PlayerBaseBundle::new(),
                    Owner(player_entity),
                    OnBoard { position: tile },
                ))
                .id();
        }
    }

    pub fn add_relative_tile(&self, pos: U16Vec2, reltile: I16Vec2) -> Option<U16Vec2> {
        let new_x = pos.x.checked_add_signed(reltile.x)?;
        let new_y = pos.y.checked_add_signed(reltile.y)?;

        let new_pos = U16Vec2::new(new_x, new_y);

        if new_pos.x < self.size.x && new_pos.y < self.size.y {
            Some(new_pos)
        } else {
            None
        }
    }

    pub fn iter(&self) -> std::collections::hash_map::Iter<U16Vec2, Entity> {
        self.tiles.iter()
    }

    pub fn width(&self) -> u16 {
        self.size.x
    }

    pub fn height(&self) -> u16 {
        self.size.y
    }

    pub fn get_tile(&self, pos: &U16Vec2) -> Option<Entity> {
        self.tiles.get(pos).copied()
    }
}

pub(crate) fn refresh_movement_points(
    creatures: Query<(&mut CurrentMovementPoints, &Owner, &BaseMovementPoints), With<CreatureCard>>,
    player: Query<Entity, With<TurnPlayer>>,
) {
    for (mut movement, owner, base_movement) in creatures.into_iter() {
        if owner.0 == player.single().expect("No turn player found") {
            movement.remaining_points = base_movement.0
        }
    }
}

pub fn update_attack_values(
    mut tiles: Query<&Position>,
    creatures: Query<
        (
            Entity,
            &CurrentAttack,
            &AttackPattern,
            &mut Attacks,
            &OnBoard,
        ),
        Changed<OnBoard>,
    >,
    players: Query<&Player>,
    board: Res<BoardRes>,
    mut commands: Commands,
) -> Result {
    for (card_entity, attack_value, pattern, attack_pattern, on_board) in &creatures {
        let pos = tiles.get(on_board.position).unwrap();
        commands
            .entity(card_entity)
            .insert(Attacks(pattern.into_tiles(pos)));
    }
    Ok(())
}

#[derive(Message)]
pub struct EffectRequested {
    effect: Effect,
    indices: Vec<U16Vec2>,
}

fn add_effect_to_tile(
    mut commands: Commands,
    board: Res<BoardRes>,
    mut effects: MessageReader<EffectRequested>,
    mut effects_added: MessageWriter<EffectAdded>,
) -> Result {
    for effect_play_event in effects.read() {
        for index in &effect_play_event.indices {
            let tile = board.get_tile(index).ok_or(BoardError::Index)?;
            commands.spawn((ChildOf(tile), effect_play_event.effect));
            effects_added.write(EffectAdded {
                effect: effect_play_event.effect,
                tile,
            });
        }
    }
    Ok(())
}

#[derive(Message)]
pub struct MoveRequest {
    pub entity: Entity,
    pub from: U16Vec2,
    pub to: U16Vec2,
}

pub fn is_validate_move() -> bool {
    true
}

#[derive(Debug)]
pub enum MoveValidationError {
    InsufficientMovementPoints { required: u16, available: u16 },
    InvalidMovePattern { from: U16Vec2, to: U16Vec2 },
    Occupied,
}

impl Display for MoveValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl std::error::Error for MoveValidationError {}

fn check_valid_move_and_get_cost(
    from: U16Vec2,
    to: U16Vec2,
    movement_points: u16,
    movement_pattern: &MovementPattern,
    tile_has_slow: bool,
) -> Result<u16, MoveValidationError> {
    // Check movement points requirement
    let required_points = if tile_has_slow { 2 } else { 1 };

    if movement_points < required_points {
        return Err(MoveValidationError::InsufficientMovementPoints {
            required: required_points,
            available: movement_points,
        });
    }

    let from_signed = from.as_i16vec2();
    let to_signed = to.as_i16vec2();
    let delta = to_signed - from_signed;

    let pattern_valid = movement_pattern.0.contains(&delta);

    if !pattern_valid {
        return Err(MoveValidationError::InvalidMovePattern { from, to });
    }
    Ok(required_points)
}

pub fn handle_movement(
    mut commands: Commands,
    mut move_requests: MessageReader<MoveRequest>,
    mut move_completed: MessageWriter<CardMoved>,
    mut creatures: Query<
        (&OnBoard, &mut CurrentMovementPoints, &MovementPattern),
        With<CreatureCard>,
    >,
    board: Res<BoardRes>,
    occupied: Query<&Occupant>,
    effects: Query<(&EffectType, &ChildOf)>,
) -> Result {
    for event in move_requests.read() {
        let (mut creature, mut movement, pattern) = creatures
            .get_mut(event.entity)
            .map_err(|_| BoardError::CardNotFound)?;
        let old_pos = event.from;

        let old_tile = board.get_tile(&old_pos).ok_or(BoardError::TileNotFound)?;
        let new_tile = board.get_tile(&event.to).ok_or(BoardError::TileNotFound)?;

        if occupied.contains(new_tile) {
            return Err(BoardError::InvalidMove(MoveValidationError::Occupied).into());
        }

        let tile_has_slow = effects
            .iter()
            .any(|(ef, co)| co.0 == old_tile && *ef == EffectType::Slow);

        let cost = check_valid_move_and_get_cost(
            old_pos,
            event.to,
            movement.remaining_points,
            pattern,
            tile_has_slow,
        )
        .map_err(BoardError::InvalidMove)?;

        commands
            .entity(event.entity)
            .insert(OnBoard { position: new_tile });
        movement.remaining_points -= cost;

        move_completed.write(CardMoved {
            card: event.entity,
            from: old_pos,
            to: event.to,
        });
    }
    Ok(())
}

pub fn update_position_on_move(
    mut cards: Query<(&OnBoard, &mut Transform), Changed<OnBoard>>,
    mut tiles: Query<&Position, With<Tile>>,
    render_config: Res<RenderConfig>,
) {
    for (pos, mut trans) in cards.iter_mut() {
        let &Position(pos) = tiles.get(pos.position).unwrap();
        trans.translation = render_config.to_absolute_position(pos);
    }
}

pub fn decrease_effect_duration(
    mut commands: Commands,
    mut effects: Query<(&mut EffectDuration, &EffectType, Entity, &ChildOf), With<EffectType>>,
    mut effect_removed: MessageWriter<EffectRemoved>,
) {
    for (mut duration, effect_type, effect_entity, tile) in effects.iter_mut() {
        duration.decrease();
        if duration.over() {
            commands.entity(effect_entity).despawn();
            effect_removed.write(EffectRemoved {
                effect: *effect_type,
                tile: tile.0,
            });
        }
    }
}

pub fn place_card(
    mut card_place_requests: MessageReader<CardPlayRequested>,
    free_tiles: Query<&Tile, Without<Occupant>>,
    board: Res<BoardRes>,
    mut commands: Commands,
) {
    for card_place_request in card_place_requests.read() {
        let tile = board.get_tile(&card_place_request.position).unwrap();
        if free_tiles.contains(tile) {
            commands
                .entity(card_place_request.card)
                .remove::<InHand>()
                .insert(OnBoard { position: tile });
        }
    }
}

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app
            // Register the Messages (events)
            .add_message::<EffectRequested>()
            .add_message::<MoveRequest>()
            // Setup systems (run once at startup)
            .add_systems(
                Startup,
                (
                    BoardRes::setup_board,
                    BoardRes::setup_player_bases.after(BoardRes::setup_board),
                ),
            )
            // Update systems that run every frame
            .add_systems(
                Update,
                (
                    handle_movement,
                    // Movement handling
                    update_position_on_move.after(handle_movement),
                    // Effect handling
                    add_effect_to_tile,
                    decrease_effect_duration,
                    place_card,
                    update_attack_values,
                ),
            )
            // System that runs at the start of each turn
            .add_systems(OnEnter(TurnPhase::Start), refresh_movement_points);
    }
}
