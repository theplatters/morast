use bevy::ecs::query::With;
use bevy::ecs::{
    entity::Entity,
    query::{Added, AnyOf},
    system::{Commands, Query},
};

use crate::actions::{
    NeedsTargeting, Pending, RequiredForCompletion, Requirement, UnitAction,
    targeting::AnyTargetSelector, value_source::ValueSource,
};
