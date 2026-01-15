use bevy::ecs::component::Component;
use janet_bindings::types::janetabstract::IsAbstract;

use crate::actions::action_prototype::ValueSource;

mod filters;

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
    type Filter: Clone + std::fmt::Debug + Send + Sync + 'static;
}
pub trait TargetKind<C: Constraint>:
    'static + Send + Sync + Clone + std::fmt::Debug + TargetFilter
{
    type Auto: Clone + std::fmt::Debug + Send + Sync + 'static;
    type Manual: Clone + std::fmt::Debug + Send + Sync + 'static;
}

// generic composition type
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RulesWithExtras<Base, Extra> {
    pub base: Base,
    pub extras: Vec<Extra>,
}

impl<Base, Extra> RulesWithExtras<Base, Extra> {
    pub fn from_base(base: Base) -> Self {
        Self {
            base,
            extras: Vec::new(),
        }
    }
}

impl<Base: Default, Extra> Default for RulesWithExtras<Base, Extra> {
    fn default() -> Self {
        Self {
            base: Base::default(),
            extras: Vec::new(),
        }
    }
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

pub type TileSel<C: Constraint> = TargetSelector<TileTarget, C>;
pub type PlayerSel<C: Constraint> = TargetSelector<PlayerTarget, C>;
pub type HandSel<C: Constraint> = TargetSelector<HandTarget, C>;

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

pub mod systems {

    use bevy::{
        app::{FixedUpdate, Plugin, Update},
        ecs::{
            component::Component,
            entity::Entity,
            hierarchy::{ChildOf, Children},
            query::{QueryData, With},
            schedule::{IntoScheduleConfigs, common_conditions::any_with_component},
            system::{Commands, Query},
        },
    };
    use rand::seq::SliceRandom;

    use crate::{
        actions::{
            action_prototype::NeedsTargeting,
            targeting::{
                AutoHand, AutoMultiCreature, AutoPlayerMulti, AutoPlayerSingle, AutoSingleCreature,
                Constraint, CreatureTarget, HandTarget, ManualCreature, ManualHand, ManualPlayer,
                ManualTile, MultiTarget, PlayerTarget, SingleTarget, TargetKind, TargetSelector,
                TileTarget,
            },
        },
        board::tile::{Position, Tile},
        card::{CreatureCard, CurrentAttack, CurrentDefense, InHand, SpellCard, TrapCard},
        components::{Health, Owner},
        player::{Player, TurnPlayer},
    };

    #[derive(Component)]
    pub struct SuitableTarget {
        pub for_selector: Entity,
    }

    #[derive(QueryData)]
    pub struct CreatureQuery {
        pub health: &'static Health,
        pub current_atttack: &'static CurrentAttack,
        pub current_defense: &'static CurrentDefense,
        pub entity: Entity,
        pub owner: &'static Owner,
    }

    #[derive(Debug, QueryData)]
    struct TileQuery {
        pub entity: Entity,
        pub children: &'static Children,
        pub position: &'static Position,
    }

    #[derive(Debug, QueryData)]
    struct PlayerQuery {
        pub entity: Entity,
        pub turn_player: Option<&'static TurnPlayer>,
    }

    #[derive(Debug, QueryData)]
    struct HandQuery {
        pub entity: Entity,
        pub creature: Option<&'static CreatureCard>,
        pub spell: Option<&'static SpellCard>,
        pub trap: Option<&'static TrapCard>,
        pub in_hand: &'static InHand,
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

    impl AutoSingleCreature {
        pub fn find_suitable(
            &self,
            q_creatures: Query<CreatureQuery>,
            caster: Entity,
        ) -> Vec<Entity> {
            match self {
                AutoSingleCreature::Strongest => vec![
                    q_creatures
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

    impl AutoMultiCreature {
        pub fn find_suitable(
            &self,
            q_creatures: Query<CreatureQuery>,
            caster: Entity,
        ) -> Vec<Entity> {
            match self {
                AutoMultiCreature::AllEnemy => select_enemy(q_creatures, caster),
                AutoMultiCreature::AllFriendly => select_friendly(q_creatures, caster),
                AutoMultiCreature::Random { count } => {
                    let mut rng = rand::rng();
                    let mut all_creatures = select_all_creatures(q_creatures);
                    all_creatures.shuffle(&mut rng);
                    all_creatures
                }
            }
        }
    }

    impl ManualCreature {
        pub fn find_suitable(
            &self,
            q_creatures: Query<CreatureQuery>,
            caster: Entity,
        ) -> Vec<Entity> {
            match self {
                ManualCreature::Choose { .. } => select_all_creatures(q_creatures),
                ManualCreature::MaxNFriendly { .. } | ManualCreature::ExactlyNFriendly { .. } => {
                    select_friendly(q_creatures, caster)
                }
            }
        }
    }

    impl ManualTile {
        fn find_suitable(
            &self,
            q_tiles: Query<TileQuery, With<Tile>>,
            caster: Entity,
        ) -> Vec<Entity> {
            match self {
                ManualTile::ChooseTiles { .. } | ManualTile::ChooseArea { .. } => {
                    select_all_tiles(q_tiles)
                }
            }
        }
    }
    /// Auto selection for single-player target
    impl AutoPlayerSingle {
        fn find_suitable(&self, q_players: Query<PlayerQuery, With<Player>>) -> Vec<Entity> {
            let want_turn = matches!(self, AutoPlayerSingle::TurnPlayer);

            q_players
                .iter()
                .filter_map(|p| (p.turn_player.is_some() == want_turn).then_some(p.entity))
                .collect()
        }
    }

    /// Auto selection for multi-player target (currently: all players)
    impl AutoPlayerMulti {
        fn find_suitable(&self, q_players: Query<PlayerQuery, With<Player>>) -> Vec<Entity> {
            // only one mode right now
            q_players.iter().map(|p| p.entity).collect()
        }
    }

    /// Manual player selection: return all players as "suitable" candidates for UI selection
    impl ManualPlayer {
        fn find_suitable(&self, q_players: Query<PlayerQuery, With<Player>>) -> Vec<Entity> {
            q_players.iter().map(|p| p.entity).collect()
        }
    }

    impl ManualHand {
        fn find_suitable(&self, q_hand: Query<HandQuery>) -> Vec<Entity> {
            q_hand.iter().map(|p| p.entity).collect()
        }
    }

    impl AutoHand {
        fn find_suitable(&self, q_hand: Query<HandQuery>) -> Vec<Entity> {
            q_hand
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

    #[derive(QueryData)]
    struct TargetSelectorQuery<K: TargetKind<C>, C: Constraint> {
        pub entity: Entity,
        pub child_of: &'static ChildOf,
        pub selector: &'static TargetSelector<K, C>,
    }

    fn apply_targeting<TTarget, TCardinality, TAuto, TManual>(
        q_selectors: Query<TargetSelectorQuery<TTarget, TCardinality>, With<NeedsTargeting>>,
        mut commands: Commands,
        mut find_auto: TAuto,
        mut find_manual: TManual,
    ) where
        TTarget: TargetKind<TCardinality>,
        TCardinality: Constraint,
        TAuto: FnMut(Entity, &TTarget::Auto) -> Vec<Entity>,
        TManual: FnMut(Entity, &TTarget::Manual) -> Vec<Entity>,
    {
        for TargetSelectorQueryItem::<TTarget, TCardinality> {
            entity: e_selector,
            child_of: &ChildOf(caster),
            selector,
        } in q_selectors.iter()
        {
            let e = match &selector.selection {
                super::SelectionMethod::Auto(auto_sel) => find_auto(caster, &auto_sel.mode),

                super::SelectionMethod::Manual(manual_sel) => find_manual(caster, &manual_sel.mode),
            }
            .into_iter()
            .map(|e| {
                (
                    e,
                    SuitableTarget {
                        for_selector: e_selector,
                    },
                )
            })
            .collect::<Vec<_>>();
            commands.insert_batch(e);
        }
    }

    fn find_targets_for_creature_single(
        q_selectors: Query<TargetSelectorQuery<CreatureTarget, SingleTarget>, With<NeedsTargeting>>,
        q_creature: Query<CreatureQuery>,
        commands: Commands,
    ) {
        apply_targeting(
            q_selectors,
            commands,
            |caster, mode| mode.find_suitable(q_creature, caster),
            |caster, mode| mode.find_suitable(q_creature, caster),
        );
    }

    fn find_targets_for_creature_multi(
        q_selectors: Query<TargetSelectorQuery<CreatureTarget, MultiTarget>, With<NeedsTargeting>>,
        q_creature: Query<CreatureQuery>,
        commands: Commands,
    ) {
        apply_targeting(
            q_selectors,
            commands,
            |caster, mode| mode.find_suitable(q_creature, caster),
            |caster, mode| mode.find_suitable(q_creature, caster),
        );
    }

    fn find_targets_for_tile_single(
        q_selectors: Query<TargetSelectorQuery<TileTarget, SingleTarget>, With<NeedsTargeting>>,
        q_tiles: Query<TileQuery, With<Tile>>,
        commands: Commands,
    ) {
        apply_targeting(
            q_selectors,
            commands,
            |_caster, _mode| Vec::new(),
            |caster, mode| mode.find_suitable(q_tiles, caster),
        );
    }

    fn find_targets_for_tile_multi(
        q_selectors: Query<TargetSelectorQuery<TileTarget, MultiTarget>, With<NeedsTargeting>>,
        q_tiles: Query<TileQuery, With<Tile>>,
        commands: Commands,
    ) {
        apply_targeting(
            q_selectors,
            commands,
            |_caster, _mode| Vec::new(),
            |caster, mode| mode.find_suitable(q_tiles, caster),
        );
    }

    fn find_targets_for_player_single(
        q_selectors: Query<TargetSelectorQuery<PlayerTarget, SingleTarget>, With<NeedsTargeting>>,
        q_players: Query<PlayerQuery, With<Player>>,
        commands: Commands,
    ) {
        apply_targeting(
            q_selectors,
            commands,
            |_caster, mode| mode.find_suitable(q_players),
            |_caster, mode| mode.find_suitable(q_players),
        )
    }

    fn find_targets_for_player_multi(
        q_selectors: Query<TargetSelectorQuery<PlayerTarget, MultiTarget>, With<NeedsTargeting>>,
        q_players: Query<PlayerQuery, With<Player>>,
        commands: Commands,
    ) {
        apply_targeting(
            q_selectors,
            commands,
            |_caster, mode| mode.find_suitable(q_players),
            |_caster, mode| mode.find_suitable(q_players),
        )
    }

    fn find_targets_for_hand_single(
        q_selectors: Query<TargetSelectorQuery<HandTarget, SingleTarget>, With<NeedsTargeting>>,
        q_hand: Query<HandQuery>,
        commands: Commands,
    ) {
        apply_targeting(
            q_selectors,
            commands,
            |_caster, mode| mode.find_suitable(q_hand),
            |_caster, mode| mode.find_suitable(q_hand),
        )
    }

    fn find_targets_for_hand_multi(
        q_selectors: Query<TargetSelectorQuery<HandTarget, MultiTarget>, With<NeedsTargeting>>,
        q_hand: Query<HandQuery>,
        commands: Commands,
    ) {
        apply_targeting(
            q_selectors,
            commands,
            |_caster, mode| mode.find_suitable(q_hand),
            |_caster, mode| mode.find_suitable(q_hand),
        )
    }

    pub struct TargetPlugin;

    impl Plugin for TargetPlugin {
        fn build(&self, app: &mut bevy::app::App) {
            app.add_systems(
                FixedUpdate,
                (
                    find_targets_for_hand_single,
                    find_targets_for_hand_multi,
                    find_targets_for_creature_single,
                    find_targets_for_creature_multi,
                    find_targets_for_player_single,
                    find_targets_for_player_multi,
                    find_targets_for_tile_single,
                    find_targets_for_tile_multi,
                )
                    .run_if(any_with_component::<NeedsTargeting>),
            );
        }
    }
}
