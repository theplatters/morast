use std::marker::PhantomData;

use super::targeting::{
    AutoSelector, Constraint, CreatureTarget, ManualSelector, MultiTarget, Or, SelectionMethod,
    SingleTarget, TargetFilter, TargetKind, TargetSelector,
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
    use std::{
        ffi::{CStr, c_void},
        os::raw::c_char,
        ptr,
    };

    use bevy::{asset::ron::value, log::info, ui::auto};
    use janet_bindings::{
        bindings::{
            Janet, JanetAbstract, JanetAbstractType, janet_abstract, janet_fixarity, janet_panic,
            janet_register_abstract_type, janet_unwrap_abstract, janet_wrap_abstract,
        },
        controller::{CoreFunction, Environment},
        types::janetenum::JanetEnum,
    };

    use crate::actions::{
        target_builder::{
            SetCardinality, SetSelection, TargetSelectorBuilder, UnsetCardinality, UnsetSelection,
        },
        targeting::{
            AutoMultiCreature, Constraint, CreatureTarget, HandTarget, MultiTarget, PlayerTarget,
            SingleTarget, TargetKind, TargetSelector, TileTarget,
        },
    };

    type SingleCreatureTargetBuilder = crate::actions::target_builder::TargetSelectorBuilder<
        crate::actions::targeting::CreatureTarget,
        crate::actions::targeting::SingleTarget,
        SetCardinality,
        UnsetSelection,
    >;
    type MultiCreatureTargetBuilder = crate::actions::target_builder::TargetSelectorBuilder<
        crate::actions::targeting::CreatureTarget,
        crate::actions::targeting::MultiTarget,
        SetCardinality,
        UnsetSelection,
    >;
    type SingleTileTargetBuilder = crate::actions::target_builder::TargetSelectorBuilder<
        crate::actions::targeting::TileTarget,
        crate::actions::targeting::SingleTarget,
        SetCardinality,
        UnsetSelection,
    >;
    type MultiTileTargetBuilder = crate::actions::target_builder::TargetSelectorBuilder<
        crate::actions::targeting::TileTarget,
        crate::actions::targeting::MultiTarget,
        SetCardinality,
        UnsetSelection,
    >;
    type SinglePlayerTargetBuilder = crate::actions::target_builder::TargetSelectorBuilder<
        crate::actions::targeting::PlayerTarget,
        crate::actions::targeting::SingleTarget,
        SetCardinality,
        UnsetSelection,
    >;
    type MultiPlayerTargetBuilder = crate::actions::target_builder::TargetSelectorBuilder<
        crate::actions::targeting::PlayerTarget,
        crate::actions::targeting::MultiTarget,
        SetCardinality,
        UnsetSelection,
    >;
    type SingleHandTargetBuilder = crate::actions::target_builder::TargetSelectorBuilder<
        crate::actions::targeting::HandTarget,
        crate::actions::targeting::SingleTarget,
        SetCardinality,
        UnsetSelection,
    >;
    type MultiHandTargetBuilder = crate::actions::target_builder::TargetSelectorBuilder<
        crate::actions::targeting::HandTarget,
        crate::actions::targeting::MultiTarget,
        SetCardinality,
        UnsetSelection,
    >;

    type SingleCreatureTargetBuilderSetSelection =
        crate::actions::target_builder::TargetSelectorBuilder<
            crate::actions::targeting::CreatureTarget,
            crate::actions::targeting::SingleTarget,
            SetCardinality,
            SetSelection,
        >;
    type MultiCreatureTargetBuilderSetSelection =
        crate::actions::target_builder::TargetSelectorBuilder<
            crate::actions::targeting::CreatureTarget,
            crate::actions::targeting::MultiTarget,
            SetCardinality,
            SetSelection,
        >;
    type SingleTileTargetBuilderSetSelection =
        crate::actions::target_builder::TargetSelectorBuilder<
            crate::actions::targeting::TileTarget,
            crate::actions::targeting::SingleTarget,
            SetCardinality,
            SetSelection,
        >;
    type MultiTileTargetBuilderSetSelection = crate::actions::target_builder::TargetSelectorBuilder<
        crate::actions::targeting::TileTarget,
        crate::actions::targeting::MultiTarget,
        SetCardinality,
        SetSelection,
    >;
    type SinglePlayerTargetBuilderSetSelection =
        crate::actions::target_builder::TargetSelectorBuilder<
            crate::actions::targeting::PlayerTarget,
            crate::actions::targeting::SingleTarget,
            SetCardinality,
            SetSelection,
        >;
    type MultiPlayerTargetBuilderSetSelection =
        crate::actions::target_builder::TargetSelectorBuilder<
            crate::actions::targeting::PlayerTarget,
            crate::actions::targeting::MultiTarget,
            SetCardinality,
            SetSelection,
        >;
    type SingleHandTargetBuilderSetSelection =
        crate::actions::target_builder::TargetSelectorBuilder<
            crate::actions::targeting::HandTarget,
            crate::actions::targeting::SingleTarget,
            SetCardinality,
            SetSelection,
        >;
    type MultiHandTargetBuilderSetSelection = crate::actions::target_builder::TargetSelectorBuilder<
        crate::actions::targeting::HandTarget,
        crate::actions::targeting::MultiTarget,
        SetCardinality,
        SetSelection,
    >;

    enum AnyTargetBuilder {
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

    unsafe fn set_auto_for<T, M>(
        any: &mut AnyTargetBuilder,
        b: TargetSelectorBuilder<T, M, SetCardinality, UnsetSelection>,
        value: Janet,
        wrap: fn(TargetSelectorBuilder<T, M, SetCardinality, SetSelection>) -> AnyTargetBuilder,
    ) where
        T: TargetKind<M>,
        M: Constraint,
    {
        let auto_abst = janet_unwrap_abstract(value) as *mut <T as TargetKind<M>>::Auto;
        let auto = std::ptr::read(auto_abst);

        let b2 = b.auto(auto);
        *any = wrap(b2);
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
        let man_abst = janet_unwrap_abstract(value) as *mut <T as TargetKind<M>>::Manual;
        let manual = std::ptr::read(man_abst);

        let b2 = b.manual(manual);
        *any = wrap(b2);
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
        let key_enum = JanetEnum::from(key).unwrap();
        if let Some(key_str) = key_enum.into_string() {
            match key_str.as_str() {
                "auto" => unsafe { any_builder_set_auto(data, value) },
                "manual" => unsafe { any_builder_set_manual(data, value) },
                _ => unsafe { janet_panic(c"unknown key".as_ptr()) },
            }
        }
    }

    unsafe fn make_builder_auto<T: TargetKind<M>, M: Constraint>(data: *mut c_void, value: Janet) {
        let auto_kind_ptr = unsafe { janet_unwrap_abstract(value) } as *mut T::Auto;
        let builder_ptr = data as *mut TargetSelectorBuilder<T, M, SetCardinality, UnsetSelection>;
        let auto_kind = unsafe { ptr::read(auto_kind_ptr) };

        let builder = unsafe { ptr::read(builder_ptr).auto(auto_kind) };
    }

    unsafe extern "C" fn builder_put<T: TargetKind<M>, M: Constraint>(
        data: *mut c_void,
        key: Janet,
        value: Janet,
    ) {
        let key_enum = JanetEnum::from(key).unwrap();
        if let Some(key_str) = key_enum.into_string() {
            match key_str.as_str() {
                "auto" => unsafe { make_builder_auto::<T, M>(data, value) },
                _ => panic!("Not yet implemeted"),
            }
        }
    }

    static mut ANY_TARGET_BUILDER: JanetAbstractType = JanetAbstractType::new(
        c"target/any-taret-builder",
        JanetAbstractType::gc::<AnyTargetBuilder>,
    )
    .with_put_metod(any_builder_put);

    macro_rules! core_fns {
    ($( $name:literal => $cfun:path ; $docs:literal ),* $(,)?) => {
        &[
            $(
                CoreFunction { name: $name, cfun: $cfun, docs: $docs },
            )*
        ] as &[CoreFunction]
    };
    }

    unsafe fn cfun_make_any_builder(
        argc: i32,
        argv: *mut Janet,
        init: impl FnOnce() -> AnyTargetBuilder,
    ) -> Janet {
        unsafe {
            janet_fixarity(argc, 0);

            let abst: JanetAbstract =
                janet_abstract(&raw const ANY_TARGET_BUILDER, size_of::<AnyTargetBuilder>());
            let p = abst as *mut AnyTargetBuilder;
            ptr::write(p, init());
            janet_wrap_abstract(abst)
        }
    }

    macro_rules! def_cfun_target_builder {
        (
        $fn_name:ident,
        $variant:ident,
        $target_ty:ty,
        $mode_ty:ty,
        $mode_method:ident
    ) => {
            pub unsafe extern "C" fn $fn_name(argc: i32, _argv: *mut Janet) -> Janet {
                unsafe {
                    janet_fixarity(argc, 0);

                    let abst: JanetAbstract = janet_abstract(
                        &raw const ANY_TARGET_BUILDER,
                        ::std::mem::size_of::<AnyTargetBuilder>(),
                    );
                    let p = abst as *mut AnyTargetBuilder;

                    let b = TargetSelector::<$target_ty, $mode_ty>::builder().$mode_method();
                    ::std::ptr::write(p, AnyTargetBuilder::$variant(b));

                    janet_wrap_abstract(abst)
                }
            }
        };
    }

    def_cfun_target_builder!(
        cfun_target_single_creature,
        SingleCreatureTargetBuilder,
        CreatureTarget,
        SingleTarget,
        single
    );

    def_cfun_target_builder!(
        cfun_target_multi_creature,
        MultiCreatureTargetBuilder,
        CreatureTarget,
        MultiTarget,
        multi
    );

    def_cfun_target_builder!(
        cfun_target_single_tile,
        SingleTileTargetBuilder,
        TileTarget,
        SingleTarget,
        single
    );

    def_cfun_target_builder!(
        cfun_target_multi_tile,
        MultiTileTargetBuilder,
        TileTarget,
        MultiTarget,
        multi
    );

    def_cfun_target_builder!(
        cfun_target_single_player,
        SinglePlayerTargetBuilder,
        PlayerTarget,
        SingleTarget,
        single
    );

    def_cfun_target_builder!(
        cfun_target_multi_player,
        MultiPlayerTargetBuilder,
        PlayerTarget,
        MultiTarget,
        multi
    );

    def_cfun_target_builder!(
        cfun_target_single_hand,
        SingleHandTargetBuilder,
        HandTarget,
        SingleTarget,
        single
    );

    def_cfun_target_builder!(
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
