use bevy::ecs::{
    component::Component,
    entity::Entity,
    message::{Message, MessageReader},
    system::Commands,
};

use crate::game::actions::targeting::TargetSelector;

#[derive(Component, Clone, Copy)]
pub struct TargetSelected;

#[derive(Component, Clone, Copy)]
pub struct TargetCandidate;

#[derive(Message)]
pub struct RequestManualTargets {
    pub action_entity: Entity,
}
