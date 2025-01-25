use std::collections::{HashMap, VecDeque};

use super::{
    event::{Event, EventType},
    event_handler::EventHandler,
};

pub struct EventManager {
    subscribers: HashMap<EventType, Vec<Box<dyn EventHandler>>>,
    event_queue: VecDeque<Event>,
}

impl EventManager {
    pub fn new() -> Self {
        EventManager {
            subscribers: HashMap::new(),
            event_queue: VecDeque::new(),
        }
    }

    pub fn subscribe(&mut self, event_type: EventType, handler: Box<dyn EventHandler>) {
        self.subscribers
            .entry(event_type)
            .or_default()
            .push(handler);
    }

    pub fn publish(&mut self, initial_event: Event) {
        self.event_queue.push_back(initial_event);

        while let Some(event) = self.event_queue.pop_front() {
            let event_type = event.event_type();

            if let Some(handlers) = self.subscribers.get_mut(&event_type) {
                for handler in handlers {
                    let events = handler.handle_event(&event);
                    self.event_queue.extend(events);
                }
            }
        }
    }
}
