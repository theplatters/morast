use bevy::{
    app::{FixedUpdate, Plugin},
    ecs::{
        component::Component,
        entity::Entity,
        hierarchy::{ChildOf, Children},
        query::{QueryData, With},
        schedule::{IntoScheduleConfigs, common_conditions::any_with_component},
        system::{Commands, Query},
    },
};

use crate::{
    actions::{
        targeting::{
            Constraint, CreatureTarget, HandTarget, IsTargetSelectMode, MultiTarget, PlayerTarget,
            SingleTarget, TargetKind, TargetSelector, TileTarget,
            filters::{FilterParams, IsFilter},
        },
        {NeedsFiltering, NeedsTargeting},
    },
    board::tile::{Position, Tile},
    card::{CreatureCard, CurrentAttack, CurrentDefense, InHand, OnBoard, SpellCard, TrapCard},
    components::{Health, Owner},
    player::{Player, TurnPlayer},
};

pub struct TargetPlugin;

impl Plugin for TargetPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(
            FixedUpdate,
            (
                (
                    apply_targeting::<CreatureTarget, SingleTarget>,
                    apply_targeting::<CreatureTarget, MultiTarget>,
                    apply_targeting::<TileTarget, SingleTarget>,
                    apply_targeting::<TileTarget, MultiTarget>,
                    apply_targeting::<HandTarget, SingleTarget>,
                    apply_targeting::<HandTarget, MultiTarget>,
                    apply_targeting::<PlayerTarget, SingleTarget>,
                    apply_targeting::<PlayerTarget, MultiTarget>,
                )
                    .run_if(any_with_component::<NeedsTargeting>),
                (
                    apply_filter::<CreatureTarget, SingleTarget>,
                    apply_filter::<CreatureTarget, MultiTarget>,
                    apply_filter::<TileTarget, SingleTarget>,
                    apply_filter::<TileTarget, MultiTarget>,
                    apply_filter::<HandTarget, SingleTarget>,
                    apply_filter::<HandTarget, MultiTarget>,
                    apply_filter::<PlayerTarget, SingleTarget>,
                    apply_filter::<PlayerTarget, MultiTarget>,
                )
                    .run_if(any_with_component::<NeedsFiltering>),
            ),
        );
    }
}

#[derive(Component)]
pub struct Candidate {
    pub for_selector: Entity,
    pub target: Entity,
}

#[derive(QueryData)]
pub struct CreatureQuery {
    pub health: &'static Health,
    pub current_atttack: &'static CurrentAttack,
    pub current_defense: &'static CurrentDefense,
    pub entity: Entity,
    pub owner: &'static Owner,
    pub position: &'static OnBoard,
}

#[derive(Debug, QueryData)]
pub struct TileQuery {
    pub entity: Entity,
    pub children: &'static Children,
    pub position: &'static Position,
}

#[derive(Debug, QueryData)]
pub struct PlayerQuery {
    pub entity: Entity,
    pub turn_player: Option<&'static TurnPlayer>,
}

#[derive(Debug, QueryData)]
pub struct HandQuery {
    pub entity: Entity,
    pub creature: Option<&'static CreatureCard>,
    pub spell: Option<&'static SpellCard>,
    pub trap: Option<&'static TrapCard>,
    pub in_hand: &'static InHand,
}

#[derive(QueryData)]
struct TargetSelectorQuery<K: TargetKind<C>, C: Constraint> {
    pub entity: Entity,
    pub child_of: &'static ChildOf,
    pub selector: &'static TargetSelector<K, C>,
}

fn apply_targeting<TTarget, TCardinality>(
    q_selectors: Query<TargetSelectorQuery<TTarget, TCardinality>, With<NeedsTargeting>>,
    query: FilterParams,
    mut commands: Commands,
) where
    TTarget: TargetKind<TCardinality>,
    TCardinality: Constraint,
{
    for TargetSelectorQueryItem::<TTarget, TCardinality> {
        entity: e_selector,
        child_of: &ChildOf(caster),
        selector,
    } in &q_selectors
    {
        let targets = match &selector.selection {
            super::SelectionMethod::Auto(auto_sel) => auto_sel.mode.find_suitable(&query, caster),
            super::SelectionMethod::Manual(manual_sel) => {
                manual_sel.mode.find_suitable(&query, caster)
            }
        };

        commands.spawn_batch(targets.into_iter().map(move |e| Candidate {
            for_selector: e_selector,
            target: e,
        }));

        commands
            .entity(e_selector)
            .remove::<NeedsTargeting>()
            .insert(NeedsFiltering);
    }
}

#[derive(Component, Clone, Copy, PartialEq)]
pub struct NeedsFinalization;

fn apply_filter<TTarget, TCardinality>(
    q_selectors: Query<TargetSelectorQuery<TTarget, TCardinality>, With<NeedsFiltering>>,
    q_suitable_targets: Query<(Entity, &Candidate)>,
    params: FilterParams, // or take as argument in the outer system and capture
    mut commands: Commands,
) where
    TTarget: TargetKind<TCardinality>,
    TCardinality: Constraint,
{
    for q_selector in q_selectors {
        for (
            e_candidate,
            Candidate {
                for_selector,
                target,
            },
        ) in q_suitable_targets
        {
            if *for_selector == q_selector.entity
                && q_selector
                    .selector
                    .validation
                    .validate(&params, q_selector.child_of.0, *target)
            {
                commands.entity(e_candidate).despawn();
            }
            commands
                .entity(q_selector.entity)
                .remove::<NeedsFiltering>()
                .insert(NeedsFinalization);
        }
    }
}
