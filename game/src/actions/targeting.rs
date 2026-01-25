use bevy::{
    ecs::{component::Component, entity::Entity, query::With, system::Query},
    log::warn,
    reflect::Is,
};
use janet_bindings::types::janetabstract::IsAbstract;
use rand::seq::SliceRandom;

use crate::{
    actions::{
        IsWaiter, Requirement,
        targeting::{
            filters::{FilterParams, IsFilter},
            systems::{CreatureQuery, NeedsFinalization, TileQuery},
        },
        value_source::{ValueEvalParams, ValueSource},
    },
    board::tile::Tile,
};

mod filters;
pub mod systems;
pub mod target_builder;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SingleTarget;
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MultiTarget;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CreatureTarget;
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TileTarget;
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PlayerTarget;
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HandTarget;

#[derive(Clone, Debug)]
pub struct Or<A, B>(std::marker::PhantomData<(A, B)>);

pub trait Constraint: 'static + Send + Sync + Clone + std::fmt::Debug {}
impl<
    A: std::fmt::Debug + Send + Sync + 'static + Clone,
    B: std::fmt::Debug + Send + Sync + 'static + Clone,
> Constraint for Or<A, B>
{
}
impl Constraint for SingleTarget {}
impl Constraint for MultiTarget {}

pub trait TargetFilter {
    type FilterBase: Clone + std::fmt::Debug + Send + Sync + 'static;
    type FilterExtra: Clone + std::fmt::Debug + Send + Sync + 'static;
    type Filter: Clone + std::fmt::Debug + Send + Sync + 'static + IsFilter;
}
pub trait TargetKind<C: Constraint>:
    'static + Send + Sync + Clone + std::fmt::Debug + TargetFilter
{
    type Auto: IsTargetSelectMode + Clone;
    type Manual: IsTargetSelectMode + Clone;
}

#[derive(Component, Clone, Debug)]
pub struct TargetSelector<K, C>
where
    K: TargetKind<C>,
    C: Constraint,
{
    pub(crate) selection: SelectionMethod<K, C>,
    pub(crate) validation: K::Filter,
    _kind: std::marker::PhantomData<(K, C)>,
}

impl<K, C> TargetSelector<K, C>
where
    K: TargetKind<C>,
    C: Constraint,
{
    pub fn new(selection: SelectionMethod<K, C>, validation: K::Filter) -> Self {
        Self {
            selection,
            validation,
            _kind: std::marker::PhantomData,
        }
    }
}

// Internal enums stay the same but are now private construction
#[derive(Debug, Clone)]
pub enum SelectionMethod<K: TargetKind<C>, C: Constraint> {
    Auto(AutoSelector<K, C>),
    Manual(ManualSelector<K, C>),
}

impl<K: TargetKind<C>, C: Constraint> IsTargetSelectMode for SelectionMethod<K, C> {
    fn find_suitable(&self, query: &FilterParams, caster: Entity) -> Vec<Entity> {
        match self {
            SelectionMethod::Auto(auto_selector) => auto_selector.find_suitable(query, caster),
            SelectionMethod::Manual(manual_selector) => {
                manual_selector.find_suitable(query, caster)
            }
        }
    }
    fn finalize(&self, candidates: &[Entity]) -> FinalizeEffect {
        match self {
            SelectionMethod::Auto(auto_selector) => auto_selector.finalize(candidates),
            SelectionMethod::Manual(manual_selector) => manual_selector.finalize(candidates),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AutoSelector<K: TargetKind<C>, C: Constraint> {
    pub mode: K::Auto,
    _k: std::marker::PhantomData<(K, C)>,
}

impl<K: TargetKind<C>, C: Constraint> IsTargetSelectMode for AutoSelector<K, C> {
    fn find_suitable(&self, query: &FilterParams, caster: Entity) -> Vec<Entity> {
        self.mode.find_suitable(query, caster)
    }
    fn finalize(&self, candidates: &[Entity]) -> FinalizeEffect {
        self.mode.finalize(candidates)
    }
}

impl<K: TargetKind<C>, C: Constraint> AutoSelector<K, C> {
    pub fn new(mode: K::Auto) -> Self {
        Self {
            mode,
            _k: std::marker::PhantomData,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ManualSelector<K: TargetKind<C>, C: Constraint> {
    pub mode: K::Manual,
    _k: std::marker::PhantomData<(K, C)>,
}

impl<K: TargetKind<C>, C: Constraint> IsTargetSelectMode for ManualSelector<K, C> {
    fn find_suitable(&self, query: &FilterParams, caster: Entity) -> Vec<Entity> {
        self.mode.find_suitable(query, caster)
    }

    fn finalize(&self, candidates: &[Entity]) -> FinalizeEffect {
        self.mode.finalize(candidates)
    }
}

impl<K: TargetKind<C>, C: Constraint> ManualSelector<K, C> {
    pub fn new(mode: K::Manual) -> Self {
        Self {
            mode,
            _k: std::marker::PhantomData,
        }
    }
}

pub enum AutoMultiTile {
    AllTiles,
    RadiusAroundCaster { radius: u8 },
}

#[derive(Clone, Debug)]
pub enum AutoMultiCreature {
    AllEnemy,
    AllFriendly,
    Random { count: ValueSource },
}

#[derive(Clone, Debug)]
pub enum ManualCreature {
    Choose { min: ValueSource, max: ValueSource },
    MaxNFriendly { count: ValueSource },
    ExactlyNFriendly { count: ValueSource },
}

#[derive(Clone, Debug)]
pub enum AutoSingleCreature {
    Strongest,
    Caster,
}

#[derive(Clone, Debug)]
pub enum ManualTile {
    ChooseTiles { amount: ValueSource },
    ChooseArea { radius: ValueSource },
}

#[derive(Clone, Debug)]
pub enum AutoPlayerSingle {
    TurnPlayer,
    NonTurnPlayer,
}

#[derive(Clone, Debug)]
pub struct AutoPlayerMulti;

#[derive(Clone, Debug)]
pub struct ManualPlayer;

#[derive(Clone, Debug)]
pub enum AutoHand {
    AllCards,
    AllCreatures,
    AllSpells,
    AllTraps,
}

#[derive(Clone, Debug)]
pub struct ManualHand {
    count: ValueSource,
}

impl TargetKind<SingleTarget> for CreatureTarget {
    type Auto = AutoSingleCreature;
    type Manual = ManualCreature;
}

impl TargetKind<MultiTarget> for CreatureTarget {
    type Auto = AutoMultiCreature;
    type Manual = ManualCreature;
}

impl TargetKind<SingleTarget> for TileTarget {
    type Auto = ();
    type Manual = ManualTile;
}

impl TargetKind<MultiTarget> for TileTarget {
    type Auto = ();
    type Manual = ManualTile;
}

impl TargetKind<SingleTarget> for PlayerTarget {
    type Auto = AutoPlayerSingle;
    type Manual = ManualPlayer;
}

impl TargetKind<MultiTarget> for PlayerTarget {
    type Auto = AutoPlayerMulti;
    type Manual = ManualPlayer;
}

impl TargetKind<SingleTarget> for HandTarget {
    type Auto = AutoHand;

    type Manual = ManualHand;
}

impl TargetKind<MultiTarget> for HandTarget {
    type Auto = AutoHand;

    type Manual = ManualHand;
}

impl<K, A, B> TargetKind<Or<A, B>> for K
where
    A: Constraint,
    B: Constraint,
    K: TargetKind<A> + TargetKind<B>,
{
    // For Or, the mode types must be able to represent both.
    // Easiest is to wrap them in enums:
    type Auto = Either<<K as TargetKind<A>>::Auto, <K as TargetKind<B>>::Auto>;
    type Manual = Either<<K as TargetKind<A>>::Manual, <K as TargetKind<B>>::Manual>;
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Either<L, R> {
    Left(L),
    Right(R),
}

pub type CreatureSel<C> = TargetSelector<CreatureTarget, C>;

pub type TileSel<C> = TargetSelector<TileTarget, C>;
pub type PlayerSel<C> = TargetSelector<PlayerTarget, C>;
pub type HandSel<C> = TargetSelector<HandTarget, C>;

#[derive(Debug, Clone)]
pub enum SingleTargetSelector {
    Creature(CreatureSel<SingleTarget>),
    Tile(TileSel<SingleTarget>),
    Player(PlayerSel<SingleTarget>),
    Hand(HandSel<SingleTarget>),
}

#[derive(Debug, Clone)]
pub enum MultiTargetSelector {
    Creature(CreatureSel<MultiTarget>),
    Tile(TileSel<MultiTarget>),
    Player(PlayerSel<MultiTarget>),
    Hand(HandSel<MultiTarget>),
}

impl From<MultiTargetSelector> for AnyTargetSelector {
    fn from(value: MultiTargetSelector) -> Self {
        match value {
            MultiTargetSelector::Creature(target_selector) => target_selector.into(),
            MultiTargetSelector::Tile(target_selector) => target_selector.into(),
            MultiTargetSelector::Player(target_selector) => target_selector.into(),
            MultiTargetSelector::Hand(target_selector) => target_selector.into(),
        }
    }
}

impl<K> From<TargetSelector<K, SingleTarget>> for TargetSelector<K, Or<SingleTarget, MultiTarget>>
where
    K: TargetKind<SingleTarget> + TargetKind<MultiTarget>,
{
    fn from(value: TargetSelector<K, SingleTarget>) -> Self {
        TargetSelector {
            selection: match value.selection {
                SelectionMethod::Auto(a) => SelectionMethod::Auto(AutoSelector {
                    mode: Either::Left(a.mode),
                    _k: std::marker::PhantomData,
                }),
                SelectionMethod::Manual(m) => SelectionMethod::Manual(ManualSelector {
                    mode: Either::Left(m.mode),
                    _k: std::marker::PhantomData,
                }),
            },
            // Filter type is shared via TargetFilter on K, so this is the same
            validation: value.validation,
            _kind: std::marker::PhantomData,
        }
    }
}

impl<K> From<TargetSelector<K, MultiTarget>> for TargetSelector<K, Or<SingleTarget, MultiTarget>>
where
    K: TargetKind<SingleTarget> + TargetKind<MultiTarget>,
{
    fn from(value: TargetSelector<K, MultiTarget>) -> Self {
        TargetSelector {
            selection: match value.selection {
                SelectionMethod::Auto(a) => SelectionMethod::Auto(AutoSelector {
                    mode: Either::Right(a.mode),
                    _k: std::marker::PhantomData,
                }),
                SelectionMethod::Manual(m) => SelectionMethod::Manual(ManualSelector {
                    mode: Either::Right(m.mode),
                    _k: std::marker::PhantomData,
                }),
            },
            validation: value.validation,
            _kind: std::marker::PhantomData,
        }
    }
}

impl<K, C> IsAbstract for TargetSelector<K, C>
where
    K: TargetKind<C>,
    C: Constraint,
{
    fn type_info() -> &'static janet_bindings::bindings::JanetAbstractType {
        todo!()
    }
}

fn select_friendly(q_creatures: Query<CreatureQuery>, caster: Entity) -> Vec<Entity> {
    let owner_of_caster = q_creatures.get(caster).unwrap().owner;
    q_creatures
        .iter()
        .filter(|q| q.owner == owner_of_caster)
        .map(|q| q.entity)
        .collect()
}

fn select_enemy(q_creatures: Query<CreatureQuery>, caster: Entity) -> Vec<Entity> {
    let owner_of_caster = q_creatures.get(caster).unwrap().owner;
    q_creatures
        .iter()
        .filter(|q| q.owner != owner_of_caster)
        .map(|q| q.entity)
        .collect()
}

fn select_all_creatures(q_creatures: Query<CreatureQuery>) -> Vec<Entity> {
    q_creatures.iter().map(|c| c.entity).collect()
}

fn select_all_tiles(q_tiles: Query<TileQuery, With<Tile>>) -> Vec<Entity> {
    q_tiles.iter().map(|c| c.entity).collect()
}

pub trait IsTargetSelectMode: std::fmt::Debug + Send + Sync + 'static {
    fn find_suitable(&self, query: &FilterParams, caster: Entity) -> Vec<Entity>;
    fn finalize(&self, candidates: &[Entity]) -> FinalizeEffect;
}

#[derive(Debug)]
pub enum FinalizeEffect {
    None,
    AwaitInput,
    ExecuteSingle(Entity),
    ExecuteAll,
    ExecuteSubset(Vec<Entity>), // for random / limited / reordered selections
    AwaitingValueSource { value_source: ValueSource }, // evaluated later using runtime context
}

impl IsTargetSelectMode for AutoSingleCreature {
    fn find_suitable(&self, query: &FilterParams, caster: Entity) -> Vec<Entity> {
        match self {
            AutoSingleCreature::Strongest => vec![
                query
                    .creatures
                    .iter()
                    .reduce(|acc, curr| {
                        if acc.current_atttack < curr.current_atttack {
                            curr
                        } else {
                            acc
                        }
                    })
                    .map(|el| el.entity)
                    .expect("No strongest creature found"),
            ],
            AutoSingleCreature::Caster => vec![caster],
        }
    }
    fn finalize(&self, candidates: &[Entity]) -> FinalizeEffect {
        match candidates {
            [] => FinalizeEffect::None,
            [only] => FinalizeEffect::ExecuteSingle(*only),
            _ => {
                warn!("Too many items for single target selector");
                FinalizeEffect::None
            }
        }
    }
}

impl IsTargetSelectMode for AutoMultiCreature {
    fn find_suitable(&self, query: &FilterParams, caster: Entity) -> Vec<Entity> {
        match self {
            AutoMultiCreature::AllEnemy => select_enemy(query.creatures, caster),
            AutoMultiCreature::AllFriendly => select_friendly(query.creatures, caster),
            AutoMultiCreature::Random { count: _ } => {
                let mut rng = rand::rng();
                let mut all_creatures = select_all_creatures(query.creatures);
                all_creatures.shuffle(&mut rng);
                all_creatures
            }
        }
    }
    fn finalize(&self, candidates: &[Entity]) -> FinalizeEffect {
        match self {
            AutoMultiCreature::AllEnemy | AutoMultiCreature::AllFriendly => {
                if candidates.is_empty() {
                    FinalizeEffect::None
                } else {
                    FinalizeEffect::ExecuteAll
                }
            }
            AutoMultiCreature::Random { count } => {
                if candidates.is_empty() {
                    return FinalizeEffect::None;
                }

                FinalizeEffect::AwaitingValueSource {
                    value_source: count.clone(),
                }
            }
        }
    }
}

impl IsTargetSelectMode for ManualCreature {
    fn find_suitable(&self, query: &FilterParams, caster: Entity) -> Vec<Entity> {
        match self {
            ManualCreature::Choose { .. } => select_all_creatures(query.creatures),
            ManualCreature::MaxNFriendly { .. } | ManualCreature::ExactlyNFriendly { .. } => {
                select_friendly(query.creatures, caster)
            }
        }
    }
    fn finalize(&self, candidates: &[Entity]) -> FinalizeEffect {
        match self {
            ManualCreature::Choose { min, max } => todo!(),
            ManualCreature::MaxNFriendly { count } => todo!(),
            ManualCreature::ExactlyNFriendly { count } => todo!(),
        }
    }
}

impl IsTargetSelectMode for ManualTile {
    fn find_suitable(&self, query: &FilterParams, caster: Entity) -> Vec<Entity> {
        match self {
            ManualTile::ChooseTiles { .. } | ManualTile::ChooseArea { .. } => {
                select_all_tiles(query.tiles)
            }
        }
    }
    fn finalize(&self, candidates: &[Entity]) -> FinalizeEffect {
        match self {
            ManualTile::ChooseTiles { amount } => todo!(),
            ManualTile::ChooseArea { radius } => todo!(),
        }
    }
}

/// Auto selection for single-player target
impl IsTargetSelectMode for AutoPlayerSingle {
    fn find_suitable(&self, query: &FilterParams, caster: Entity) -> Vec<Entity> {
        let want_turn = matches!(self, AutoPlayerSingle::TurnPlayer);

        query
            .player
            .iter()
            .filter_map(|p| (p.turn_player.is_some() == want_turn).then_some(p.entity))
            .collect()
    }
    fn finalize(&self, candidates: &[Entity]) -> FinalizeEffect {
        match candidates {
            [] => FinalizeEffect::None,
            [only] => FinalizeEffect::ExecuteSingle(*only),
            _ => {
                warn!("Too many items for single target selector");
                FinalizeEffect::None
            }
        }
    }
}

/// Auto selection for multi-player target (currently: all players)
impl IsTargetSelectMode for AutoPlayerMulti {
    fn find_suitable(&self, query: &FilterParams, _caster: Entity) -> Vec<Entity> {
        // only one mode right now
        query.player.iter().map(|p| p.entity).collect()
    }
    fn finalize(&self, candidates: &[Entity]) -> FinalizeEffect {
        FinalizeEffect::ExecuteAll
    }
}

/// Manual player selection: return all players as "suitable" candidates for UI selection
impl IsTargetSelectMode for ManualPlayer {
    fn find_suitable(&self, query: &FilterParams, _caster: Entity) -> Vec<Entity> {
        query.player.iter().map(|p| p.entity).collect()
    }

    fn finalize(&self, candidates: &[Entity]) -> FinalizeEffect {
        todo!()
    }
}

impl IsTargetSelectMode for ManualHand {
    fn find_suitable(&self, query: &FilterParams, _caster: Entity) -> Vec<Entity> {
        query.hand.iter().map(|p| p.entity).collect()
    }

    fn finalize(&self, candidates: &[Entity]) -> FinalizeEffect {
        todo!()
    }
}

impl IsTargetSelectMode for AutoHand {
    fn find_suitable(&self, query: &FilterParams, _caster: Entity) -> Vec<Entity> {
        query
            .hand
            .iter()
            .filter(|card| match self {
                AutoHand::AllCards => true,
                AutoHand::AllCreatures => card.creature.is_some(),
                AutoHand::AllSpells => card.spell.is_some(),
                AutoHand::AllTraps => card.trap.is_some(),
            })
            .map(|card| card.entity)
            .collect()
    }
    fn finalize(&self, _candidates: &[Entity]) -> FinalizeEffect {
        FinalizeEffect::ExecuteAll
    }
}

impl IsTargetSelectMode for () {
    fn find_suitable(&self, _query: &FilterParams, _caster: Entity) -> Vec<Entity> {
        Vec::new()
    }
    fn finalize(&self, _candidates: &[Entity]) -> FinalizeEffect {
        FinalizeEffect::None
    }
}

impl<L: IsTargetSelectMode, R: IsTargetSelectMode> IsTargetSelectMode for Either<L, R> {
    fn find_suitable(&self, query: &FilterParams, caster: Entity) -> Vec<Entity> {
        match self {
            Either::Left(l) => l.find_suitable(query, caster),
            Either::Right(r) => r.find_suitable(query, caster),
        }
    }
    fn finalize(&self, candidates: &[Entity]) -> FinalizeEffect {
        match self {
            Either::Left(l) => l.finalize(candidates),
            Either::Right(r) => r.finalize(candidates),
        }
    }
}

impl<K: TargetKind<C>, C: Constraint> IsTargetSelectMode for TargetSelector<K, C> {
    fn find_suitable(&self, query: &FilterParams, caster: Entity) -> Vec<Entity> {
        self.selection.find_suitable(query, caster)
    }

    fn finalize(&self, candidates: &[Entity]) -> FinalizeEffect {
        self.selection.finalize(candidates)
    }
}

#[derive(Component, Debug, Clone)]
pub enum AnyTargetSelector {
    CreatureSingle(CreatureSel<SingleTarget>),
    CreatureMulti(CreatureSel<MultiTarget>),
    TileSingle(TileSel<SingleTarget>),
    TileMulti(TileSel<MultiTarget>),
    PlayerSingle(PlayerSel<SingleTarget>),
    PlayerMulti(PlayerSel<MultiTarget>),
    HandSingle(HandSel<SingleTarget>),
    HandMulti(HandSel<MultiTarget>),
    CreatureSingleMulti(CreatureSel<Or<SingleTarget, MultiTarget>>),
    TileSingleMulti(TileSel<Or<SingleTarget, MultiTarget>>),
    PlayerSingleMulti(PlayerSel<Or<SingleTarget, MultiTarget>>),
    HandSingleMulti(HandSel<Or<SingleTarget, MultiTarget>>),
}

impl AnyTargetSelector {
    pub fn selection(&self) -> &dyn IsTargetSelectMode {
        match self {
            AnyTargetSelector::CreatureSingle(ts) => &ts.selection,
            AnyTargetSelector::CreatureMulti(ts) => &ts.selection,
            AnyTargetSelector::TileSingle(ts) => &ts.selection,
            AnyTargetSelector::TileMulti(ts) => &ts.selection,
            AnyTargetSelector::PlayerSingle(ts) => &ts.selection,
            AnyTargetSelector::PlayerMulti(ts) => &ts.selection,
            AnyTargetSelector::HandSingle(ts) => &ts.selection,
            AnyTargetSelector::HandMulti(ts) => &ts.selection,
            AnyTargetSelector::CreatureSingleMulti(ts) => &ts.selection,
            AnyTargetSelector::TileSingleMulti(ts) => &ts.selection,
            AnyTargetSelector::PlayerSingleMulti(ts) => &ts.selection,
            AnyTargetSelector::HandSingleMulti(ts) => &ts.selection,
        }
    }

    pub fn validation(&self) -> &dyn IsFilter {
        match self {
            AnyTargetSelector::CreatureSingle(ts) => &ts.validation,
            AnyTargetSelector::CreatureMulti(ts) => &ts.validation,
            AnyTargetSelector::TileSingle(ts) => &ts.validation,
            AnyTargetSelector::TileMulti(ts) => &ts.validation,
            AnyTargetSelector::PlayerSingle(ts) => &ts.validation,
            AnyTargetSelector::PlayerMulti(ts) => &ts.validation,
            AnyTargetSelector::HandSingle(ts) => &ts.validation,
            AnyTargetSelector::HandMulti(ts) => &ts.validation,
            AnyTargetSelector::CreatureSingleMulti(ts) => &ts.validation,
            AnyTargetSelector::TileSingleMulti(ts) => &ts.validation,
            AnyTargetSelector::PlayerSingleMulti(ts) => &ts.validation,
            AnyTargetSelector::HandSingleMulti(ts) => &ts.validation,
        }
    }
}

impl IsWaiter for AnyTargetSelector {
    fn emit_requirements(&self, f: &mut dyn FnMut(Requirement)) {
        match self {
            AnyTargetSelector::CreatureSingle(target_selector) => {
                target_selector.emit_requirements(f)
            }
            AnyTargetSelector::CreatureMulti(target_selector) => {
                target_selector.emit_requirements(f)
            }
            AnyTargetSelector::TileSingle(target_selector) => target_selector.emit_requirements(f),
            AnyTargetSelector::TileMulti(target_selector) => target_selector.emit_requirements(f),
            AnyTargetSelector::PlayerSingle(target_selector) => {
                target_selector.emit_requirements(f)
            }
            AnyTargetSelector::PlayerMulti(target_selector) => target_selector.emit_requirements(f),
            AnyTargetSelector::HandSingle(target_selector) => target_selector.emit_requirements(f),
            AnyTargetSelector::HandMulti(target_selector) => target_selector.emit_requirements(f),
            AnyTargetSelector::CreatureSingleMulti(target_selector) => {
                target_selector.emit_requirements(f)
            }
            AnyTargetSelector::TileSingleMulti(target_selector) => {
                target_selector.emit_requirements(f)
            }
            AnyTargetSelector::PlayerSingleMulti(target_selector) => {
                target_selector.emit_requirements(f)
            }
            AnyTargetSelector::HandSingleMulti(target_selector) => {
                target_selector.emit_requirements(f)
            }
        }
    }
}

impl From<TargetSelector<CreatureTarget, SingleTarget>> for AnyTargetSelector {
    fn from(value: TargetSelector<CreatureTarget, SingleTarget>) -> Self {
        AnyTargetSelector::CreatureSingle(value)
    }
}

impl From<TargetSelector<CreatureTarget, MultiTarget>> for AnyTargetSelector {
    fn from(value: TargetSelector<CreatureTarget, MultiTarget>) -> Self {
        AnyTargetSelector::CreatureMulti(value)
    }
}

impl From<TargetSelector<CreatureTarget, Or<SingleTarget, MultiTarget>>> for AnyTargetSelector {
    fn from(value: TargetSelector<CreatureTarget, Or<SingleTarget, MultiTarget>>) -> Self {
        AnyTargetSelector::CreatureSingleMulti(value)
    }
}

impl From<TargetSelector<PlayerTarget, SingleTarget>> for AnyTargetSelector {
    fn from(value: TargetSelector<PlayerTarget, SingleTarget>) -> Self {
        AnyTargetSelector::PlayerSingle(value)
    }
}

impl From<TargetSelector<PlayerTarget, MultiTarget>> for AnyTargetSelector {
    fn from(value: TargetSelector<PlayerTarget, MultiTarget>) -> Self {
        AnyTargetSelector::PlayerMulti(value)
    }
}

impl From<TargetSelector<PlayerTarget, Or<SingleTarget, MultiTarget>>> for AnyTargetSelector {
    fn from(value: TargetSelector<PlayerTarget, Or<SingleTarget, MultiTarget>>) -> Self {
        AnyTargetSelector::PlayerSingleMulti(value)
    }
}
impl From<TargetSelector<HandTarget, SingleTarget>> for AnyTargetSelector {
    fn from(value: TargetSelector<HandTarget, SingleTarget>) -> Self {
        AnyTargetSelector::HandSingle(value)
    }
}

impl From<TargetSelector<HandTarget, MultiTarget>> for AnyTargetSelector {
    fn from(value: TargetSelector<HandTarget, MultiTarget>) -> Self {
        AnyTargetSelector::HandMulti(value)
    }
}

impl From<TargetSelector<HandTarget, Or<SingleTarget, MultiTarget>>> for AnyTargetSelector {
    fn from(value: TargetSelector<HandTarget, Or<SingleTarget, MultiTarget>>) -> Self {
        AnyTargetSelector::HandSingleMulti(value)
    }
}

impl From<TargetSelector<TileTarget, SingleTarget>> for AnyTargetSelector {
    fn from(value: TargetSelector<TileTarget, SingleTarget>) -> Self {
        AnyTargetSelector::TileSingle(value)
    }
}

impl From<TargetSelector<TileTarget, MultiTarget>> for AnyTargetSelector {
    fn from(value: TargetSelector<TileTarget, MultiTarget>) -> Self {
        AnyTargetSelector::TileMulti(value)
    }
}

impl From<TargetSelector<TileTarget, Or<SingleTarget, MultiTarget>>> for AnyTargetSelector {
    fn from(value: TargetSelector<TileTarget, Or<SingleTarget, MultiTarget>>) -> Self {
        AnyTargetSelector::TileSingleMulti(value)
    }
}

impl<K, C> IsWaiter for SelectionMethod<K, C>
where
    K: TargetKind<C>,
    C: Constraint,
    K::Auto: IsWaiter,
    K::Manual: IsWaiter,
{
    fn emit_requirements(&self, f: &mut dyn FnMut(Requirement)) {
        match self {
            SelectionMethod::Auto(auto_selector) => auto_selector.emit_requirements(f),
            SelectionMethod::Manual(manual_selector) => manual_selector.emit_requirements(f),
        }
    }
}

impl<K, C> IsWaiter for ManualSelector<K, C>
where
    K: TargetKind<C>,
    C: Constraint,
    K::Auto: IsWaiter,
    K::Manual: IsWaiter,
{
    fn emit_requirements(&self, f: &mut dyn FnMut(Requirement)) {
        self.mode.emit_requirements(f)
    }
}

impl<K, C> IsWaiter for AutoSelector<K, C>
where
    K: TargetKind<C>,
    C: Constraint,
    K::Auto: IsWaiter,
    K::Manual: IsWaiter,
{
    fn emit_requirements(&self, f: &mut dyn FnMut(Requirement)) {
        self.mode.emit_requirements(f)
    }
}

impl<K, C> IsWaiter for TargetSelector<K, C>
where
    K: TargetKind<C>,
    C: Constraint,
    K::Auto: IsWaiter,
    K::Manual: IsWaiter,
    K::Filter: IsWaiter,
{
    fn emit_requirements(&self, f: &mut dyn FnMut(Requirement)) {
        self.validation.emit_requirements(f);
        self.selection.emit_requirements(f);
    }
}

impl IsWaiter for () {
    fn emit_requirements(&self, _: &mut dyn FnMut(Requirement)) {}
}

impl IsWaiter for AutoMultiCreature {
    fn emit_requirements(&self, f: &mut dyn FnMut(Requirement)) {
        match self {
            AutoMultiCreature::AllEnemy => {}
            AutoMultiCreature::AllFriendly => {}
            AutoMultiCreature::Random { count } => {
                f(Requirement::value(count.clone()));
            }
        }
    }
}

impl IsWaiter for ManualCreature {
    fn emit_requirements(&self, f: &mut dyn FnMut(Requirement)) {
        match self {
            ManualCreature::Choose { min, max } => {
                f(Requirement::value(min.clone()));
                f(Requirement::value(max.clone()));
            }
            ManualCreature::MaxNFriendly { count } => {
                f(Requirement::value(count.clone()));
            }
            ManualCreature::ExactlyNFriendly { count } => {
                f(Requirement::value(count.clone()));
            }
        }
    }
}

impl IsWaiter for AutoSingleCreature {
    fn emit_requirements(&self, _: &mut dyn FnMut(Requirement)) {
        // no requirements
    }
}

impl IsWaiter for ManualTile {
    fn emit_requirements(&self, f: &mut dyn FnMut(Requirement)) {
        match self {
            ManualTile::ChooseTiles { amount } => {
                f(Requirement::value(amount.clone()));
            }
            ManualTile::ChooseArea { radius } => {
                f(Requirement::value(radius.clone()));
            }
        }
    }
}

// If you also want an impl for AutoPlayerSingle (it was missing before)
impl IsWaiter for AutoPlayerSingle {
    fn emit_requirements(&self, _: &mut dyn FnMut(Requirement)) {
        // no requirements
    }
}

impl IsWaiter for AutoPlayerMulti {
    fn emit_requirements(&self, _: &mut dyn FnMut(Requirement)) {
        // no requirements
    }
}

impl IsWaiter for ManualPlayer {
    fn emit_requirements(&self, _: &mut dyn FnMut(Requirement)) {
        // no requirements
    }
}

impl IsWaiter for AutoHand {
    fn emit_requirements(&self, _: &mut dyn FnMut(Requirement)) {
        // no requirements
    }
}

impl IsWaiter for ManualHand {
    fn emit_requirements(&self, f: &mut dyn FnMut(Requirement)) {
        f(Requirement::value(self.count.clone()));
    }
}

impl IsWaiter for MultiTargetSelector {
    fn emit_requirements(&self, f: &mut dyn FnMut(Requirement)) {
        match self {
            MultiTargetSelector::Creature(target_selector) => target_selector.emit_requirements(f),
            MultiTargetSelector::Tile(target_selector) => target_selector.emit_requirements(f),
            MultiTargetSelector::Player(target_selector) => target_selector.emit_requirements(f),
            MultiTargetSelector::Hand(target_selector) => target_selector.emit_requirements(f),
        }
    }
}

impl<L, R> IsWaiter for Either<L, R>
where
    L: IsWaiter,
    R: IsWaiter,
{
    fn emit_requirements(&self, f: &mut dyn FnMut(Requirement)) {
        match self {
            Either::Left(l) => l.emit_requirements(f),
            Either::Right(r) => r.emit_requirements(f),
        }
    }
}
