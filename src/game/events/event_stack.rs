use std::collections::BinaryHeap;

use crate::game::{
    card::card_registry::CardRegistry,
    events::{self, action::Action, action_effect::GameAction, execution_result::ExecutionResult},
};

#[derive(Debug)]
pub struct EventStack {
    events: BinaryHeap<Action>,
}

impl EventStack {
    pub fn new() -> Self {
        Self {
            events: BinaryHeap::new(),
        }
    }

    pub fn schedule(&mut self, action: Action) {
        self.events.push(action);
    }

    pub(crate) fn process_events(
        &mut self,
        context: &mut crate::game::game_context::GameContext,
        card_registry: &CardRegistry,
    ) -> Result<Vec<ExecutionResult>, crate::game::error::Error> {
        let mut events = Vec::new();
        while let Some(action) = self.events.pop() {
            let execution_result = action.execute(context, card_registry)?;
            events.push(execution_result);
        }
        Ok(events)
    }
}
