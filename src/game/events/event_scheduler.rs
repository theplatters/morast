use std::collections::BinaryHeap;

use log::debug;

use super::event::{Event, EventTiming, ScheduledEvent};
use crate::{
    engine::janet_handler::types::janetenum::ToVoidPointer,
    game::{error::Error, game_context::GameContext, phases::Phase},
};

#[derive(Debug)]
pub struct GameScheduler {
    pub current_turn: u32,
    current_phase: Phase,
    next_insertion: u32,
    future_events: BinaryHeap<ScheduledEvent>,
    immediate_events: BinaryHeap<Event>,
    deferred_events: BinaryHeap<Event>,
}

impl GameScheduler {
    pub fn new() -> Self {
        Self {
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
        owner: i32,
        action: impl FnOnce(&mut GameContext) -> Result<(), Error> + 'static,
        priority: u32,
    ) {
        let timing = EventTiming::new(
            self.current_turn + turns_ahead,
            Phase::Start,
            self.next_insertion,
        );
        self.next_insertion += 1;
        self.future_events
            .push(ScheduledEvent::new(timing, priority, owner, action));
    }

    // Schedule at the end of the n-th turn including this turn
    pub fn schedule_at_end(
        &mut self,
        turns_including: u32,
        owner: i32,
        action: impl FnOnce(&mut GameContext) -> Result<(), Error> + 'static,
        priority: u32,
    ) {
        let timing = EventTiming::new(
            self.current_turn + turns_including - 1,
            Phase::End,
            self.next_insertion,
        );
        self.next_insertion += 1;
        self.future_events
            .push(ScheduledEvent::new(timing, priority, owner, action));
    }

    // Schedule to execute now (after current batch of events)
    pub fn schedule_now(
        &mut self,
        owner: i32,
        action: impl FnOnce(&mut GameContext) -> Result<(), Error> + 'static,
        priority: u32,
    ) {
        self.immediate_events
            .push(Event::new(priority, owner, action));
    }

    // Schedule to execute after all currently scheduled events
    pub fn schedule_after_current(
        &mut self,
        owner: i32,
        action: impl FnOnce(&mut GameContext) -> Result<(), Error> + 'static,
        priority: u32,
    ) {
        self.deferred_events
            .push(Event::new(priority, owner, action));
    }

    // Advance the game state (call this when progressing phases/turns)
    pub fn process_events(&mut self, context: &mut GameContext) -> Result<(), Error> {
        debug!("Processing events, {:?}", self);
        // Process all deferred events from previous cycle first
        while let Some(event) = self.deferred_events.pop() {
            (event.action)(context)?;
        }

        // Process current phase events
        while let Some(event) = self.future_events.peek() {
            if event.timing.turn < self.current_turn
                || (event.timing.turn == self.current_turn
                    && event.timing.phase < self.current_phase)
            {
                // Remove and execute outdated events

                let event = self.future_events.pop().unwrap();
                (event.event.action)(context)?;
            } else {
                break;
            }
        }

        // Process immediate events
        while let Some(event) = self.immediate_events.pop() {
            debug!("Handling immediate event: {:?}", event);
            (event.action)(context)?;
        }
        Ok(())
    }

    // Call these when progressing through game phases
    pub fn advance_to_phase(&mut self, phase: Phase, context: &mut GameContext) {
        self.current_phase = phase;
        self.process_events(context);
    }

    pub fn advance_turn(&mut self, context: &mut GameContext) {
        self.current_turn += 1;
        self.current_phase = Phase::Start;
        self.process_events(context);
    }

    pub fn get_turn_count(&mut self) -> u32 {
        self.current_turn
    }
}

impl ToVoidPointer for GameScheduler {
    // add code here
}
