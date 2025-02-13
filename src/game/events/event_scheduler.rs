use std::collections::BinaryHeap;

use crate::game::{game_context::GameContext, phases::Phase};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct EventTiming {
    turn: u32,
    phase: Phase,
    insertion_order: u32,
}

struct Event {
    priority: u32,
    action: Box<dyn FnOnce(&mut GameContext)>,
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

//TODO: Rework this
struct ScheduledEvent {
    timing: EventTiming,
    event: Event,
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

pub struct GameScheduler<'a> {
    context: &'a mut GameContext,
    current_turn: u32,
    current_phase: Phase,
    next_insertion: u32,
    future_events: BinaryHeap<ScheduledEvent>,
    immediate_events: BinaryHeap<Event>,
    deferred_events: BinaryHeap<Event>,
}

impl<'a> GameScheduler<'a> {
    pub fn new(context: &'a mut GameContext) -> Self {
        Self {
            context,
            current_turn: 0,
            current_phase: Phase::Start,
            next_insertion: 0,
            future_events: BinaryHeap::new(),
            immediate_events: BinaryHeap::new(),
            deferred_events: BinaryHeap::new(),
        }
    }

    // Schedule at the start of the n-th turn after this turn
    pub fn schedule_at_start(
        &mut self,
        turns_ahead: u32,
        priority: u32,
        action: impl FnOnce(&mut GameContext) + 'static,
    ) {
        let timing = EventTiming {
            turn: self.current_turn + turns_ahead,
            phase: Phase::Start,
            insertion_order: self.next_insertion,
        };
        self.next_insertion += 1;
        self.future_events.push(ScheduledEvent {
            timing,
            event: Event {
                priority,
                action: Box::new(action),
            },
        });
    }

    // Schedule at the end of the n-th turn including this turn
    pub fn schedule_at_end(
        &mut self,
        turns_including: u32,
        priority: u32,
        action: impl FnOnce(&mut GameContext) + 'static,
    ) {
        let timing = EventTiming {
            turn: self.current_turn + turns_including - 1,
            phase: Phase::End,
            insertion_order: self.next_insertion,
        };
        self.next_insertion += 1;
        self.future_events.push(ScheduledEvent {
            timing,
            event: Event {
                priority,
                action: Box::new(action),
            },
        });
    }

    // Schedule to execute now (after current batch of events)
    pub fn schedule_now(&mut self, action: impl FnOnce(&mut GameContext) + 'static, priority: u32) {
        self.immediate_events.push(Event {
            priority,
            action: Box::new(action),
        });
    }

    // Schedule to execute after all currently scheduled events
    pub fn schedule_after_current(
        &mut self,
        action: impl FnOnce(&mut GameContext) + 'static,
        priority: u32,
    ) {
        self.deferred_events.push(Event {
            priority,
            action: Box::new(action),
        });
    }

    // Advance the game state (call this when progressing phases/turns)
    pub fn process_events(&mut self) {
        // Process all deferred events from previous cycle first
        while let Some(event) = self.deferred_events.pop() {
            (event.action)(self.context);
        }

        // Process current phase events
        while let Some(event) = self.future_events.peek() {
            if event.timing.turn < self.current_turn
                || (event.timing.turn == self.current_turn
                    && event.timing.phase < self.current_phase)
            {
                // Remove and execute outdated events

                let event = self.future_events.pop().unwrap();
                (event.event.action)(self.context);
            } else {
                break;
            }
        }

        // Process immediate events
        while let Some(event) = self.immediate_events.pop() {
            (event.action)(self.context);
        }
    }

    // Call these when progressing through game phases
    pub fn advance_to_phase(&mut self, phase: Phase) {
        self.current_phase = phase;
        self.process_events();
    }

    pub fn advance_turn(&mut self) {
        self.current_turn += 1;
        self.current_phase = Phase::Start;
        self.process_events();
    }
}
