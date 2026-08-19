//! Ability executor: drives `AbilityData` components through their effect stack,
//! emitting `EntityEvent`s and pausing for player choices / manual targeting.

use bevy::{math::I16Vec2, prelude::*};

use crate::{
    GameRng,
    actions::{
        AbilityData, Action,
        conditions::Condition,
        targeting::{AnyTargetSelector, FinalizeEffect},
        value_source::ValueEvalParams,
    },
    board::{
        placement::CardPlayed,
        tile::Occupant,
    },
    card::{CreatureCard, OnBoard},
    components::Owner,
    def::{effect::EffectDef, trigger::TriggerDef},
    error::GameError,
    events::TurnEnd,
    player::TurnPlayer,
    turn_controller::TurnState,
};

use super::{
    targeting::filters::FilterParams,
    value_source::{StatModifier, ValueSource},
    AddGold, ApplyEffect, DealDamage, DestroyCreature, DiscardCards, DrawCards, HealCreature,
    Mill, ModifyStats, MoveCreature, ReturnToHand,
};

/// Remaining effect stack for a running ability.
#[derive(Component, Debug, Clone)]
pub struct AbilityCursor {
    pub stack: Vec<EffectDef>,
    pub context: AbilityContext,
}

/// Runtime context for an ability (current target, chosen entities, etc.).
#[derive(Component, Debug, Clone, Default)]
pub struct AbilityContext {
    pub current_target: Option<Entity>,
    pub chosen_entities: Vec<Entity>,
    /// Targets supplied by a manual-input resume for the effect currently at the front of the stack.
    pub pending_targets: Option<Vec<Entity>>,
}

/// A request for the active player to make a choice.
#[derive(Message, Clone, Debug)]
pub enum ChoiceRequested {
    /// Pick one of the labelled ability options.
    Options {
        cursor: Entity,
        labels: Vec<String>,
    },
    /// Pick one or more entities from the candidate list.
    Entities {
        cursor: Entity,
        candidates: Vec<Entity>,
    },
}

/// Component marking a cursor that is waiting for a player choice.
#[derive(Component, Debug, Clone)]
pub struct AwaitingChoice {
    pub cursor: Entity,
    pub kind: AwaitingChoiceKind,
}

#[derive(Debug, Clone)]
pub enum AwaitingChoiceKind {
    Options(Vec<String>),
    Entities,
}

/// Start executing an ability by spawning a cursor on its ability entity.
/// If the ability's condition is not met, the ability is skipped.
pub fn start_ability(
    commands: &mut Commands,
    ability_entity: Entity,
    ability: &AbilityData,
    caster: Entity,
    params: &mut ValueEvalParams,
) {
    let condition = match Condition::try_from(&ability.0.condition) {
        Ok(c) => c,
        Err(e) => {
            warn!("Failed to evaluate ability condition: {}", e);
            return;
        }
    };

    if !condition.eval(params, caster) {
        return;
    }

    commands.entity(ability_entity).insert(AbilityCursor {
        stack: ability.0.effects.clone(),
        context: AbilityContext::default(),
    });
}

/// Driver system: processes ability cursors each frame.
pub fn drive_abilities(
    mut commands: Commands,
    mut cursors: Query<(Entity, &AbilityData, &mut AbilityCursor, &Action), Without<AwaitingChoice>>,
    filter_params: FilterParams,
    mut rng: ResMut<GameRng>,
    occupied: Query<&Occupant>,
    creatures: Query<&OnBoard, With<CreatureCard>>,
    mut next_state: ResMut<NextState<TurnState>>,
    mut choice_requests: MessageWriter<ChoiceRequested>,
) {
    let mut value_params = filter_params.as_value_params(&mut *rng);

    for (ability_entity, _ability, mut cursor, action) in &mut cursors {
        if cursor.stack.is_empty() {
            commands.entity(ability_entity).remove::<AbilityCursor>();
            continue;
        }

        let effect = cursor.stack.remove(0);
        match execute_effect(
            &mut commands,
            ability_entity,
            &mut cursor,
            action.caster,
            &effect,
            &mut value_params,
            &occupied,
            &creatures,
        ) {
            Ok(EffectResult::Done) => {}
            Ok(EffectResult::NeedsChoice(request)) => {
                let kind = match &request {
                    ChoiceRequested::Options { labels, .. } => {
                        AwaitingChoiceKind::Options(labels.clone())
                    }
                    ChoiceRequested::Entities { .. } => AwaitingChoiceKind::Entities,
                };
                commands.entity(ability_entity).insert(AwaitingChoice {
                    cursor: ability_entity,
                    kind,
                });
                choice_requests.write(request);
                // Re-insert the effect we removed so it is re-tried after the choice.
                cursor.stack.insert(0, effect);
                next_state.set(TurnState::AwaitingInputs);
                break;
            }
            Err(e) => {
                warn!("Ability execution error: {}", e);
            }
        }
    }
}

#[derive(Debug)]
enum EffectResult {
    Done,
    NeedsChoice(ChoiceRequested),
}

fn execute_effect(
    commands: &mut Commands,
    ability_entity: Entity,
    cursor: &mut AbilityCursor,
    caster: Entity,
    effect: &EffectDef,
    params: &mut ValueEvalParams,
    occupied: &Query<&Occupant>,
    creatures: &Query<&OnBoard, With<CreatureCard>>,
) -> Result<EffectResult, GameError> {
    match effect {
        EffectDef::If {
            condition,
            then,
            otherwise,
        } => {
            let cond: crate::actions::conditions::Condition = condition.try_into()?;
            let branch = if cond.eval(params, caster) {
                then.clone()
            } else {
                otherwise.clone()
            };
            cursor.stack.splice(0..0, branch);
            Ok(EffectResult::Done)
        }
        EffectDef::Choose { options } => {
            let labels: Vec<String> = options.iter().map(|o| o.label.clone()).collect();
            Ok(EffectResult::NeedsChoice(ChoiceRequested::Options {
                cursor: ability_entity,
                labels,
            }))
        }
        _ => {
            // If a manual-input resume supplied targets, use them directly.
            if let Some(targets) = cursor.context.pending_targets.take() {
                if let Some(&first) = targets.first() {
                    cursor.context.current_target = Some(first);
                    params.current_target = Some(first);
                }
                emit_effect_events(
                    commands,
                    effect,
                    params,
                    caster,
                    &targets,
                    occupied,
                    creatures,
                )?;
                return Ok(EffectResult::Done);
            }

            let selector = effect_selector(effect)?;
            let candidates = selector.selection().find_suitable(params, caster);
            let validated: Vec<Entity> = candidates
                .into_iter()
                .filter(|&c| selector.validation().validate(params, caster, c))
                .collect();

            match selector.selection().finalize(&validated) {
                FinalizeEffect::None => Ok(EffectResult::Done),
                FinalizeEffect::AwaitInput => {
                    Ok(EffectResult::NeedsChoice(ChoiceRequested::Entities {
                        cursor: ability_entity,
                        candidates: validated,
                    }))
                }
                FinalizeEffect::ExecuteSingle(e) => {
                    cursor.context.current_target = Some(e);
                    params.current_target = Some(e);
                    emit_effect_events(commands, effect, params, caster, &[e], occupied, creatures)?;
                    Ok(EffectResult::Done)
                }
                FinalizeEffect::ExecuteAll => {
                    if let Some(&first) = validated.first() {
                        cursor.context.current_target = Some(first);
                        params.current_target = Some(first);
                    }
                    emit_effect_events(
                        commands,
                        effect,
                        params,
                        caster,
                        &validated,
                        occupied,
                        creatures,
                    )?;
                    Ok(EffectResult::Done)
                }
                FinalizeEffect::ExecuteSubset(list) => {
                    if let Some(&first) = list.first() {
                        cursor.context.current_target = Some(first);
                        params.current_target = Some(first);
                    }
                    emit_effect_events(commands, effect, params, caster, &list, occupied, creatures)?;
                    Ok(EffectResult::Done)
                }
                FinalizeEffect::AwaitingValueSource { value_source } => {
                    let count = value_source.eval(params, caster) as usize;
                    let list: Vec<Entity> = validated.into_iter().take(count).collect();
                    if let Some(&first) = list.first() {
                        cursor.context.current_target = Some(first);
                        params.current_target = Some(first);
                    }
                    emit_effect_events(commands, effect, params, caster, &list, occupied, creatures)?;
                    Ok(EffectResult::Done)
                }
            }
        }
    }
}

fn effect_selector(effect: &EffectDef) -> Result<AnyTargetSelector, GameError> {
    match effect {
        EffectDef::DealDamage { selector, .. } => {
            AnyTargetSelector::try_from(selector).map_err(Into::into)
        }
        EffectDef::Heal { selector, .. } => {
            AnyTargetSelector::try_from(selector).map_err(Into::into)
        }
        EffectDef::DrawCards { player, .. } => {
            AnyTargetSelector::try_from(player).map_err(Into::into)
        }
        EffectDef::AddGold { player, .. } => {
            AnyTargetSelector::try_from(player).map_err(Into::into)
        }
        EffectDef::ApplyEffect { selector, .. } => {
            AnyTargetSelector::try_from(selector).map_err(Into::into)
        }
        EffectDef::DestroyCreature { selector } => {
            AnyTargetSelector::try_from(selector).map_err(Into::into)
        }
        EffectDef::ModifyStats { selector, .. } => {
            AnyTargetSelector::try_from(selector).map_err(Into::into)
        }
        EffectDef::MoveCreature { selector, .. } => {
            AnyTargetSelector::try_from(selector).map_err(Into::into)
        }
        EffectDef::DiscardCards { player, .. } => {
            AnyTargetSelector::try_from(player).map_err(Into::into)
        }
        EffectDef::Mill { player, .. } => {
            AnyTargetSelector::try_from(player).map_err(Into::into)
        }
        EffectDef::ReturnToHand { selector } => {
            AnyTargetSelector::try_from(selector).map_err(Into::into)
        }
        EffectDef::If { .. } | EffectDef::Choose { .. } => Err(GameError::ActionError(
            "branch/choice effects do not have a selector",
        )),
    }
}

fn emit_effect_events(
    commands: &mut Commands,
    effect: &EffectDef,
    params: &mut ValueEvalParams,
    caster: Entity,
    targets: &[Entity],
    occupied: &Query<&Occupant>,
    creatures: &Query<&OnBoard, With<CreatureCard>>,
) -> Result<(), GameError> {
    let _ = caster;
    match effect {
        EffectDef::DealDamage { amount, .. } => {
            let amount = eval_value(amount, params, caster)?;
            for &target in targets {
                commands
                    .entity(target)
                    .trigger(|e| DealDamage::new(amount, e));
            }
        }
        EffectDef::Heal { amount, .. } => {
            let amount = eval_value(amount, params, caster)?;
            for &target in targets {
                commands
                    .entity(target)
                    .trigger(|e| HealCreature::new(amount, e));
            }
        }
        EffectDef::DrawCards { amount, .. } => {
            let amount = eval_value(amount, params, caster)?;
            for &target in targets {
                commands
                    .entity(target)
                    .trigger(|e| DrawCards::new(amount, e));
            }
        }
        EffectDef::AddGold { amount, .. } => {
            let amount = eval_value(amount, params, caster)?;
            for &target in targets {
                commands
                    .entity(target)
                    .trigger(|e| AddGold::new(amount, e));
            }
        }
        EffectDef::ApplyEffect { effect, duration, .. } => {
            for &target in targets {
                commands
                    .entity(target)
                    .trigger(|e| ApplyEffect::new(*effect, *duration, e));
            }
        }
        EffectDef::DestroyCreature { .. } => {
            for &target in targets {
                commands
                    .entity(target)
                    .trigger(|e| DestroyCreature::new(e));
            }
        }
        EffectDef::ModifyStats { modifier, .. } => {
            let modifier: StatModifier = modifier.into();
            for &target in targets {
                commands
                    .entity(target)
                    .trigger(|e| ModifyStats::new(e, modifier));
            }
        }
        EffectDef::MoveCreature {
            direction,
            absolute,
            ..
        } => {
            let direction = I16Vec2::new(direction[0], direction[1]);
            for &tile in targets {
                let Some(creature) = find_creature_on_tile(tile, occupied, creatures) else {
                    continue;
                };
                commands
                    .entity(creature)
                    .trigger(|e| MoveCreature::new(direction, *absolute, e));
            }
        }
        EffectDef::DiscardCards { amount, .. } => {
            let amount = eval_value(amount, params, caster)?;
            for &target in targets {
                commands
                    .entity(target)
                    .trigger(|e| DiscardCards::new(amount, e));
            }
        }
        EffectDef::Mill { amount, .. } => {
            let amount = eval_value(amount, params, caster)?;
            for &target in targets {
                commands
                    .entity(target)
                    .trigger(|e| Mill::new(amount, e));
            }
        }
        EffectDef::ReturnToHand { .. } => {
            for &target in targets {
                commands
                    .entity(target)
                    .trigger(|e| ReturnToHand::new(e));
            }
        }
        EffectDef::If { .. } | EffectDef::Choose { .. } => unreachable!(),
    }
    Ok(())
}

fn eval_value(
    value: &crate::def::value::ValueDef,
    params: &mut ValueEvalParams,
    caster: Entity,
) -> Result<u16, GameError> {
    let source: ValueSource = value.try_into()?;
    Ok(source.eval(params, caster))
}

fn find_creature_on_tile(
    tile: Entity,
    occupied: &Query<&Occupant>,
    creatures: &Query<&OnBoard, With<CreatureCard>>,
) -> Option<Entity> {
    let occupant = occupied.get(tile).ok()?;
    let creature = occupant.get();
    creatures.get(creature).ok()?;
    Some(creature)
}

/// System that starts OnPlay abilities when a card is played.
pub fn on_card_played(
    mut plays: MessageReader<CardPlayed>,
    abilities: Query<(Entity, &AbilityData, &Action)>,
    mut commands: Commands,
    filter_params: FilterParams,
    mut rng: ResMut<GameRng>,
) {
    let mut value_params = filter_params.as_value_params(&mut *rng);
    for play in plays.read() {
        for (ability_entity, ability, action) in &abilities {
            if action.caster != play.card {
                continue;
            }
            if ability.0.trigger != TriggerDef::OnPlay {
                continue;
            }
            start_ability(
                &mut commands,
                ability_entity,
                ability,
                action.caster,
                &mut value_params,
            );
        }
    }
}

/// Stub for OnReveal trap triggers. Trap reveal events are not yet wired; log a warning.
pub fn on_reveal_stub(
    abilities: Query<(Entity, &AbilityData, &Action)>,
) {
    for (_entity, ability, _action) in &abilities {
        if ability.0.trigger == TriggerDef::OnReveal {
            warn!("OnReveal ability found but trap reveal flow is not yet implemented");
        }
    }
}

/// System that triggers OnTurnEnd abilities at the end of the turn.
pub fn on_turn_end(
    mut commands: Commands,
    turn_player: Query<Entity, With<TurnPlayer>>,
    on_board: Query<(Entity, &Owner), With<CreatureCard>>,
    abilities: Query<(Entity, &AbilityData, &Action)>,
    mut turn_end_writer: MessageWriter<TurnEnd>,
    filter_params: FilterParams,
    mut rng: ResMut<GameRng>,
) {
    let Ok(player) = turn_player.single() else {
        return;
    };

    let mut value_params = filter_params.as_value_params(&mut *rng);
    for (ability_entity, ability, action) in &abilities {
        if ability.0.trigger != TriggerDef::OnTurnEnd {
            continue;
        }
        let Ok((_, owner)) = on_board.get(action.caster) else {
            continue;
        };
        if owner.0 != player {
            continue;
        }
        start_ability(
            &mut commands,
            ability_entity,
            ability,
            action.caster,
            &mut value_params,
        );
    }

    turn_end_writer.write(TurnEnd);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::actions::conditions::CompareOp;
    use crate::def::{
        card::PatternDef,
        condition::ConditionDef,
        effect::EffectDef,
        selector::{CardinalityDef, SelectionDef, SelectorDef, SelectorKindDef},
        trigger::{AbilityDef, TriggerDef},
        value::ValueDef,
    };
    use bevy::math::I16Vec2;

    #[test]
    fn if_branch_selection() {
        let then_branch = vec![EffectDef::DestroyCreature {
            selector: SelectorDef {
                kind: SelectorKindDef::Creature,
                cardinality: CardinalityDef::Single,
                selection: SelectionDef::Caster,
                filters: vec![],
            },
        }];
        let otherwise_branch = vec![EffectDef::DrawCards {
            player: SelectorDef {
                kind: SelectorKindDef::Player,
                cardinality: CardinalityDef::Single,
                selection: SelectionDef::Owner,
                filters: vec![],
            },
            amount: ValueDef::Constant(1),
        }];
        let ability = AbilityDef {
            trigger: TriggerDef::OnPlay,
            condition: ConditionDef::Always,
            speed: Default::default(),
            timing: Default::default(),
            effects: vec![EffectDef::If {
                condition: ConditionDef::Never,
                then: then_branch.clone(),
                otherwise: otherwise_branch.clone(),
            }],
        };
        let cursor = AbilityCursor {
            stack: ability.effects.clone(),
            context: AbilityContext::default(),
        };
        assert_eq!(cursor.stack.len(), 1);
        assert!(matches!(cursor.stack[0], EffectDef::If { .. }));
    }

    #[test]
    fn pattern_plus_offsets() {
        let offsets: Vec<I16Vec2> = (&PatternDef::Plus(1)).into();
        assert_eq!(offsets.len(), 4);
        assert!(offsets.contains(&I16Vec2::new(1, 0)));
        assert!(offsets.contains(&I16Vec2::new(-1, 0)));
    }

    #[test]
    fn deal_damage_applier_reduces_current_defense() {
        use crate::{
            GameRng,
            actions::ActionPlugin,
            card::{CreatureCard, CurrentDefense},
            components::{Health, Owner},
            turn_controller::TurnControllerPlugin,
        };
        use bevy::input::{ButtonInput, keyboard::KeyCode};
        use bevy::state::app::StatesPlugin;

        let mut app = App::new();
        app.add_plugins((StatesPlugin, ActionPlugin, TurnControllerPlugin))
            .init_resource::<GameRng>()
            .insert_resource(ButtonInput::<KeyCode>::default());

        let creature = app
            .world_mut()
            .spawn((
                CreatureCard,
                CurrentDefense(5),
                Health(5),
                Owner(Entity::PLACEHOLDER),
            ))
            .id();

        app.update();

        app.world_mut().trigger(DealDamage::new(3, creature));
        app.update();

        let defense = app.world().get::<CurrentDefense>(creature).unwrap();
        assert_eq!(defense.0, 2);
    }

    /// The war_golem If branch should pick DealDamage when at least three
    /// friendly creatures (including the caster) are on the board.
    #[test]
    fn war_golem_if_picks_deal_damage_with_three_friendly_creatures() {
        use crate::{
            GameRng,
            actions::{ActionPlugin, targeting::filters::FilterParams},
            board::tile::{Occupant, Position, Tile},
            card::{
                CreatureCard, CurrentAttack, CurrentDefense, CurrentMovementPoints, OnBoard,
            },
            components::{Health, Owner},
            turn_controller::TurnControllerPlugin,
        };
        use bevy::input::{ButtonInput, keyboard::KeyCode};
        use bevy::state::app::StatesPlugin;

        let mut app = App::new();
        app.add_plugins((StatesPlugin, ActionPlugin, TurnControllerPlugin))
            .init_resource::<GameRng>()
            .insert_resource(ButtonInput::<KeyCode>::default());

        let player = app.world_mut().spawn_empty().id();
        let spawn_creature = |world: &mut bevy::prelude::World, owner| {
            let creature = world
                .spawn((
                    CreatureCard,
                    CurrentAttack(1),
                    CurrentDefense(1),
                    CurrentMovementPoints(1),
                    Health(1),
                    Owner(owner),
                ))
                .id();
            let tile = world
                .spawn((Tile, Position(bevy::math::U16Vec2::new(0, 0))))
                .id();
            world.entity_mut(creature).insert(OnBoard { position: tile });
            creature
        };

        let caster = spawn_creature(app.world_mut(), player);
        spawn_creature(app.world_mut(), player);
        spawn_creature(app.world_mut(), player);
        let enemy_player = app.world_mut().spawn_empty().id();
        spawn_creature(app.world_mut(), enemy_player);

        app.update();

        app.add_systems(
            bevy::app::Update,
            move |mut commands: Commands,
                  filter_params: FilterParams,
                  occupied: Query<&Occupant>,
                  creatures: Query<&OnBoard, With<CreatureCard>>,
                  mut rng: ResMut<GameRng>| {
                let ability_entity = commands.spawn_empty().id();
                let if_effect = EffectDef::If {
                    condition: ConditionDef::Compare {
                        left: ValueDef::Count(Box::new(SelectorDef {
                            kind: SelectorKindDef::Creature,
                            cardinality: CardinalityDef::Multi,
                            selection: SelectionDef::AllFriendly,
                            filters: vec![],
                        })),
                        op: CompareOp::GreaterOrEqual,
                        right: ValueDef::Constant(3),
                    },
                    then: vec![EffectDef::DealDamage {
                        selector: SelectorDef {
                            kind: SelectorKindDef::Creature,
                            cardinality: CardinalityDef::Multi,
                            selection: SelectionDef::AllEnemy,
                            filters: vec![],
                        },
                        amount: ValueDef::Expr("2".into()),
                    }],
                    otherwise: vec![EffectDef::DrawCards {
                        player: SelectorDef {
                            kind: SelectorKindDef::Player,
                            cardinality: CardinalityDef::Single,
                            selection: SelectionDef::TurnPlayer,
                            filters: vec![],
                        },
                        amount: ValueDef::Constant(1),
                    }],
                };
                let mut cursor = AbilityCursor {
                    stack: vec![if_effect],
                    context: AbilityContext::default(),
                };
                let effect = cursor.stack.remove(0);
                let mut value_params = filter_params.as_value_params(&mut *rng);
                let result = super::execute_effect(
                    &mut commands,
                    ability_entity,
                    &mut cursor,
                    caster,
                    &effect,
                    &mut value_params,
                    &occupied,
                    &creatures,
                );
                assert!(result.is_ok(), "execute_effect failed: {:?}", result);
                assert_eq!(cursor.stack.len(), 1, "then branch should be spliced");
                assert!(
                    matches!(cursor.stack[0], EffectDef::DealDamage { .. }),
                    "expected DealDamage branch, got {:?}",
                    cursor.stack[0]
                );
            },
        );
        app.update();
    }
}
