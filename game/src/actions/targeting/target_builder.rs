use std::marker::PhantomData;

use super::{
    AutoSelector, Constraint, CreatureTarget, ManualSelector, MultiTarget, Or, SelectionMethod,
    SingleTarget, TargetKind, TargetSelector,
};

// ------------------------------------------------------------
// TargetSelector Builder (typestate: must set selection)
// ------------------------------------------------------------

// in target_builder.rs
#[derive(Debug, Clone, Copy)]
pub struct UnsetSelection;
#[derive(Debug, Clone, Copy)]
pub struct SetSelection;
#[derive(Debug, Clone, Copy)]
pub struct UnsetCardinality;
#[derive(Debug, Clone, Copy)]
pub struct SetCardinality;

#[derive(Debug, Clone)]
pub struct TargetSelectorBuilder<K, C, Card, Sel>
where
    C: Constraint,
    K: TargetKind<C>,
{
    selection: Option<SelectionMethod<K, C>>,
    validation: Option<K::Filter>,
    _pd: PhantomData<(K, C, Card, Sel)>,
}

impl<K, C> TargetSelector<K, C>
where
    C: Constraint,
    K: TargetKind<C>,
{
    pub fn builder() -> TargetSelectorBuilder<K, C, UnsetCardinality, UnsetSelection> {
        TargetSelectorBuilder::new()
    }
}

impl<K, C> TargetSelectorBuilder<K, C, UnsetCardinality, UnsetSelection>
where
    C: Constraint,
    K: TargetKind<C>,
{
    pub fn new() -> Self {
        Self {
            selection: None,
            validation: None,
            _pd: PhantomData,
        }
    }
}

impl<K, Card, Sel> TargetSelectorBuilder<K, SingleTarget, Card, Sel>
where
    K: TargetKind<SingleTarget>,
{
    pub fn single(self) -> TargetSelectorBuilder<K, SingleTarget, SetCardinality, Sel> {
        TargetSelectorBuilder {
            selection: self.selection,
            validation: self.validation,
            _pd: PhantomData,
        }
    }
}

impl<K, Card, Sel> TargetSelectorBuilder<K, MultiTarget, Card, Sel>
where
    K: TargetKind<MultiTarget>,
{
    pub fn multi(self) -> TargetSelectorBuilder<K, MultiTarget, SetCardinality, Sel> {
        TargetSelectorBuilder {
            selection: self.selection,
            validation: self.validation,
            _pd: PhantomData,
        }
    }
}

// convenience builder for “either”
pub type AnyCardinality = Or<SingleTarget, MultiTarget>;

impl<K, Card, Sel> TargetSelectorBuilder<K, AnyCardinality, Card, Sel>
where
    K: TargetKind<AnyCardinality>,
{
    pub fn any_cardinality(self) -> TargetSelectorBuilder<K, AnyCardinality, SetCardinality, Sel> {
        TargetSelectorBuilder {
            selection: self.selection,
            validation: self.validation,
            _pd: PhantomData,
        }
    }
}

impl<K, C, Card, Sel> TargetSelectorBuilder<K, C, Card, Sel>
where
    C: Constraint,
    K: TargetKind<C>,
{
    pub fn auto(self, mode: K::Auto) -> TargetSelectorBuilder<K, C, Card, SetSelection> {
        TargetSelectorBuilder {
            selection: Some(SelectionMethod::Auto(AutoSelector::new(mode))),
            validation: self.validation,
            _pd: PhantomData,
        }
    }

    pub fn manual(self, mode: K::Manual) -> TargetSelectorBuilder<K, C, Card, SetSelection> {
        TargetSelectorBuilder {
            selection: Some(SelectionMethod::Manual(ManualSelector::new(mode))),
            validation: self.validation,
            _pd: PhantomData,
        }
    }

    pub fn validation(mut self, validation: K::Filter) -> Self {
        self.validation = Some(validation);
        self
    }

    pub fn map_validation(mut self, f: impl FnOnce(K::Filter) -> K::Filter) -> Self
    where
        K::Filter: Default,
    {
        let cur = self.validation.take().unwrap_or_default();
        self.validation = Some(f(cur));
        self
    }
}

impl<K, C> TargetSelectorBuilder<K, C, SetCardinality, SetSelection>
where
    C: Constraint,
    K: TargetKind<C>,
{
    pub fn build(self) -> TargetSelector<K, C>
    where
        K::Filter: Default,
    {
        TargetSelector::new(self.selection.unwrap(), self.validation.unwrap_or_default())
    }

    pub fn build_strict(self) -> TargetSelector<K, C> {
        TargetSelector::new(self.selection.unwrap(), self.validation.unwrap())
    }
}

impl TargetSelector<CreatureTarget, SingleTarget> {
    pub fn creature_single()
    -> TargetSelectorBuilder<CreatureTarget, SingleTarget, SetCardinality, UnsetSelection> {
        TargetSelector::<CreatureTarget, SingleTarget>::builder().single()
    }
}

impl TargetSelector<CreatureTarget, MultiTarget> {
    pub fn creature_multi()
    -> TargetSelectorBuilder<CreatureTarget, MultiTarget, SetCardinality, UnsetSelection> {
        TargetSelector::<CreatureTarget, MultiTarget>::builder().multi()
    }
}

impl TargetSelector<CreatureTarget, AnyCardinality> {
    pub fn creature_any()
    -> TargetSelectorBuilder<CreatureTarget, AnyCardinality, SetCardinality, UnsetSelection> {
        TargetSelector::<CreatureTarget, AnyCardinality>::builder().any_cardinality()
    }
}

// repeat similarly for Tile/Player/Hand

pub mod janet {
    use std::ffi::c_void;

    use janet_bindings::{
        bindings::{Janet, JanetAbstractType, janet_panic, janet_unwrap_abstract},
        controller::CoreFunction,
        core_fns,
        types::{janetabstract::IsAbstract, janetenum::JanetEnum},
    };

    use crate::actions::targeting::{
        Constraint, CreatureTarget, HandTarget, MultiTarget, PlayerTarget, SingleTarget,
        TargetKind, TileTarget,
        target_builder::{SetCardinality, SetSelection, TargetSelectorBuilder, UnsetSelection},
    };

    type SingleCreatureTargetBuilder = TargetSelectorBuilder<
        crate::actions::targeting::CreatureTarget,
        crate::actions::targeting::SingleTarget,
        SetCardinality,
        UnsetSelection,
    >;
    type MultiCreatureTargetBuilder = TargetSelectorBuilder<
        crate::actions::targeting::CreatureTarget,
        crate::actions::targeting::MultiTarget,
        SetCardinality,
        UnsetSelection,
    >;
    type SingleTileTargetBuilder = TargetSelectorBuilder<
        crate::actions::targeting::TileTarget,
        crate::actions::targeting::SingleTarget,
        SetCardinality,
        UnsetSelection,
    >;
    type MultiTileTargetBuilder = TargetSelectorBuilder<
        crate::actions::targeting::TileTarget,
        crate::actions::targeting::MultiTarget,
        SetCardinality,
        UnsetSelection,
    >;
    type SinglePlayerTargetBuilder = TargetSelectorBuilder<
        crate::actions::targeting::PlayerTarget,
        crate::actions::targeting::SingleTarget,
        SetCardinality,
        UnsetSelection,
    >;
    type MultiPlayerTargetBuilder = TargetSelectorBuilder<
        crate::actions::targeting::PlayerTarget,
        crate::actions::targeting::MultiTarget,
        SetCardinality,
        UnsetSelection,
    >;
    type SingleHandTargetBuilder = TargetSelectorBuilder<
        crate::actions::targeting::HandTarget,
        crate::actions::targeting::SingleTarget,
        SetCardinality,
        UnsetSelection,
    >;
    type MultiHandTargetBuilder = TargetSelectorBuilder<
        crate::actions::targeting::HandTarget,
        crate::actions::targeting::MultiTarget,
        SetCardinality,
        UnsetSelection,
    >;

    type SingleCreatureTargetBuilderSetSelection = TargetSelectorBuilder<
        crate::actions::targeting::CreatureTarget,
        crate::actions::targeting::SingleTarget,
        SetCardinality,
        SetSelection,
    >;
    type MultiCreatureTargetBuilderSetSelection = TargetSelectorBuilder<
        crate::actions::targeting::CreatureTarget,
        crate::actions::targeting::MultiTarget,
        SetCardinality,
        SetSelection,
    >;
    type SingleTileTargetBuilderSetSelection = TargetSelectorBuilder<
        crate::actions::targeting::TileTarget,
        crate::actions::targeting::SingleTarget,
        SetCardinality,
        SetSelection,
    >;
    type MultiTileTargetBuilderSetSelection = TargetSelectorBuilder<
        crate::actions::targeting::TileTarget,
        crate::actions::targeting::MultiTarget,
        SetCardinality,
        SetSelection,
    >;
    type SinglePlayerTargetBuilderSetSelection = TargetSelectorBuilder<
        crate::actions::targeting::PlayerTarget,
        crate::actions::targeting::SingleTarget,
        SetCardinality,
        SetSelection,
    >;
    type MultiPlayerTargetBuilderSetSelection = TargetSelectorBuilder<
        crate::actions::targeting::PlayerTarget,
        crate::actions::targeting::MultiTarget,
        SetCardinality,
        SetSelection,
    >;
    type SingleHandTargetBuilderSetSelection = TargetSelectorBuilder<
        crate::actions::targeting::HandTarget,
        crate::actions::targeting::SingleTarget,
        SetCardinality,
        SetSelection,
    >;
    type MultiHandTargetBuilderSetSelection = TargetSelectorBuilder<
        crate::actions::targeting::HandTarget,
        crate::actions::targeting::MultiTarget,
        SetCardinality,
        SetSelection,
    >;

    pub enum AnyTargetBuilder {
        SingleCreatureTargetBuilder(SingleCreatureTargetBuilder),
        MultiCreatureTargetBuilder(MultiCreatureTargetBuilder),
        SingleTileTargetBuilder(SingleTileTargetBuilder),
        MultiTileTargetBuilder(MultiTileTargetBuilder),
        SinglePlayerTargetBuilder(SinglePlayerTargetBuilder),
        MultiPlayerTargetBuilder(MultiPlayerTargetBuilder),
        SingleHandTargetBuilder(SingleHandTargetBuilder),
        MultiHandTargetBuilder(MultiHandTargetBuilder),
        SingleCreatureTargetBuilderSetSelection(SingleCreatureTargetBuilderSetSelection),
        MultiCreatureTargetBuilderSetSelection(MultiCreatureTargetBuilderSetSelection),
        SingleTileTargetBuilderSetSelection(SingleTileTargetBuilderSetSelection),
        MultiTileTargetBuilderSetSelection(MultiTileTargetBuilderSetSelection),
        SinglePlayerTargetBuilderSetSelection(SinglePlayerTargetBuilderSetSelection),
        MultiPlayerTargetBuilderSetSelection(MultiPlayerTargetBuilderSetSelection),
        SingleHandTargetBuilderSetSelection(SingleHandTargetBuilderSetSelection),
        MultiHandTargetBuilderSetSelection(MultiHandTargetBuilderSetSelection),
        Poison,
    }

    impl IsAbstract for AnyTargetBuilder {
        const SIZE: usize = std::mem::size_of::<AnyTargetBuilder>();

        fn type_info() -> &'static JanetAbstractType {
            const ANY_TARGET_BUILDER_ATYPE: JanetAbstractType = JanetAbstractType::new(
                c"target/any-target-builder",
                JanetAbstractType::gc::<AnyTargetBuilder>,
            )
            .with_put_metod(any_builder_put);
            &ANY_TARGET_BUILDER_ATYPE
        }
    }

    unsafe fn set_auto_for<T, M>(
        any: &mut AnyTargetBuilder,
        b: TargetSelectorBuilder<T, M, SetCardinality, UnsetSelection>,
        value: Janet,
        wrap: fn(TargetSelectorBuilder<T, M, SetCardinality, SetSelection>) -> AnyTargetBuilder,
    ) where
        T: TargetKind<M>,
        M: Constraint,
    {
        unsafe {
            let auto_abst = janet_unwrap_abstract(value) as *mut <T as TargetKind<M>>::Auto;
            let auto = std::ptr::read(auto_abst);

            let b2 = b.auto(auto);
            *any = wrap(b2);
        }
    }

    unsafe fn set_manual_for<T, M>(
        any: &mut AnyTargetBuilder,
        b: TargetSelectorBuilder<T, M, SetCardinality, UnsetSelection>,
        value: Janet,
        wrap: fn(TargetSelectorBuilder<T, M, SetCardinality, SetSelection>) -> AnyTargetBuilder,
    ) where
        T: TargetKind<M>,
        M: Constraint,
    {
        unsafe {
            let man_abst = janet_unwrap_abstract(value) as *mut <T as TargetKind<M>>::Manual;
            let manual = std::ptr::read(man_abst);

            let b2 = b.manual(manual);
            *any = wrap(b2);
        }
    }

    macro_rules! any_builder_setters {
    (
        $any:expr, $value:expr,
        $(
            $base:ident,
            $T:ty,
            single => ($single_unset:ident => $single_set:ident),
            multi  => ($multi_unset:ident  => $multi_set:ident)
        );+ $(;)?
    ) => {{
        match std::mem::replace($any, AnyTargetBuilder::Poison) {
            $(
                AnyTargetBuilder::$single_unset(b) => {
                    set_auto_for::<$T, SingleTarget>($any, b, $value, AnyTargetBuilder::$single_set);
                }
                AnyTargetBuilder::$multi_unset(b) => {
                    set_auto_for::<$T, MultiTarget>($any, b, $value, AnyTargetBuilder::$multi_set);
                }
            )+
            _ => janet_panic(c"auto not supported for this builder".as_ptr()),
        }
    }};
}

    macro_rules! any_builder_setters_manual {
    (
        $any:expr, $value:expr,
        $(
            $base:ident,
            $T:ty,
            single => ($single_unset:ident => $single_set:ident),
            multi  => ($multi_unset:ident  => $multi_set:ident)
        );+ $(;)?
    ) => {{
        match std::mem::replace($any, AnyTargetBuilder::Poison) {
            $(
                AnyTargetBuilder::$single_unset(b) => {
                    set_manual_for::<$T, SingleTarget>($any, b, $value, AnyTargetBuilder::$single_set);
                }
                AnyTargetBuilder::$multi_unset(b) => {
                    set_manual_for::<$T, MultiTarget>($any, b, $value, AnyTargetBuilder::$multi_set);
                }
            )+
            _ => janet_panic(c"manual not supported for this builder".as_ptr()),
        }
    }};
}

    unsafe fn any_builder_set_auto(data: *mut c_void, value: Janet) {
        unsafe {
            let any = &mut *(data as *mut AnyTargetBuilder);

            any_builder_setters!(any, value,
                creature, CreatureTarget,
                    single => (SingleCreatureTargetBuilder => SingleCreatureTargetBuilderSetSelection),
                    multi  => (MultiCreatureTargetBuilder  => MultiCreatureTargetBuilderSetSelection);
                tile, TileTarget,
                    single => (SingleTileTargetBuilder => SingleTileTargetBuilderSetSelection),
                    multi  => (MultiTileTargetBuilder  => MultiTileTargetBuilderSetSelection);
                player, PlayerTarget,
                    single => (SinglePlayerTargetBuilder => SinglePlayerTargetBuilderSetSelection),
                    multi  => (MultiPlayerTargetBuilder  => MultiPlayerTargetBuilderSetSelection);
                hand, HandTarget,
                    single => (SingleHandTargetBuilder => SingleHandTargetBuilderSetSelection),
                    multi  => (MultiHandTargetBuilder  => MultiHandTargetBuilderSetSelection);
            );
        }
    }

    unsafe fn any_builder_set_manual(data: *mut c_void, value: Janet) {
        unsafe {
            let any = &mut *(data as *mut AnyTargetBuilder);

            any_builder_setters_manual!(any, value,
                creature, CreatureTarget,
                    single => (SingleCreatureTargetBuilder => SingleCreatureTargetBuilderSetSelection),
                    multi  => (MultiCreatureTargetBuilder  => MultiCreatureTargetBuilderSetSelection);
                tile, TileTarget,
                    single => (SingleTileTargetBuilder => SingleTileTargetBuilderSetSelection),
                    multi  => (MultiTileTargetBuilder  => MultiTileTargetBuilderSetSelection);
                player, PlayerTarget,
                    single => (SinglePlayerTargetBuilder => SinglePlayerTargetBuilderSetSelection),
                    multi  => (MultiPlayerTargetBuilder  => MultiPlayerTargetBuilderSetSelection);
                hand, HandTarget,
                    single => (SingleHandTargetBuilder => SingleHandTargetBuilderSetSelection),
                    multi  => (MultiHandTargetBuilder  => MultiHandTargetBuilderSetSelection);
            );
        }
    }

    unsafe extern "C" fn any_builder_put(data: *mut c_void, key: Janet, value: Janet) {
        let key_enum = JanetEnum::try_from(key).unwrap();
        if let Some(key_str) = key_enum.into_string() {
            match key_str.as_str() {
                "auto" => unsafe { any_builder_set_auto(data, value) },
                "manual" => unsafe { any_builder_set_manual(data, value) },
                _ => unsafe { janet_panic(c"unknown key".as_ptr()) },
            }
        }
    }

    #[macro_export]
    macro_rules! def_cfun_target_builder {
        (
        $impl_name:ident,     // Rust-level impl fn
        $raw_name:ident,      // Janet ABI wrapper fn
        $variant:ident,
        $target_ty:ty,
        $mode_ty:ty,
        $mode_method:ident
    ) => {
            fn $impl_name(
                _argv: &[janet_bindings::types::janetenum::JanetEnum],
            ) -> Result<
                janet_bindings::types::janetenum::JanetEnum,
                janet_bindings::error::JanetError,
            > {
                let b =
                    $crate::actions::targeting::TargetSelector::<$target_ty, $mode_ty>::builder()
                        .$mode_method();

                print!("Creating target builder");
                Ok(janet_bindings::types::janetenum::JanetEnum::Abstract(
                    janet_bindings::types::janetabstract::JanetAbstract::new(
                        AnyTargetBuilder::$variant(b),
                    ),
                ))
            }

            // Generate the actual Janet ABI wrapper:
            janet_bindings::janet_cfun!($raw_name, $impl_name);
        };
    }

    def_cfun_target_builder!(
        target_single_creature_impl,
        cfun_target_single_creature,
        SingleCreatureTargetBuilder,
        CreatureTarget,
        SingleTarget,
        single
    );

    def_cfun_target_builder!(
        target_multi_creature_impl,
        cfun_target_multi_creature,
        MultiCreatureTargetBuilder,
        CreatureTarget,
        MultiTarget,
        multi
    );

    def_cfun_target_builder!(
        tile_single_creature_impl,
        cfun_target_single_tile,
        SingleTileTargetBuilder,
        TileTarget,
        SingleTarget,
        single
    );

    def_cfun_target_builder!(
        tile_multi_creature_impl,
        cfun_target_multi_tile,
        MultiTileTargetBuilder,
        TileTarget,
        MultiTarget,
        multi
    );

    def_cfun_target_builder!(
        player_single_creature_impl,
        cfun_target_single_player,
        SinglePlayerTargetBuilder,
        PlayerTarget,
        SingleTarget,
        single
    );

    def_cfun_target_builder!(
        player_multi_creature_impl,
        cfun_target_multi_player,
        MultiPlayerTargetBuilder,
        PlayerTarget,
        MultiTarget,
        multi
    );

    def_cfun_target_builder!(
        hand_single_creature_impl,
        cfun_target_single_hand,
        SingleHandTargetBuilder,
        HandTarget,
        SingleTarget,
        single
    );

    def_cfun_target_builder!(
        hand_multi_creature_impl,
        cfun_target_multi_hand,
        MultiHandTargetBuilder,
        HandTarget,
        MultiTarget,
        multi
    );

    pub const BUILDER_FUNCTIONS: &[CoreFunction] = core_fns![
        "creature-single" => cfun_target_single_creature; "Creates a single creature builder",
        "creature-multi" => cfun_target_multi_creature; "Creates a multi creature builder",
        "tile-single" => cfun_target_single_tile; "Creates a single tile builder",
        "tile-multi" => cfun_target_multi_tile; "Creates a multi tile builder",
        "player-single" => cfun_target_single_player; "Creates a single player builder",
        "player-multi" => cfun_target_multi_player; "Creates a multi player builder",
        "hand-single" => cfun_target_single_hand; "Creates a single hand builder",
        "hand-multi" => cfun_target_multi_hand; "Creates a multi hand builder",
    ];
}
