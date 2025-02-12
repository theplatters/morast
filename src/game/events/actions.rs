use macroquad::math::U16Vec2;

use crate::game::{card::card_id::CardID, player::PlayerID};

#[derive(Debug, Clone, Copy)]
pub struct CardAction {
    pub card: CardID,
    pub player: PlayerID,
}

#[derive(Debug, Clone, Copy)]
pub struct PlaceOnBoardAction {
    pub cost: i32,
    pub player: PlayerID,
    pub card: CardID,
    pub index: U16Vec2,
}

#[derive(Debug, Clone, Copy)]
pub struct GoldAction {
    pub player: PlayerID,
    pub amount: i32,
}
