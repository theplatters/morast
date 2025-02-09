use macroquad::math::{U16Vec2, Vec2};

use crate::game::{
    card::{card_registry::CardID, Card},
    player::PlayerID,
};

pub struct CardAction {
    pub card: CardID,
    pub player: PlayerID,
}

pub struct PlaceOnBoardAction<'a> {
    pub cost: u16,
    pub card: &'a CardID,
    pub index: U16Vec2,
}

pub struct GoldAction {
    pub player: PlayerID,
    pub amount: i32,
}
