use crate::game::events::{action::Action, event::Event};

pub enum ExecutionResult {
    Executed { event: Option<Event> },
    NeedsTargeting { action: Box<Action> },
}

impl ExecutionResult {}
