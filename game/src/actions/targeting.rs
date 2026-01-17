use bevy::ecs::{component::Component, entity::Entity, query::With, system::Query};
use janet_bindings::types::janetabstract::IsAbstract;
use rand::seq::SliceRandom;

use crate::{
    actions::{
        action_prototype::ValueSource,
        targeting::{
            filters::{FilterParams, IsFilter},
            systems::{CreatureQuery, TileQuery},
        },
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
    type Auto: IsTargetSelectMode;
    type Manual: IsTargetSelectMode;
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

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AutoSelector<K: TargetKind<C>, C: Constraint> {
    pub mode: K::Auto,
    _k: std::marker::PhantomData<(K, C)>,
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
    count: u16,
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

#[derive(Debug, Clone)]
pub enum AnyTargetSelector {
    CreatureSingle(CreatureSel<SingleTarget>),
    CreatureMulti(CreatureSel<MultiTarget>),
    TileSingle(TileSel<SingleTarget>),
    TileMulti(TileSel<MultiTarget>),
    PlayerSingle(PlayerSel<SingleTarget>),
    PlayerMulti(PlayerSel<MultiTarget>),
    HandSingle(HandSel<SingleTarget>),
    HandMulti(HandSel<MultiTarget>),
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

pub trait IsTargetSelectMode: Clone + std::fmt::Debug + Send + Sync + 'static {
    fn find_suitable(&self, query: &FilterParams, caster: Entity) -> Vec<Entity>;
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
}

impl IsTargetSelectMode for ManualTile {
    fn find_suitable(&self, query: &FilterParams, caster: Entity) -> Vec<Entity> {
        match self {
            ManualTile::ChooseTiles { .. } | ManualTile::ChooseArea { .. } => {
                select_all_tiles(query.tiles)
            }
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
}

/// Auto selection for multi-player target (currently: all players)
impl IsTargetSelectMode for AutoPlayerMulti {
    fn find_suitable(&self, query: &FilterParams, _caster: Entity) -> Vec<Entity> {
        // only one mode right now
        query.player.iter().map(|p| p.entity).collect()
    }
}

/// Manual player selection: return all players as "suitable" candidates for UI selection
impl IsTargetSelectMode for ManualPlayer {
    fn find_suitable(&self, query: &FilterParams, _caster: Entity) -> Vec<Entity> {
        query.player.iter().map(|p| p.entity).collect()
    }
}

impl IsTargetSelectMode for ManualHand {
    fn find_suitable(&self, query: &FilterParams, _caster: Entity) -> Vec<Entity> {
        query.hand.iter().map(|p| p.entity).collect()
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
}

impl IsTargetSelectMode for () {
    fn find_suitable(&self, query: &FilterParams, caster: Entity) -> Vec<Entity> {
        Vec::new()
    }
}

impl<L: IsTargetSelectMode, R: IsTargetSelectMode> IsTargetSelectMode for Either<L, R> {
    fn find_suitable(&self, query: &FilterParams, caster: Entity) -> Vec<Entity> {
        match self {
            Either::Left(l) => l.find_suitable(query, caster),
            Either::Right(r) => r.find_suitable(query, caster),
        }
    }
}
