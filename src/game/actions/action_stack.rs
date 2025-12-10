use std::collections::BinaryHeap;

use crate::game::{
    actions::{action::Action, action_effect::GameAction},
    card::card_registry::CardRegistry,
    events::event::Event,
};

#[derive(Debug)]
pub struct ActionStack {
    events: BinaryHeap<Action>,
}

impl ActionStack {
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
    ) -> Result<Vec<Event>, crate::game::error::Error> {
        let mut events = Vec::new();
        while let Some(action) = self.events.pop() {
            if let Some(event) = action.execute(context, card_registry)? {
                events.push(event);
            }
        }
        Ok(events)
    }
}
