use std::fmt::Debug;

use crate::game::{game_context::GameContext, phases::Phase};

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
    owner: i32,
    pub action: Box<dyn FnOnce(&mut GameContext)>,
}

impl Debug for Event {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Event")
            .field("priority", &self.priority)
            .finish()
    }
}

impl Event {
    pub fn new(priority: u32, owner: i32, action: impl FnOnce(&mut GameContext) + 'static) -> Self {
        Self {
            priority,
            owner,
            action: Box::new(action),
        }
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
        owner: i32,
        action: impl FnOnce(&mut GameContext) + 'static,
    ) -> Self {
        Self {
            timing,
            event: Event {
                owner,
                priority,
                action: Box::new(action),
            },
        }
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
