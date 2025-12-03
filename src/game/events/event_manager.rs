use std::{collections::BinaryHeap, fmt::Debug};

use crate::game::{
    card::card_registry::CardRegistry,
    error::Error,
    events::{
        action::{Action, ActionTiming},
        event::Event,
        event_stack::EventStack,
    },
    game_context::GameContext,
    phases::Phase,
};

pub struct ScheduledAction {
    pub action: Action,
    pub execute_turn: u32,
    pub execute_phase: Phase,
    pub insertion_order: u32, // For tie-breaking
}

impl Debug for ScheduledAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ScheduledAction")
            .field("execute_turn", &self.execute_turn)
            .field("execute_phase", &self.execute_phase)
            .field("insertion_order", &self.insertion_order)
            .finish()
    }
}

impl PartialEq for ScheduledAction {
    fn eq(&self, other: &Self) -> bool {
        self.execute_turn == other.execute_turn
            && self.execute_phase == other.execute_phase
            && self.insertion_order == other.insertion_order
    }
}

impl Eq for ScheduledAction {}

impl PartialOrd for ScheduledAction {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ScheduledAction {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // BinaryHeap is max-heap, so reverse for min-heap behavior
        other
            .execute_turn
            .cmp(&self.execute_turn)
            .then(other.execute_phase.cmp(&self.execute_phase))
            .then(other.insertion_order.cmp(&self.insertion_order))
    }
}
#[derive(Debug)]
pub struct EventManager {
    pub current_turn: u32,
    current_phase: Phase,
    next_insertion: u32,
    future_events: BinaryHeap<ScheduledAction>,
    conditional_events: Vec<Action>,
    stack: EventStack,
}

impl EventManager {
    pub fn new() -> Self {
        Self {
            current_turn: 0,
            current_phase: Phase::Start,
            next_insertion: 0,
            future_events: BinaryHeap::new(),
            conditional_events: Vec::new(),
            stack: EventStack::new(),
        }
    }

    pub fn schedule(&mut self, action: Action) {
        match &action.timing {
            ActionTiming::Immediate => self.stack.schedule(action.to_owned()),
            ActionTiming::Delayed { phase, turns } => {
                let scheduled = ScheduledAction {
                    action: action.clone(),
                    execute_turn: self.current_turn + turns,
                    execute_phase: *phase,
                    insertion_order: self.next_insertion,
                };
                self.next_insertion += 1;
                self.future_events.push(scheduled);
            }
            ActionTiming::AtTrigger { .. } => self.conditional_events.push(action.to_owned()),
        }
    }

    pub fn process_events(
        &mut self,
        context: &mut GameContext,
        card_registry: &CardRegistry,
    ) -> Result<Vec<Event>, Error> {
        self.stack.process_events(context, card_registry)
    }

    // Call these when progressing through game phases
    pub fn advance_to_phase(&mut self, phase: Phase) {
        self.current_phase = phase;
        self.check_ready_events();
    }

    pub fn advance_turn(&mut self) {
        self.current_turn += 1;
        self.current_phase = Phase::Start;

        // Only check the top of the heap - O(log n) per ready event
        self.check_ready_events();
    }

    fn check_ready_events(&mut self) {
        while let Some(scheduled) = self.future_events.peek() {
            if scheduled.execute_turn >= self.current_turn
                && scheduled.execute_phase >= self.current_phase
            {
                let scheduled = self.future_events.pop().unwrap();
                self.stack.schedule(scheduled.action);
            } else {
                break; // Heap is ordered, so no more ready events
            }
        }
    }

    pub fn get_turn_count(&mut self) -> u32 {
        self.current_turn
    }
}
