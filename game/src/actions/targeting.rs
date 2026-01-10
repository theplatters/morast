pub trait Cardinality: 'static {}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SingleTarget;
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MultiTarget;
impl Cardinality for SingleTarget {}
impl Cardinality for MultiTarget {}

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

#[derive(Clone, Debug)]
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
    Random { count: u8 },
}

#[derive(Clone, Debug)]
pub enum ManualCreature {
    Choose { min: u8, max: u8 },
}

#[derive(Clone, Debug)]
pub enum AutoSingleCreature {}

#[derive(Clone, Debug)]
pub enum ManualTile {
    ChooseTiles { amount: u8 },
    ChooseArea { radius: u8 },
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

// Filter structs with defaults
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct CreatureFilters {
    pub min_health: Option<u32>,
    pub max_health: Option<u32>,
    pub health_percent: Option<(u8, u8)>,
    pub damaged_only: bool,
    pub min_attack: Option<u32>,
    pub can_attack: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CreatureExtraRules {}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct TileFilters {
    pub empty_only: bool,
    pub occupied_only: bool,
    pub in_range_of_caster: Option<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TileExtraRules {}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct PlayerFilters {
    pub min_gold: Option<u32>,
    pub max_gold: Option<u32>,
    pub has_cards_in_hand: Option<u8>,
    pub min_health: Option<u16>,
    pub max_health: Option<u16>,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlayerExtraRules {
    TookDamageLastRound,
    PlayedCardThisTurn,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HandFilters {
    pub min_cost: Option<u16>,
    pub max_cost: Option<u16>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum HandExtraRules {
    ExludeCreatures,
    ExcludeSpells,
    ExcludeTraps,
}

impl TargetFilter for CreatureTarget {
    type FilterBase = CreatureFilters;
    type FilterExtra = CreatureExtraRules;
    type Filter = RulesWithExtras<Self::FilterBase, Self::FilterExtra>;
}
impl TargetKind<SingleTarget> for CreatureTarget {
    type Auto = AutoSingleCreature;
    type Manual = ManualCreature;
}

impl TargetKind<MultiTarget> for CreatureTarget {
    type Auto = AutoMultiCreature;
    type Manual = ManualCreature;
}

impl TargetFilter for TileTarget {
    type FilterBase = TileFilters;
    type FilterExtra = TileExtraRules;
    type Filter = RulesWithExtras<Self::FilterBase, Self::FilterExtra>;
}

impl TargetKind<SingleTarget> for TileTarget {
    type Auto = ();
    type Manual = ManualTile;
}

impl TargetKind<MultiTarget> for TileTarget {
    type Auto = ();
    type Manual = ManualTile;
}

impl TargetFilter for PlayerTarget {
    type FilterBase = PlayerFilters;
    type FilterExtra = PlayerExtraRules;
    type Filter = RulesWithExtras<Self::FilterBase, Self::FilterExtra>;
}

impl TargetKind<SingleTarget> for PlayerTarget {
    type Auto = AutoPlayerSingle;
    type Manual = ManualPlayer;
}

impl TargetKind<MultiTarget> for PlayerTarget {
    type Auto = AutoPlayerMulti;
    type Manual = ManualPlayer;
}

impl TargetFilter for HandTarget {
    type FilterBase = HandFilters;
    type FilterExtra = HandExtraRules;
    type Filter = RulesWithExtras<Self::FilterBase, Self::FilterExtra>;
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

mod janet {
    use janet_bindings::bindings::JanetAbstractType;

    use crate::actions::targeting::AnyTargetSelector;

    static mut ANY_TARGET: JanetAbstractType = JanetAbstractType::new(
        c"target/any-target",
        JanetAbstractType::gc::<AnyTargetSelector>,
    );
}
