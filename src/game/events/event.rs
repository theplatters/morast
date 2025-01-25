use super::actions::CardAction;

pub enum Event {
    DrawCard(u16),
    DiscardCard(u16),
    SendCardToHand(CardAction),
    SendCardToDiscard(CardAction),
    CardDrawn(u16),
    DeckEmpty(u16),
    CardDiscarded(u16),
    HandEmpty(u16),
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub enum EventType {
    DrawCard,
    DiscardCard,
    SendCardToHand,
    SendCardToDiscard,
    CardDrawn,
    DeckEmpty,
    CardDiscarded,
    HandEmpty,
}

impl Event {
    pub fn event_type(&self) -> EventType {
        match self {
            Event::DrawCard(_) => EventType::DrawCard,
            Event::DiscardCard(_) => EventType::DiscardCard,
            Event::SendCardToHand(_) => EventType::SendCardToHand,
            Event::SendCardToDiscard(_) => EventType::SendCardToDiscard,
            Event::CardDrawn(_) => EventType::CardDrawn,
            Event::DeckEmpty(_) => EventType::DeckEmpty,
            Event::CardDiscarded(_) => EventType::CardDiscarded,
            Event::HandEmpty(_) => EventType::HandEmpty,
        }
    }
}
