//! Observers that apply the atomic entity events emitted by the ability executor.

use bevy::{math::U16Vec2, prelude::*};

use crate::{
    board::{
        effect::Effect,
        tile::{Occupant, Position},
        BoardRes, EffectRequested,
    },
    card::{
        creature::{BaseAttack, BaseDefense, BaseMovementPoints},
        CreatureCard, CurrentAttack, CurrentDefense, CurrentMovementPoints, InDeck, InGraveyard,
        InHand, OnBoard,
    },
    components::{Health, Owner},
    events::CardsDrawn,
    player::{Deck, Hand, Player, PlayerResources},
};

use super::{
    AddGold, ApplyEffect, DealDamage, DestroyCreature, DiscardCards, DrawCards, HealCreature,
    Mill, ModifyStats, MoveCreature, ReturnToHand,
};

pub fn apply_deal_damage(
    trigger: On<DealDamage>,
    mut creatures: Query<(&mut CurrentDefense, &Health, &Owner, Entity), With<CreatureCard>>,
    mut commands: Commands,
) {
    let event = trigger.event();
    let target = trigger.event_target();
    let Ok((mut defense, _health, owner, entity)) = creatures.get_mut(target) else {
        return;
    };

    let damage = event.amount;
    defense.0 = defense.0.saturating_sub(damage);

    if defense.0 == 0 {
        destroy_creature(entity, owner.0, &mut commands);
    }
}

pub fn apply_heal(
    trigger: On<HealCreature>,
    mut creatures: Query<(&mut CurrentDefense, &Health), With<CreatureCard>>,
) {
    let event = trigger.event();
    let target = trigger.event_target();
    let Ok((mut defense, health)) = creatures.get_mut(target) else {
        return;
    };

    // Cap healing at the creature's max health (Health component).
    let max = health.value();
    defense.0 = (defense.0 + event.amount).min(max);
}

pub fn apply_add_gold(trigger: On<AddGold>, mut players: Query<&mut PlayerResources>) {
    let event = trigger.event();
    let target = trigger.event_target();
    let Ok(mut resources) = players.get_mut(target) else {
        return;
    };
    resources.gold = resources.gold.saturating_add(event.amount);
}

pub fn apply_draw_cards(
    trigger: On<DrawCards>,
    mut decks: Query<&mut Deck>,
    mut hands: Query<&mut Hand>,
    mut commands: Commands,
    mut drawn: MessageWriter<CardsDrawn>,
) {
    let event = trigger.event();
    let target = trigger.event_target();
    let amount = event.amount as usize;

    let Ok(mut deck) = decks.get_mut(target) else {
        return;
    };
    let Ok(mut hand) = hands.get_mut(target) else {
        return;
    };

    let to_draw: Vec<Entity> = deck.iter().take(amount).collect();
    for card_entity in to_draw {
        commands
            .entity(card_entity)
            .remove::<InDeck>()
            .insert(InHand { parent: target });
        drawn.write(CardsDrawn { card: card_entity });
    }

    // Force the relationship targets to refresh their cached vectors.
    deck.set_changed();
    hand.set_changed();
}

pub fn apply_apply_effect(
    trigger: On<ApplyEffect>,
    tiles: Query<&Position>,
    mut effect_requests: MessageWriter<EffectRequested>,
) {
    let event = trigger.event();
    let target = trigger.event_target();

    let Ok(&Position(pos)) = tiles.get(target) else {
        return;
    };

    // The executor already resolved the selected tile as the event target.
    // We request the board system to place the effect on that tile.
    effect_requests.write(EffectRequested {
        effect: Effect::new(event.effect, event.duration, Player { number: 0 }),
        indices: vec![pos],
    });
}

pub fn apply_modify_stats(
    trigger: On<ModifyStats>,
    mut creatures: Query<
        (
            &mut CurrentAttack,
            &mut CurrentDefense,
            &Health,
            &BaseAttack,
            &BaseDefense,
            &BaseMovementPoints,
            &mut CurrentMovementPoints,
        ),
        With<CreatureCard>,
    >,
) {
    let event = trigger.event();
    let target = trigger.event_target();
    let Ok((mut attack, mut defense, health, _base_attack, _base_defense, _base_speed, mut speed)) =
        creatures.get_mut(target)
    else {
        return;
    };

    // Apply the modifier. MaxHealth changes are applied to a local copy only;
    // Health is treated as the immutable max-HP source for this fix pass.
    let mut max_health = health.value();
    event.stat_modifier.apply(
        &mut attack.0,
        &mut defense.0,
        &mut max_health,
        &mut speed.0,
    );

    // Allow temporary buffs to exceed base stats, but keep current HP at or below max HP.
    defense.0 = defense.0.min(health.value());
}

pub fn apply_move_creature(
    trigger: On<MoveCreature>,
    creatures: Query<&OnBoard, With<CreatureCard>>,
    board: Res<BoardRes>,
    occupied: Query<&Occupant>,
    tiles: Query<&Position>,
    mut commands: Commands,
) {
    let event = trigger.event();
    let target = trigger.event_target();
    let Ok(on_board) = creatures.get(target) else {
        return;
    };

    // Find the creature's current tile via the OnBoard relationship.
    let Ok(&Position(current_pos)) = tiles.get(on_board.position) else {
        return;
    };

    let next_pos = if event.absolute {
        U16Vec2::new(event.direction.x as u16, event.direction.y as u16)
    } else {
        let dx = event.direction.x;
        let dy = event.direction.y;
        let new_x = current_pos.x.checked_add_signed(dx).unwrap_or(current_pos.x);
        let new_y = current_pos.y.checked_add_signed(dy).unwrap_or(current_pos.y);
        U16Vec2::new(new_x, new_y)
    };

    let Some(new_tile) = board.get_tile(&next_pos) else {
        return;
    };

    if occupied.contains(new_tile) {
        warn!(
            "Cannot move creature {} to occupied tile {}",
            target, new_tile
        );
        return;
    }

    commands.entity(target).insert(OnBoard { position: new_tile });
}

pub fn apply_destroy_creature(
    trigger: On<DestroyCreature>,
    mut commands: Commands,
    owners: Query<&Owner>,
) {
    let target = trigger.event_target();
    let owner = owners.get(target).map(|o| o.0).unwrap_or(Entity::PLACEHOLDER);
    destroy_creature(target, owner, &mut commands);
}

pub fn apply_discard_cards(
    trigger: On<DiscardCards>,
    mut commands: Commands,
    cards: Query<(Entity, &InHand, &Owner)>,
) {
    let event = trigger.event();
    let target = trigger.event_target();
    let amount = event.amount as usize;

    let mut discarded = 0;
    for (card_entity, in_hand, owner) in &cards {
        if discarded >= amount {
            break;
        }
        if owner.0 != target || in_hand.parent != target {
            continue;
        }
        commands
            .entity(card_entity)
            .remove::<InHand>()
            .insert(InGraveyard { owner: target });
        discarded += 1;
    }
}

pub fn apply_mill(
    trigger: On<Mill>,
    mut commands: Commands,
    cards: Query<(Entity, &InDeck, &Owner)>,
) {
    let event = trigger.event();
    let target = trigger.event_target();
    let amount = event.amount as usize;

    let mut milled = 0;
    for (card_entity, in_deck, owner) in &cards {
        if milled >= amount {
            break;
        }
        if owner.0 != target || in_deck.parent != target {
            continue;
        }
        commands
            .entity(card_entity)
            .remove::<InDeck>()
            .insert(InGraveyard { owner: target });
        milled += 1;
    }
}

pub fn apply_return_to_hand(
    trigger: On<ReturnToHand>,
    mut commands: Commands,
    owners: Query<&Owner>,
) {
    let target = trigger.event_target();
    let owner = owners.get(target).map(|o| o.0).unwrap_or(Entity::PLACEHOLDER);
    commands
        .entity(target)
        .remove::<OnBoard>()
        .remove::<InGraveyard>()
        .remove::<InDeck>()
        .insert(InHand { parent: owner });
}

fn destroy_creature(entity: Entity, owner: Entity, commands: &mut Commands) {
    commands
        .entity(entity)
        .remove::<OnBoard>()
        .remove::<InHand>()
        .remove::<InDeck>()
        .insert(InGraveyard { owner });
}
