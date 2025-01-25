use crate::game::card::Card;

pub struct CardAction {
    pub card: Card,
    pub player: u16,
}

pub struct PlaceOnBoardAction<'a> {
    pub cost: u16,
    pub card: &'a Card,
}
