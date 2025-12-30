use std::collections::HashMap;

use bevy::{
    math::{I16Vec2, U16Vec2},
    prelude::*,
};
use effect::Effect;
use place_error::BoardError;

use crate::game::{
    board::{
        effect::*,
        movement::*,
        placement::{place_card, CardPlayed},
        tile::*,
    },
    card::{
        creature::{AttackPattern, Attacks},
        OnBoard,
    },
    components::{Health, Owner},
    events::{EffectAdded, EffectRemoved},
    player::Player,
    turn_controller::{BoardClicked, TurnPhase},
};

pub mod effect;
pub mod movement;
pub mod place_error;
pub mod placement;
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
    pub const XSIZE: u16 = 24;
    pub const YSIZE: u16 = 12;
    pub fn setup_board(mut commands: Commands) {
        let mut tiles = HashMap::new();
        let player_base_positions = [
            U16Vec2::new(2, Self::YSIZE / 2),
            U16Vec2::new(Self::XSIZE - 3, Self::YSIZE / 2),
        ];
        let board_id = commands.spawn((Board,)).id();

        for x in 0..Self::XSIZE {
            for y in 0..Self::YSIZE {
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
            size: U16Vec2::new(Self::XSIZE, Self::YSIZE),
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

pub fn update_attack_values(
    tiles: Query<&Position>,
    creatures: Query<(Entity, &AttackPattern, &OnBoard), Changed<OnBoard>>,
    mut commands: Commands,
) -> Result {
    for (card_entity, pattern, on_board) in &creatures {
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

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app
            // Register the Messages (events)
            .add_message::<EffectRequested>()
            .add_message::<MoveRequest>()
            .add_message::<CardPlayed>()
            // Setup systems (run once at startup)
            .add_systems(
                Startup,
                (
                    BoardRes::setup_board,
                    BoardRes::setup_player_bases.after(BoardRes::setup_board),
                ),
            )
            .add_systems(
                Update,
                (
                    handle_movement,
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
