use bevy::prelude::*;

use macroquad::{input::KeyCode, math::U16Vec2};

use crate::game::{
    actions::{
        action_prototype::{NeedsTargeting, Pending, ReadyToExecute},
        targeting::TargetingType,
    },
    board::{tile::Occupant, Board, MoveRequest},
    card::{InHand, OnBoard, Selected},
    player::{Hand, Player, TurnPlayer},
};

// ============================================================================
// STATES
// ============================================================================
#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum TurnState {
    #[default]
    Idle,
    CardSelected,
    AwaitingInputs,
    FigureSelected,
    EndTurn,
}

// ============================================================================
// PLAY COMMANDS
// ============================================================================

#[derive(Message)]
pub struct CardPlayRequested {
    pub card: Entity,
    pub hand_position: usize,
    pub pos: U16Vec2,
}

#[derive(Message)]
pub struct TargetingComplete;

#[derive(Message)]
pub struct EndTurn;

// ============================================================================
// EVENTS / MESSAGES
// ============================================================================

#[derive(Message)]
pub struct BoardClicked(pub U16Vec2);

#[derive(Message)]
pub struct CardClicked(pub usize);

#[derive(Message)]
pub struct EndTurnPressed;

#[derive(Message)]
pub struct CancelPressed;

// ============================================================================
// PLUGIN
// ============================================================================

pub struct TurnControllerPlugin;

impl Plugin for TurnControllerPlugin {
    fn build(&self, app: &mut App) {
        app
            // States
            .init_state::<TurnState>()
            // Events
            .add_message::<BoardClicked>()
            .add_message::<CardClicked>()
            .add_message::<EndTurnPressed>()
            .add_message::<CancelPressed>()
            // Systems
            .add_systems(Update, handle_end_turn_input)
            .add_systems(Update, handle_cancel_input)
            .add_systems(Update, handle_idle_state.run_if(in_state(TurnState::Idle)))
            .add_systems(Update, handle_action.run_if(in_state(TurnState::Idle)))
            .add_systems(
                Update,
                handle_card_selected.run_if(in_state(TurnState::CardSelected)),
            )
            .add_systems(
                Update,
                handle_awaiting_inputs.run_if(in_state(TurnState::AwaitingInputs)),
            )
            .add_systems(
                Update,
                handle_figure_selected.run_if(in_state(TurnState::FigureSelected)),
            )
            // Cleanup on state exit
            .add_systems(OnExit(TurnState::CardSelected), cleanup_card_selection)
            .add_systems(OnExit(TurnState::FigureSelected), cleanup_figure_selection)
            .add_systems(OnEnter(TurnState::EndTurn), on_turn_end);
    }
}

fn handle_end_turn_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<TurnState>>,
) {
    if keyboard.just_pressed(KeyCode::Enter) {
        info!("Enter pressed - ending turn");
        next_state.set(TurnState::EndTurn);
    }
}

fn handle_cancel_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    current_state: Res<State<TurnState>>,
    mut next_state: ResMut<NextState<TurnState>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        match current_state.get() {
            TurnState::AwaitingInputs => {
                next_state.set(TurnState::Idle);
            }
            TurnState::CardSelected | TurnState::FigureSelected => {
                next_state.set(TurnState::Idle);
            }
            _ => {}
        }
    }
}

// ============================================================================
// STATE HANDLERS
// ============================================================================

fn handle_idle_state(
    mut board_clicks: MessageReader<BoardClicked>,
    mut card_clicks: MessageReader<CardClicked>,
    mut next_state: ResMut<NextState<TurnState>>,
    mut commands: Commands,
    board: Res<Board>,
    occupants: Query<(&Occupant, Entity)>,
    hand: Query<&Hand, With<TurnPlayer>>,
) {
    // Check for card selection first
    for CardClicked(card_index) in card_clicks.read() {
        let hand = hand.single().unwrap();
        let card_entity = hand.get_card(*card_index).unwrap();
        commands.entity(card_entity).insert(Selected);
        next_state.set(TurnState::CardSelected);
        return;
    }

    // Check for figure selection
    for BoardClicked(pos) in board_clicks.read() {
        if let Some(tile_entity) = board.get_tile(pos) {
            if let Ok((_, ent)) = occupants.get(tile_entity) {
                commands.entity(ent).insert(Selected);
                next_state.set(TurnState::FigureSelected);
                return;
            }
        }
    }
}

fn handle_card_selected(
    mut board_clicks: MessageReader<BoardClicked>,
    mut play_commands: MessageWriter<CardPlayRequested>,
    mut next_state: ResMut<NextState<TurnState>>,
    player_hands: Query<(&Hand, &Player), With<TurnPlayer>>,
    selected_card: Query<&InHand, With<Selected>>,
) {
    let selected_card = selected_card.single().unwrap();
    for BoardClicked(pos) in board_clicks.read() {
        let hand_position = selected_card.hand_index;

        // Get card from hand
        let Ok((hand, _)) = player_hands.single() else {
            warn!("Could not find player hand");
            next_state.set(TurnState::Idle);
            return;
        };

        let Some(card_entity) = hand.get_card(hand_position) else {
            warn!("Invalid card index: {}", hand_position);
            next_state.set(TurnState::Idle);
            return;
        };

        play_commands.write(CardPlayRequested {
            card: card_entity,
            hand_position,
            pos: *pos,
        });

        next_state.set(TurnState::Idle);
        return;
    }
}

fn handle_figure_selected(
    mut board_clicks: MessageReader<BoardClicked>,
    mut play_commands: MessageWriter<MoveRequest>,
    mut next_state: ResMut<NextState<TurnState>>,
    selected_cards: Query<(Entity, &OnBoard), With<Selected>>,
) {
    for BoardClicked(next_position) in board_clicks.read() {
        let (entity, &OnBoard { position: from }) = selected_cards.single().unwrap();

        info!("Sending move command from {} to {}", from, next_position);

        play_commands.write(MoveRequest {
            entity,
            from,
            to: *next_position,
        });
        next_state.set(TurnState::Idle);
        return;
    }
}

fn handle_awaiting_inputs(
    mut board_clicks: MessageReader<BoardClicked>,
    mut play_commands: MessageWriter<TargetingComplete>,
    mut next_state: ResMut<NextState<TurnState>>,
    selected_cards: Query<&mut Selected, With<OnBoard>>,
    board: Res<Board>,
    actions: Query<(Entity, &TargetingType), With<NeedsTargeting>>,
    tiles: Query<&Occupant>,
    mut commands: Commands,
) {
    let (action_entity, targeting) = actions.single().unwrap();
    if selected_cards.iter().len() as u8 == targeting.required_targets() {
        play_commands.write(TargetingComplete);
        info!("Sending Play Command");
        commands.entity(action_entity).remove::<NeedsTargeting>();
        commands.entity(action_entity).insert(ReadyToExecute);
        next_state.set(TurnState::Idle);
    }

    for BoardClicked(click_pos) in board_clicks.read() {
        let tile = board.get_tile(click_pos).unwrap();
        let card = tiles.get(tile).unwrap().0;
        commands.entity(card).insert(Selected);
    }
}

pub fn handle_action(
    mut commands: Commands,
    actions: Query<(Entity, &TargetingType), With<Pending>>,
    mut next_state: ResMut<NextState<TurnState>>,
) {
    let Some((entity, targeting_type)) = actions.iter().next() else {
        return;
    };

    let needs_targeting = targeting_type.requires_selection();

    commands
        .entity(entity)
        .remove::<Pending>()
        .insert_if(NeedsTargeting, || needs_targeting)
        .insert_if(ReadyToExecute, || !needs_targeting);

    if needs_targeting {
        next_state.set(TurnState::AwaitingInputs);
    }
}

// ============================================================================
// CLEANUP SYSTEMS
// ============================================================================

fn cleanup_card_selection(
    selected_cards: Query<Entity, (With<Selected>, With<InHand>)>,
    mut commands: Commands,
) {
    for card in &selected_cards {
        commands.entity(card).remove::<Selected>();
    }
}

fn cleanup_figure_selection(
    selected_cards: Query<Entity, (With<Selected>, With<OnBoard>)>,
    mut commands: Commands,
) {
    for card in &selected_cards {
        commands.entity(card).remove::<Selected>();
    }
}

fn on_turn_end(mut next_state: ResMut<NextState<TurnState>>) {
    // Handle turn end logic here
    // Then reset to idle for next turn
    info!("Turn ended");
}

/// Call this when starting a new turn
pub fn reset_turn(mut next_state: ResMut<NextState<TurnState>>) {
    next_state.set(TurnState::Idle);
}
