use std::fmt::Debug;

use macroquad::math::I16Vec2;

use crate::{
    engine::janet_handler::types::function::Function,
    game::{card::in_play_id::InPlayID, phases::Phase, player::PlayerID},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct EventTiming {
    pub turn: u32,
    pub phase: Phase,
    pub insertion_order: u32,
}

impl EventTiming {
    pub fn new(turn: u32, phase: Phase, insertion_order: u32) -> Self {
        Self {
            turn,
            phase,
            insertion_order,
        }
    }
}

pub struct Event {
    priority: u32,
    owner: PlayerID,
    pub by_id: InPlayID,
    pub action: Function,
    targets: Option<Vec<I16Vec2>>,
}

impl Debug for Event {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Event")
            .field("priority", &self.priority)
            .field("owner", &self.owner)
            .finish()
    }
}

impl Event {
    pub fn new(priority: u32, owner: PlayerID, by_id: InPlayID, action: Function) -> Self {
        Self {
            priority,
            owner,
            by_id,
            action,
            targets: None,
        }
    }

    pub fn with_targets(mut self, targets: Vec<I16Vec2>) -> Self {
        self.targets = Some(targets);
        self
    }
}

impl PartialOrd for Event {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }
}

impl Ord for Event {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.priority.cmp(&other.priority)
    }
}

impl PartialEq for Event {
    fn eq(&self, other: &Self) -> bool {
        self.priority == other.priority
    }
}

impl Eq for Event {}

#[derive(Debug)]
pub struct ScheduledEvent {
    pub timing: EventTiming,
    pub event: Event,
}

impl ScheduledEvent {
    pub fn new(
        timing: EventTiming,
        priority: u32,
        owner: PlayerID,
        by_id: InPlayID,
        action: Function,
    ) -> Self {
        Self {
            timing,
            event: Event::new(priority, owner, by_id, action),
        }
    }

    pub fn with_targets(mut self, targets: Vec<I16Vec2>) -> Self {
        self.event.targets = Some(targets);
        self
    }
}

impl PartialOrd for ScheduledEvent {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ScheduledEvent {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.timing.cmp(&other.timing)
    }
}

impl PartialEq for ScheduledEvent {
    fn eq(&self, other: &Self) -> bool {
        self.timing == other.timing
    }
}

impl Eq for ScheduledEvent {}
