use bevy::{
    app::{FixedUpdate, Plugin},
    ecs::{
        component::Component,
        entity::Entity,
        hierarchy::{ChildOf, Children},
        query::{QueryData, With},
        resource::Resource,
        schedule::{IntoScheduleConfigs, common_conditions::any_with_component},
        system::{Commands, Query},
    },
    platform::collections::HashMap,
};

use crate::{
    GameRng,
    actions::{
        NeedsFiltering, NeedsTargeting,
        targeting::{
            AnyTargetSelector,
            filters::FilterParams,
        },
        value_source::ValueSource,
    },
    board::tile::Occupant,
    board::tile::Position,
    card::{
        Cost, CreatureCard, CurrentAttack, CurrentDefense, CurrentMovementPoints, InHand, OnBoard,
        SpellCard, TrapCard,
    },
    components::{Caster, Health, Owner},
    player::{PlayerResources, TurnPlayer},
};

#[derive(Resource, Default)]
pub struct CandidateStore {
    /// selector_entity -> suitable target entities
    pub by_selector: HashMap<Entity, Vec<Entity>>,
}

pub struct TargetPlugin;

impl Plugin for TargetPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.init_resource::<CandidateStore>().add_systems(
            FixedUpdate,
            (
                (apply_targeting,).run_if(any_with_component::<NeedsTargeting>),
                (apply_filter).run_if(any_with_component::<NeedsFiltering>),
                (finalize_targeting).run_if(any_with_component::<NeedsFinalization>),
            ),
        );
    }
}

#[derive(QueryData)]
pub struct CreatureQuery {
    pub health: &'static Health,
    pub current_atttack: &'static CurrentAttack,
    pub current_defense: &'static CurrentDefense,
    pub movement_points: &'static CurrentMovementPoints,
    pub entity: Entity,
    pub owner: &'static Owner,
    pub position: &'static OnBoard,
}

#[derive(Debug, QueryData)]
pub struct TileQuery {
    pub entity: Entity,
    pub children: &'static Children,
    pub position: &'static Position,
    pub occupant: Option<&'static Occupant>,
}

#[derive(Debug, QueryData)]
pub struct PlayerQuery {
    pub entity: Entity,
    pub turn_player: Option<&'static TurnPlayer>,
    pub resources: &'static PlayerResources,
}

#[derive(Debug, QueryData)]
pub struct HandQuery {
    pub entity: Entity,
    pub creature: Option<&'static CreatureCard>,
    pub spell: Option<&'static SpellCard>,
    pub trap: Option<&'static TrapCard>,
    pub in_hand: &'static InHand,
    pub cost: Option<&'static Cost>,
}

#[derive(QueryData)]
struct TargetSelectorQuery {
    pub entity: Entity,
    pub caster: &'static Caster,
    pub selector: &'static AnyTargetSelector,
}

fn apply_targeting(
    q_selectors: Query<TargetSelectorQuery, With<NeedsTargeting>>,
    query: FilterParams,
    mut rng: bevy::ecs::system::ResMut<GameRng>,
    mut candidates: bevy::ecs::system::ResMut<CandidateStore>,
    mut commands: Commands,
) {
    let mut value_params = query.as_value_params(&mut *rng);
    for TargetSelectorQueryItem {
        entity: e_selector,
        caster: &Caster(caster),
        selector,
        ..
    } in &q_selectors
    {
        let targets: Vec<Entity> = selector.selection().find_suitable(&mut value_params, caster);

        // store targets
        candidates.by_selector.insert(e_selector, targets);

        commands
            .entity(e_selector)
            .remove::<NeedsTargeting>()
            .insert(NeedsFiltering);
    }
}

#[derive(Component, Clone, Copy, PartialEq)]
pub struct NeedsFinalization;

fn apply_filter(
    q_selectors: Query<TargetSelectorQuery, With<NeedsFiltering>>,
    params: FilterParams,
    mut rng: bevy::ecs::system::ResMut<GameRng>,
    mut candidates: bevy::ecs::system::ResMut<CandidateStore>,
    mut commands: Commands,
) {
    let mut value_params = params.as_value_params(&mut *rng);
    for q_selector in &q_selectors {
        let caster = q_selector.caster.0;

        if let Some(list) = candidates.by_selector.get_mut(&q_selector.entity) {
            list.retain(|&target| {
                q_selector
                    .selector
                    .validation()
                    .validate(&mut value_params, caster, target)
            });
        } else {
            // optional: ensure key exists
            candidates.by_selector.insert(q_selector.entity, Vec::new());
        }

        commands
            .entity(q_selector.entity)
            .remove::<NeedsFiltering>()
            .insert(NeedsFinalization);
    }
}

fn finalize_targeting(
    q_selectors: Query<TargetSelectorQuery, With<NeedsFinalization>>,
    _q_value_sources: Query<(&ValueSource, &ChildOf)>,
    mut candidates: bevy::ecs::system::ResMut<CandidateStore>,
    mut commands: Commands,
) {
    for q_selector in &q_selectors {
        let list: &[Entity] = candidates
            .by_selector
            .get(&q_selector.entity)
            .map(|v| v.as_slice())
            .unwrap_or(&[]);

        let _finalization_effect = q_selector.selector.selection().finalize(list);

        // optional: cleanup the candidate list once finalized
        // (depends on how your next pipeline step works)
        candidates.by_selector.remove(&q_selector.entity);

        commands
            .entity(q_selector.entity)
            .remove::<NeedsFinalization>();
    }
}
