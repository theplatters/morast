use std::marker::PhantomData;

use crate::game::actions::targeting::{
    AutoSelector, Constraint, ManualSelector, SelectionMethod, TargetFilter, TargetKind,
    TargetSelector,
};

// ------------------------------------------------------------
// TargetSelector Builder (typestate: must set selection)
// ------------------------------------------------------------

#[derive(Debug, Clone, Copy)]
pub struct UnsetSelection;
#[derive(Debug, Clone, Copy)]
pub struct SetSelection;

#[derive(Debug, Clone)]
pub struct TargetSelectorBuilder<K, C, S>
where
    C: Constraint,
    K: TargetKind<C>,
{
    selection: Option<SelectionMethod<K, C>>,
    validation: Option<K::Filter>,
    _pd: PhantomData<(K, C, S)>,
}

impl<K, C> TargetSelector<K, C>
where
    C: Constraint,
    K: TargetKind<C>,
{
    /// Start building a `TargetSelector<K, C>`.
    pub fn builder() -> TargetSelectorBuilder<K, C, UnsetSelection> {
        TargetSelectorBuilder::new()
    }
}

impl<K, C> TargetSelectorBuilder<K, C, UnsetSelection>
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

impl<K, C, S> TargetSelectorBuilder<K, C, S>
where
    C: Constraint,
    K: TargetKind<C>,
{
    /// Choose auto targeting mode.
    pub fn auto(self, mode: K::Auto) -> TargetSelectorBuilder<K, C, SetSelection> {
        TargetSelectorBuilder {
            selection: Some(SelectionMethod::Auto(AutoSelector::new(mode))),
            validation: self.validation,
            _pd: PhantomData,
        }
    }

    /// Choose manual targeting mode.
    pub fn manual(self, mode: K::Manual) -> TargetSelectorBuilder<K, C, SetSelection> {
        TargetSelectorBuilder {
            selection: Some(SelectionMethod::Manual(ManualSelector::new(mode))),
            validation: self.validation,
            _pd: PhantomData,
        }
    }

    /// Provide/override validation rules.
    pub fn validation(mut self, validation: K::Filter) -> Self {
        self.validation = Some(validation);
        self
    }

    /// Mutate the validation rules, defaulting to `K::Filter::default()` if unset.
    pub fn map_validation(mut self, f: impl FnOnce(K::Filter) -> K::Filter) -> Self
    where
        K::Filter: Default,
    {
        let cur = self.validation.take().unwrap_or_default();
        self.validation = Some(f(cur));
        self
    }
}

impl<K, C> TargetSelectorBuilder<K, C, SetSelection>
where
    C: Constraint,
    K: TargetKind<C>,
{
    pub fn build(self) -> TargetSelector<K, C>
    where
        K::Filter: Default,
    {
        TargetSelector::new(
            self.selection
                .expect("internal invariant: selection is SetSelection but missing"),
            self.validation.unwrap_or_default(),
        )
    }

    /// Like `build`, but requires that validation is explicitly set.
    pub fn build_strict(self) -> TargetSelector<K, C> {
        TargetSelector::new(
            self.selection
                .expect("internal invariant: selection is SetSelection but missing"),
            self.validation
                .expect("TargetSelectorBuilder::build_strict requires validation(...)"),
        )
    }
}
