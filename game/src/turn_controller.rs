use bevy::{math::U16Vec2, prelude::*};

use crate::{
    actions::execute::{AbilityCursor, AwaitingChoice, AwaitingChoiceKind},
    board::{
        movement::MoveRequest,
        tile::{Occupant, Position},
    },
    card::{InHand, OnBoard, Selected},
    def::effect::EffectDef,
    player::{Hand, Player, TurnPlayer},
};

// ============================================================================
// STATES
// ============================================================================

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum TurnPhase {
    Start,
    #[default]
    Main,
    End,
}

#[derive(SubStates, Default, Debug, Clone, PartialEq, Eq, Hash)]
#[source(TurnPhase = TurnPhase::Main)]
pub enum TurnState {
    #[default]
    Idle,
    CardSelected,
    AwaitingInputs,
    FigureSelected,
    EndTurn,
}

// ============================================================================
// Routed Messages (state-scoped intents)
// ============================================================================

#[derive(Message, Clone, Copy, Debug)]
pub enum IdleIntent {
    IdleCardClick { card_index: usize },
    IdleBoardClick { entity: Entity, position: U16Vec2 },
}

#[derive(Message, Clone, Copy, Debug)]
pub enum CardSelectedIntent {
    CardSelectedBoardClick { entity: Entity, position: U16Vec2 },
}

#[derive(Message, Clone, Copy, Debug)]
pub struct FigureSelectedBoardClick {
    pub entity: Entity,
    pub position: U16Vec2,
}

#[derive(Message, Clone, Copy, Debug)]
pub struct AwaitingInputsBoardClick {
    pub entity: Entity,
    pub position: U16Vec2,
}

#[derive(Message, Clone, Debug)]
pub enum ChoiceMade {
    Option(usize),
    Entities(Vec<Entity>),
    Cancelled,
}

// ============================================================================
// PLAY COMMANDS
// ============================================================================

#[derive(Message)]
pub struct CardPlayRequested {
    pub card: Entity,
    pub hand_position: usize,
    pub position: U16Vec2,
}

#[derive(Message)]
pub struct TargetingComplete;

#[derive(Message)]
pub struct EndTurn;

// ============================================================================
// Raw input messages
// ============================================================================

#[derive(Message)]
pub struct BoardClicked {
    pub entity: Entity,
    pub position: U16Vec2,
}

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
            .init_state::<TurnPhase>()
            .init_state::<TurnState>()
            // Raw input messages
            .add_message::<BoardClicked>()
            .add_message::<CardClicked>()
            .add_message::<EndTurnPressed>()
            .add_message::<CancelPressed>()
            // Routed intent messages
            .add_message::<IdleIntent>()
            .add_message::<CardSelectedIntent>()
            .add_message::<FigureSelectedBoardClick>()
            .add_message::<AwaitingInputsBoardClick>()
            .add_message::<ChoiceMade>()
            // Commands
            .add_message::<CardPlayRequested>()
            .add_message::<TargetingComplete>()
            // Systems
            .add_systems(
                Update,
                (
                    handle_end_turn_input,
                    handle_cancel_input,
                    // Routers MUST run before state handlers
                    (card_click_system, board_click_system),
                    handle_idle_state.run_if(in_state(TurnState::Idle)),
                    handle_card_selected.run_if(in_state(TurnState::CardSelected)),
                    handle_figure_selected.run_if(in_state(TurnState::FigureSelected)),
                    handle_awaiting_inputs
                        .run_if(in_state(TurnState::AwaitingInputs)),
                    handle_choice_made
                        .run_if(in_state(TurnState::AwaitingInputs)),
                ),
            )
            // Cleanup on state exit
            .add_systems(OnExit(TurnState::CardSelected), cleanup_selection)
            .add_systems(OnExit(TurnState::FigureSelected), cleanup_selection)
            .add_systems(OnExit(TurnState::AwaitingInputs), cleanup_selection)
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
    selected: Query<Entity, Or<(With<Selected>, With<Origin>)>>,
    mut commands: Commands,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        for entity in &selected {
            commands.entity(entity).remove::<Selected>();
            commands.entity(entity).remove::<Origin>();
        }
        match current_state.get() {
            TurnState::AwaitingInputs | TurnState::CardSelected | TurnState::FigureSelected => {
                next_state.set(TurnState::Idle);
            }
            _ => {}
        }
    }
}

// ============================================================================
// Router systems (only these read BoardClicked/CardClicked)
// ============================================================================

#[derive(Component)]
struct Origin;

pub fn board_click_system(
    mut board_clicks: MessageReader<BoardClicked>,
    state: Res<State<TurnState>>,
    mut idle_intents: MessageWriter<IdleIntent>,
    mut card_selected_intents: MessageWriter<CardSelectedIntent>,
    mut figure_selected_clicks: MessageWriter<FigureSelectedBoardClick>,
    mut awaiting_inputs_clicks: MessageWriter<AwaitingInputsBoardClick>,
) {
    for click in board_clicks.read() {
        match state.get() {
            TurnState::Idle => {
                idle_intents.write(IdleIntent::IdleBoardClick {
                    entity: click.entity,
                    position: click.position,
                });
            }
            TurnState::CardSelected => {
                card_selected_intents.write(CardSelectedIntent::CardSelectedBoardClick {
                    entity: click.entity,
                    position: click.position,
                });
            }
            TurnState::FigureSelected => {
                figure_selected_clicks.write(FigureSelectedBoardClick {
                    entity: click.entity,
                    position: click.position,
                });
            }
            TurnState::AwaitingInputs => {
                awaiting_inputs_clicks.write(AwaitingInputsBoardClick {
                    entity: click.entity,
                    position: click.position,
                });
            }
            TurnState::EndTurn => {}
        }
    }
}

pub fn card_click_system(
    mut card_clicks: MessageReader<CardClicked>,
    state: Res<State<TurnState>>,
    mut idle_intents: MessageWriter<IdleIntent>,
) {
    for CardClicked(card_index) in card_clicks.read() {
        match state.get() {
            TurnState::Idle => {
                idle_intents.write(IdleIntent::IdleCardClick {
                    card_index: *card_index,
                });
            }
            // define behavior later if you want (switch selection, etc.)
            TurnState::CardSelected
            | TurnState::AwaitingInputs
            | TurnState::FigureSelected
            | TurnState::EndTurn => {}
        }
    }
}

// ============================================================================
// STATE HANDLERS (now consume routed messages)
// ============================================================================

fn handle_idle_state(
    mut intents: MessageReader<IdleIntent>,
    mut next_state: ResMut<NextState<TurnState>>,
    mut commands: Commands,
    occupants: Query<(&Occupant, Entity)>,
    hand: Single<&Hand, With<TurnPlayer>>,
) {
    // Preserve your original priority: card click beats board click.
    // If multiple intents arrive, process in order but return after card selection.
    for intent in intents.read().copied() {
        match intent {
            IdleIntent::IdleCardClick { card_index } => {
                let Some(card_entity) = hand.get_card(card_index) else {
                    warn!("Invalid card index: {}", card_index);
                    continue;
                };
                commands.entity(card_entity).insert(Selected);
                next_state.set(TurnState::CardSelected);
                return;
            }
            IdleIntent::IdleBoardClick { entity, .. } => {
                if let Ok((occupant, _)) = occupants.get(entity) {
                    info!("Selected {} transferring to FigureSelected state", entity);
                    commands.entity(occupant.get()).insert(Origin);
                    next_state.set(TurnState::FigureSelected);
                    // don’t return; if you want “first click wins”, you can return here
                }
            }
        }
    }
}

fn handle_card_selected(
    mut intents: MessageReader<CardSelectedIntent>,
    mut play_commands: MessageWriter<CardPlayRequested>,
    mut next_state: ResMut<NextState<TurnState>>,
    player_hands: Query<(&Hand, &Player), With<TurnPlayer>>,
    selected_card: Single<Entity, (With<Selected>, With<InHand>)>,
) {
    let Some(&CardSelectedIntent::CardSelectedBoardClick { position, .. }) = intents.read().next()
    else {
        return;
    };

    let Ok((hand, _)) = player_hands.single() else {
        warn!("Could not find player hand");
        next_state.set(TurnState::Idle);
        return;
    };

    let Some(hand_position) = hand.iter().position(|r| r == *selected_card) else {
        warn!("Could not find card in players hand");
        next_state.set(TurnState::Idle);
        return;
    };

    info!("Sending Play command");
    play_commands.write(CardPlayRequested {
        card: *selected_card,
        hand_position,
        position,
    });

    next_state.set(TurnState::Idle);
}

fn handle_figure_selected(
    mut board_clicks: MessageReader<FigureSelectedBoardClick>,
    mut play_commands: MessageWriter<MoveRequest>,
    mut next_state: ResMut<NextState<TurnState>>,
    selected_figure: Query<(Entity, &OnBoard), (With<Origin>, With<OnBoard>)>,
    tiles: Query<&Position>,
) {
    let Some(FigureSelectedBoardClick {
        position: next_position,
        ..
    }) = board_clicks.read().next().copied()
    else {
        return;
    };

    let Ok((entity, &OnBoard { position: tile })) = selected_figure.single() else {
        warn!("FigureSelected state but no Origin entity found");
        next_state.set(TurnState::Idle);
        return;
    };

    let Ok(&Position(from)) = tiles.get(tile) else {
        warn!("Could not resolve 'from' position");
        next_state.set(TurnState::Idle);
        return;
    };

    info!("Sending move command from {} to {}", from, next_position);

    play_commands.write(MoveRequest {
        entity,
        from,
        to: next_position,
    });
    next_state.set(TurnState::Idle);
}

// ============================================================================
// CLEANUP
// ============================================================================

fn handle_awaiting_inputs(
    mut board_clicks: MessageReader<AwaitingInputsBoardClick>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut choice_made: MessageWriter<ChoiceMade>,
) {
    // Board click -> select the clicked entity.
    for click in board_clicks.read() {
        choice_made.write(ChoiceMade::Entities(vec![click.entity]));
        return;
    }

    // Digit keys 1-9 -> option index.
    for i in 1..=9 {
        if keyboard.just_pressed(match i {
            1 => KeyCode::Digit1,
            2 => KeyCode::Digit2,
            3 => KeyCode::Digit3,
            4 => KeyCode::Digit4,
            5 => KeyCode::Digit5,
            6 => KeyCode::Digit6,
            7 => KeyCode::Digit7,
            8 => KeyCode::Digit8,
            9 => KeyCode::Digit9,
            _ => unreachable!(),
        }) {
            choice_made.write(ChoiceMade::Option(i - 1));
            return;
        }
    }
}

fn handle_choice_made(
    mut choices: MessageReader<ChoiceMade>,
    awaiting: Query<(Entity, &AwaitingChoice)>,
    mut cursors: Query<&mut AbilityCursor>,
    mut commands: Commands,
    mut next_state: ResMut<NextState<TurnState>>,
) {
    for choice in choices.read() {
        let Some((awaiting_entity, awaiting)) = awaiting.iter().next() else {
            continue;
        };

        match choice {
            ChoiceMade::Cancelled => {
                // Abort the ability to avoid leaking cursors.
                if let Ok(mut cursor_entity) = commands.get_entity(awaiting.cursor) {
                    cursor_entity.despawn();
                }
                commands.entity(awaiting_entity).remove::<AwaitingChoice>();
                next_state.set(TurnState::Idle);
                return;
            }
            ChoiceMade::Option(index) => {
                let AwaitingChoiceKind::Options(options) = &awaiting.kind else {
                    continue;
                };
                let Some(option) = options.get(*index) else {
                    warn!("Choice option {} out of range", index);
                    continue;
                };
                // Find the matching ChoiceOptionDef in the cursor's next effect.
                let Ok(mut cursor) = cursors.get_mut(awaiting.cursor) else {
                    continue;
                };
                let Some(EffectDef::Choose { options: defs }) = cursor.stack.first().cloned() else {
                    continue;
                };
                let Some(def) = defs.iter().find(|o| &o.label == option) else {
                    continue;
                };
                cursor.stack.remove(0);
                cursor.stack.splice(0..0, def.effects.clone());
                commands.entity(awaiting_entity).remove::<AwaitingChoice>();
                next_state.set(TurnState::Idle);
                return;
            }
            ChoiceMade::Entities(entities) => {
                let AwaitingChoiceKind::Entities = &awaiting.kind else {
                    continue;
                };
                let Ok(mut cursor) = cursors.get_mut(awaiting.cursor) else {
                    continue;
                };
                cursor.context.chosen_entities.clone_from(entities);
                cursor.context.pending_targets = Some(entities.clone());
                if let Some(&first) = entities.first() {
                    cursor.context.current_target = Some(first);
                }
                commands.entity(awaiting_entity).remove::<AwaitingChoice>();
                next_state.set(TurnState::Idle);
                return;
            }
        }
    }
}

fn cleanup_selection(
    selected: Query<Entity, Or<(With<Selected>, With<Origin>)>>,
    mut commands: Commands,
) {
    for e in &selected {
        commands.entity(e).remove::<Selected>();
        commands.entity(e).remove::<Origin>();
    }
}

fn on_turn_end(mut next_state: ResMut<NextState<TurnState>>) {
    info!("Turn ended");
    next_state.set(TurnState::Idle);
}

/// Call this when starting a new turn
pub fn reset_turn(mut next_state: ResMut<NextState<TurnState>>) {
    next_state.set(TurnState::Idle);
}
