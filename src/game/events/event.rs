use crate::game::player::PlayerID;

use super::actions::{CardAction, GoldAction, PlaceOnBoardAction};

#[derive(Debug, Clone, Copy)]
pub enum Event {
    DrawCard(PlayerID),
    DiscardCard(PlayerID),
    SendCardToHand(CardAction),
    SendCardToDiscard(CardAction),
    CardDrawn(PlayerID),
    DeckEmpty(PlayerID),
    CardDiscarded(PlayerID),
    HandEmpty(PlayerID),
    GetGold(GoldAction),
    PlaceCard(PlaceOnBoardAction),
    RequestPlace(PlaceOnBoardAction),
    PlaceRequestDenied,
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
    GetGold,
    PlaceCard,
    RequestPlace,
    PlaceRequestDenied,
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
            Event::GetGold(_) => EventType::GetGold,
            Event::PlaceCard(_) => EventType::PlaceCard,
            Event::RequestPlace(_) => EventType::RequestPlace,
            Event::PlaceRequestDenied => EventType::PlaceRequestDenied,
        }
    }
}
