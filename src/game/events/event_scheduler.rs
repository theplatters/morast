use std::{collections::BinaryHeap, ffi::c_void};

use log::debug;
use macroquad::math::U16Vec2;

use super::event::{Event, EventTiming, ScheduledEvent};
use crate::{
    engine::janet_handler::{
        bindings::{janet_wrap_integer, janet_wrap_pointer},
        types::function::Function,
    },
    game::{
        card::in_play_id::InPlayID, error::Error, game_context::GameContext, phases::Phase,
        player::PlayerID,
    },
};

enum When {
    Now,
    Start(u32),
    Main(u32),
    End(u32),
}

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

    pub fn schedule(
        &mut self,
        timing: When,
        priority: u32,
        owner: PlayerID,
        by_id: InPlayID,
        action: Function,
        targets: Option<Vec<U16Vec2>>,
    ) {
        let timing = match timing {
            When::Now => {
                let event = match targets {
                    None => Event::new(priority, owner, by_id, action),
                    Some(target) => Event::new(priority, owner, by_id, action).with_targets(target),
                };

                self.immediate_events.push(event);
                return;
            }
            When::Start(turns_ahead) => EventTiming::new(
                self.current_turn + turns_ahead,
                Phase::Start,
                self.next_insertion,
            ),
            When::Main(turns_ahead) => EventTiming::new(
                self.current_turn + turns_ahead,
                Phase::Main,
                self.next_insertion,
            ),
            When::End(turns_ahead) => EventTiming::new(
                self.current_turn + turns_ahead,
                Phase::End,
                self.next_insertion,
            ),
        };

        self.next_insertion += 1;
        let event = match targets {
            None => ScheduledEvent::new(timing, priority, owner, by_id, action),
            Some(targets) => {
                ScheduledEvent::new(timing, priority, owner, by_id, action).with_targets(targets)
            }
        };
        self.future_events.push(event);
    }

    // Schedule at the start of the n-th turn after this turn
    pub fn schedule_at_start(
        &mut self,
        turns_ahead: u32,
        owner: PlayerID,
        by_id: InPlayID,
        action: Function,
        priority: u32,
    ) {
        self.schedule(
            When::Start(turns_ahead),
            priority,
            owner,
            by_id,
            action,
            None,
        );
    }

    pub fn schedule_at_start_with_targets(
        &mut self,
        turns_ahead: u32,
        owner: PlayerID,
        by_id: InPlayID,
        action: Function,
        priority: u32,
        targets: Vec<U16Vec2>,
    ) {
        self.schedule(
            When::Start(turns_ahead),
            priority,
            owner,
            by_id,
            action,
            Some(targets),
        );
    }

    // Schedule at the end of the n-th turn including this turn
    pub fn schedule_at_end(
        &mut self,
        turns_including: u32,
        owner: PlayerID,
        by_id: InPlayID,
        action: Function,
        priority: u32,
    ) {
        self.schedule(
            When::End(turns_including),
            priority,
            owner,
            by_id,
            action,
            None,
        );
    }

    // Schedule at the end of the n-th turn including this turn
    pub fn schedule_at_end_with_targets(
        &mut self,
        turns_including: u32,
        owner: PlayerID,
        by_id: InPlayID,
        action: Function,
        priority: u32,
        targets: Vec<U16Vec2>,
    ) {
        self.schedule(
            When::End(turns_including),
            priority,
            owner,
            by_id,
            action,
            Some(targets),
        );
    }

    // Schedule to execute now (after current batch of events)
    pub fn schedule_now(
        &mut self,
        owner: PlayerID,
        by_id: InPlayID,
        action: Function,
        priority: u32,
    ) {
        self.schedule(When::Now, priority, owner, by_id, action, None);
    }

    // Schedule to execute now (after current batch of events)
    pub fn schedule_now_with_targets(
        &mut self,
        owner: PlayerID,
        by_id: InPlayID,
        action: Function,
        priority: u32,
        targets: Vec<U16Vec2>,
    ) {
        self.schedule(When::Now, priority, owner, by_id, action, Some(targets));
    }

    // Schedule to execute after all currently scheduled events
    pub fn schedule_after_current(
        &mut self,
        owner: PlayerID,
        by_id: InPlayID,
        action: Function,
        priority: u32,
    ) {
        self.deferred_events
            .push(Event::new(priority, owner, by_id, action));
    }

    // Advance the game state (call this when progressing phases/turns)
    pub fn process_events(&mut self, context: &mut GameContext) -> Result<(), Error> {
        // Process all deferred events from previous cycle first
        while let Some(event) = self.deferred_events.pop() {
            unsafe {
                event
                    .action
                    .eval(&[
                        janet_wrap_pointer(context as *mut _ as *mut c_void),
                        janet_wrap_integer(event.by_id.into()),
                    ])
                    .expect("Error when processing deferred event");
            }
        }

        // Process current phase events
        while let Some(ScheduledEvent { timing, event: _ }) = self.future_events.peek() {
            if timing.turn < self.current_turn
                || (timing.turn == self.current_turn && timing.phase < self.current_phase)
            {
                // Remove and execute outdated events

                let ScheduledEvent { timing: _, event } = self.future_events.pop().unwrap();
                unsafe {
                    event
                        .action
                        .eval(&[
                            janet_wrap_pointer(context as *mut _ as *mut c_void),
                            janet_wrap_integer(event.by_id.into()),
                        ])
                        .expect("Error when processing deferred event");
                }
            } else {
                break;
            }
        }

        // Process immediate events
        while let Some(event) = self.immediate_events.pop() {
            debug!("Handling immediate event: {:?}", event);
            unsafe {
                if let Err(e) = event.action.eval(&[
                    janet_wrap_pointer(context as *mut _ as *mut c_void),
                    janet_wrap_integer(event.by_id.into()),
                ]) {
                    println!("Error when processing immediate event: {:?}", e);
                }
            }
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
