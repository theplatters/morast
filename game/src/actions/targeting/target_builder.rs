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
