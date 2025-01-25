use super::event::Event;

pub trait EventHandler: Send + Sync {
    fn handle_event(&mut self, event: &Event) -> Vec<Event>;
}
