use bevy::ecs::prelude::*;

use crate::game::{
    board::{
        place_error::BoardError,
        tile::{Occupant, Tile},
        BoardRes,
    },
    card::{Cost, CreatureCard, InHand, OnBoard},
    components::Owner,
    error::GameError,
    player::PlayerResources,
    turn_controller::CardPlayRequested,
};

#[derive(Message)]
pub struct CardPlayed {
    pub card: Entity,
}

pub fn place_card(
    mut card_place_requests: MessageReader<CardPlayRequested>,
    mut card_placed: MessageWriter<CardPlayed>,
    free_tiles: Query<&Tile, Without<Occupant>>,
    cards: Query<(&Cost, &Owner), With<CreatureCard>>,
    mut players: Query<&mut PlayerResources>,
    board: Res<BoardRes>,
    mut commands: Commands,
) -> Result {
    for card_place_request in card_place_requests.read() {
        let tile = board
            .get_tile(&card_place_request.position)
            .ok_or(BoardError::TileNotFound)?;
        if !free_tiles.contains(tile) {
            return Err(BoardError::TileOccupied.into());
        }

        let (cost, owner) = cards.get(card_place_request.card)?;

        let mut player = players.get_mut(owner.0)?;

        if player.gold < cost.value {
            return Err(GameError::InsufficientGold.into());
        }

        player.gold -= cost.value;

        commands
            .entity(card_place_request.card)
            .remove::<InHand>()
            .insert(OnBoard { position: tile });
        card_placed.write(CardPlayed {
            card: card_place_request.card,
        });
    }
    Ok(())
}
