use macroquad::math::{IVec2, U16Vec2, UVec2};

use super::{
    board::{card_on_board::CardOnBoard, place_error::PlaceError, Board},
    card::{card_id::CardID, card_registry::CardRegistry},
    error::Error,
    player::{Player, PlayerID},
};

pub struct GameContext {
    players: [Player; 2],
    board: Board,
    turn_player: PlayerID,
    cards_placed: Vec<CardOnBoard>,
    pub card_registry: CardRegistry,
}

impl GameContext {
    pub fn new(players: [Player; 2]) -> Self {
        Self {
            players,
            board: Board::new(),
            turn_player: PlayerID::new(0),
            card_registry: CardRegistry::new(),
            cards_placed: Vec::new(),
        }
    }

    pub fn change_turn_player(&mut self) {
        self.turn_player = self.other_player_id();
    }
    pub fn turn_player_id(&self) -> PlayerID {
        self.turn_player
    }

    pub fn get_player_mut(&mut self, id: PlayerID) -> Option<&mut Player> {
        self.players.iter_mut().find(|p| p.id == id)
    }
    pub fn get_player(&self, id: PlayerID) -> Option<&Player> {
        self.players.iter().find(|p| p.id == id)
    }

    pub fn other_player_id(&self) -> PlayerID {
        if self.players[0].id == self.turn_player {
            self.turn_player
        } else {
            self.players[1].id
        }
    }

    pub fn draw_cards(&mut self, player_id: PlayerID, num_cards: u16) -> Result<(), Error> {
        let player = self
            .get_player_mut(player_id)
            .ok_or(Error::PlayerNotFound)?;

        for _ in 0..num_cards {
            if let Some(card) = player.draw_from_deck() {
                player.add_to_hand(card);
            }
        }
        Ok(())
    }

    pub fn discard_cards(&mut self, player_id: PlayerID, num_cards: u16) -> Result<(), Error> {
        let player = self
            .get_player_mut(player_id)
            .ok_or(Error::PlayerNotFound)?;
        for _ in 0..num_cards {
            player.discard_card()
        }
        Ok(())
    }

    pub fn set_gold(&mut self, player_id: PlayerID, amount: i64) -> Result<(), Error> {
        let player = self
            .get_player_mut(player_id)
            .ok_or(Error::PlayerNotFound)?;

        player.set_gold(amount as i32);
        Ok(())
    }

    pub fn shuffe_deck(&mut self, player_id: PlayerID) -> Option<()> {
        let player = self.get_player_mut(player_id)?;
        player.shuffle_deck();
        Some(())
    }

    pub fn place(
        &mut self,
        card_id: CardID,
        index: U16Vec2,
        player_id: PlayerID,
    ) -> Result<(), Error> {
        match self.board.place(card_id, player_id, index) {
            Ok(_) => {
                self.cards_placed.push(CardOnBoard::new(card_id, player_id));
                Ok(())
            }
            Err(err) => Err(Error::PlaceError(err)),
        }
    }
}

impl crate::engine::janet_handler::types::janetenum::ToVoidPointer for GameContext {}
