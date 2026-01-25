use bevy::ecs::{entity::Entity, query::With, system::Query};

use crate::{
    actions::{
        IsWaiter, Requirement,
        targeting::{
            CreatureTarget, HandTarget, PlayerTarget, TargetFilter, TileTarget,
            systems::{CreatureQuery, HandQuery, PlayerQuery, TileQuery},
        },
        value_source::ValueSource,
    },
    board::tile::Tile,
};

pub trait IsFilter {
    fn validate(&self, context: &FilterParams, caster: Entity, candidate: Entity) -> bool;
}

// generic composition type
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RulesWithExtras<Base: IsFilter, Extra: IsFilter> {
    pub base: Base,
    pub extras: Vec<Extra>,
}

impl<B: IsFilter, E: IsFilter> IsFilter for RulesWithExtras<B, E> {
    fn validate(&self, context: &FilterParams, caster: Entity, candidate: Entity) -> bool {
        self.base.validate(context, caster, candidate)
            && self.extras.validate(context, caster, candidate)
    }
}

impl<Base: IsFilter, Extra: IsFilter> RulesWithExtras<Base, Extra> {
    pub fn from_base(base: Base) -> Self {
        Self {
            base,
            extras: Vec::new(),
        }
    }
}

impl<Base: Default + IsFilter, Extra: IsFilter> Default for RulesWithExtras<Base, Extra> {
    fn default() -> Self {
        Self {
            base: Base::default(),
            extras: Vec::new(),
        }
    }
}

impl<T: IsFilter> IsFilter for Vec<T> {
    fn validate(&self, context: &FilterParams, caster: Entity, candidate: Entity) -> bool {
        self.iter().all(|l| l.validate(context, caster, candidate))
    }
}
// Filter structs with defaults
#[derive(Debug, Clone, Default)]
pub struct CreatureFilters {
    pub min_health: Option<ValueSource>,
    pub max_health: Option<ValueSource>,
    pub health_percent: Option<(ValueSource, ValueSource)>,
    pub damaged_only: bool,
    pub min_attack: Option<ValueSource>,
    pub can_attack: Option<bool>,
}

impl IsFilter for CreatureFilters {
    fn validate(&self, context: &FilterParams, caster: Entity, candidate: Entity) -> bool {
        todo!()
    }
}

#[derive(Debug, Clone)]
pub enum CreatureExtraRules {}
impl IsFilter for CreatureExtraRules {
    fn validate(&self, context: &FilterParams, caster: Entity, candidate: Entity) -> bool {
        true
    }
}

#[derive(Debug, Clone, Default)]
pub struct TileFilters {
    pub empty_only: bool,
    pub occupied_only: bool,
    pub in_range_of_caster: Option<ValueSource>,
}

impl IsFilter for TileFilters {
    fn validate(&self, context: &FilterParams, caster: Entity, candidate: Entity) -> bool {
        todo!()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TileExtraRules {}

impl IsFilter for TileExtraRules {
    fn validate(&self, context: &FilterParams, caster: Entity, candidate: Entity) -> bool {
        true
    }
}

#[derive(Debug, Clone, Default)]
pub struct PlayerFilters {
    pub min_gold: Option<ValueSource>,
    pub max_gold: Option<ValueSource>,
    pub has_cards_in_hand: Option<ValueSource>,
    pub min_health: Option<ValueSource>,
    pub max_health: Option<ValueSource>,
}

impl IsFilter for PlayerFilters {
    fn validate(&self, context: &FilterParams, caster: Entity, candidate: Entity) -> bool {
        todo!()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlayerExtraRules {
    TookDamageLastRound,
    PlayedCardThisTurn,
}

impl IsFilter for PlayerExtraRules {
    fn validate(&self, context: &FilterParams, caster: Entity, candidate: Entity) -> bool {
        todo!()
    }
}

#[derive(Clone, Debug)]
pub struct HandFilters {
    pub min_cost: Option<ValueSource>,
    pub max_cost: Option<ValueSource>,
}

impl IsFilter for HandFilters {
    fn validate(&self, context: &FilterParams, caster: Entity, candidate: Entity) -> bool {
        todo!()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum HandExtraRules {
    ExludeCreatures,
    ExcludeSpells,
    ExcludeTraps,
}

impl IsFilter for HandExtraRules {
    fn validate(&self, context: &FilterParams, caster: Entity, candidate: Entity) -> bool {
        todo!()
    }
}

impl TargetFilter for CreatureTarget {
    type FilterBase = CreatureFilters;
    type FilterExtra = CreatureExtraRules;
    type Filter = RulesWithExtras<Self::FilterBase, Self::FilterExtra>;
}

impl TargetFilter for TileTarget {
    type FilterBase = TileFilters;
    type FilterExtra = TileExtraRules;
    type Filter = RulesWithExtras<Self::FilterBase, Self::FilterExtra>;
}
impl TargetFilter for PlayerTarget {
    type FilterBase = PlayerFilters;
    type FilterExtra = PlayerExtraRules;
    type Filter = RulesWithExtras<Self::FilterBase, Self::FilterExtra>;
}

impl TargetFilter for HandTarget {
    type FilterBase = HandFilters;
    type FilterExtra = HandExtraRules;
    type Filter = RulesWithExtras<Self::FilterBase, Self::FilterExtra>;
}

#[derive(bevy::ecs::system::SystemParam)]
pub struct FilterParams<'w, 's> {
    pub creatures: Query<'w, 's, CreatureQuery>,
    pub tiles: Query<'w, 's, TileQuery, With<Tile>>,
    pub hand: Query<'w, 's, HandQuery>,
    pub player: Query<'w, 's, PlayerQuery>,
}
impl IsWaiter for HandExtraRules {
    fn emit_requirements(&self, _f: &mut dyn FnMut(Requirement)) {}
}

impl<B, E> IsWaiter for RulesWithExtras<B, E>
where
    B: IsWaiter + IsFilter,
    E: IsWaiter + IsFilter,
{
    fn emit_requirements(&self, f: &mut dyn FnMut(Requirement)) {
        self.base.emit_requirements(f);
        for extra in &self.extras {
            extra.emit_requirements(f);
        }
    }
}

impl IsWaiter for HandFilters {
    fn emit_requirements(&self, f: &mut dyn FnMut(Requirement)) {
        if let Some(v) = &self.min_cost {
            f(Requirement::value(v.clone()));
        }
        if let Some(v) = &self.max_cost {
            f(Requirement::value(v.clone()));
        }
    }
}

impl IsWaiter for CreatureFilters {
    fn emit_requirements(&self, f: &mut dyn FnMut(Requirement)) {
        if let Some(v) = &self.min_health {
            f(Requirement::value(v.clone()));
        }
        if let Some(v) = &self.max_health {
            f(Requirement::value(v.clone()));
        }
        if let Some((a, b)) = &self.health_percent {
            f(Requirement::value(a.clone()));
            f(Requirement::value(b.clone()));
        }
        if let Some(v) = &self.min_attack {
            f(Requirement::value(v.clone()));
        }
    }
}

impl IsWaiter for PlayerExtraRules {
    fn emit_requirements(&self, _: &mut dyn FnMut(Requirement)) {
        // no requirements
    }
}

impl IsWaiter for PlayerFilters {
    fn emit_requirements(&self, f: &mut dyn FnMut(Requirement)) {
        if let Some(v) = &self.min_gold {
            f(Requirement::value(v.clone()));
        }
        if let Some(v) = &self.max_gold {
            f(Requirement::value(v.clone()));
        }
        if let Some(v) = &self.has_cards_in_hand {
            f(Requirement::value(v.clone()));
        }
        if let Some(v) = &self.min_health {
            f(Requirement::value(v.clone()));
        }
        if let Some(v) = &self.max_health {
            f(Requirement::value(v.clone()));
        }
    }
}

impl IsWaiter for TileExtraRules {
    fn emit_requirements(&self, _: &mut dyn FnMut(Requirement)) {
        // no requirements
    }
}

impl IsWaiter for CreatureExtraRules {
    fn emit_requirements(&self, _: &mut dyn FnMut(Requirement)) {
        // no requirements
    }
}

impl IsWaiter for TileFilters {
    fn emit_requirements(&self, f: &mut dyn FnMut(Requirement)) {
        if let Some(v) = &self.in_range_of_caster {
            f(Requirement::value(v.clone()));
        }
    }
}
