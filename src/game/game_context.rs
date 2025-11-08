use std::collections::HashMap;

use log::debug;
use macroquad::math::I16Vec2;

use crate::game::phases::Phase;

use super::{
    board::{card_on_board::CardOnBoard, effect::Effect, place_error::BoardError, Board},
    card::{card_id::CardID, card_registry::CardRegistry, in_play_id::InPlayID, Card},
    error::Error,
    events::event_scheduler::GameScheduler,
    player::{Player, PlayerID},
};

const NUM_CARDS_AT_START: u16 = 2;

pub struct GameContext {
    players: [Player; 2],
    board: Board,
    turn_player: PlayerID,
    cards_placed: HashMap<CardOnBoard, I16Vec2>,
}

impl GameContext {
    pub fn new(players: [Player; 2]) -> Self {
        Self {
            players,
            board: Board::new(),
            turn_player: PlayerID::new(1),
            cards_placed: HashMap::new(),
        }
    }

    pub fn change_turn_player(&mut self) {
        self.turn_player = self.turn_player.next();
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
            self.players[1].id
        } else {
            self.players[0].id
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
    pub fn get_player_gold(&self, player_id: PlayerID) -> Result<i64, Error> {
        let player = self.get_player(player_id).ok_or(Error::PlayerNotFound)?;
        Ok(player.get_gold())
    }

    pub fn add_gold(&mut self, player_id: PlayerID, amount: i64) -> Result<(), Error> {
        let player = self
            .get_player_mut(player_id)
            .ok_or(Error::PlayerNotFound)?;

        player.add_gold(amount);
        Ok(())
    }

    pub fn shuffle_deck(&mut self, player_id: PlayerID) -> Option<()> {
        let player = self.get_player_mut(player_id)?;
        player.shuffle_deck();
        Some(())
    }

    pub fn place_card_from_hand(
        &mut self,
        player_id: PlayerID,
        card_index: usize,
        position: I16Vec2,
        card_registry: &CardRegistry,
        scheduler: &mut GameScheduler,
    ) -> Result<(), Error> {
        // Validate side of the board
        if !self.is_on_player_side(position, player_id) {
            return Err(Error::InvalidMove);
        }

        // Ensure the tile is empty
        if self.get_card_at_index(&position).is_some() {
            return Err(Error::PlaceError(BoardError::TileOccupied));
        }

        let player = self
            .get_player_mut(player_id)
            .ok_or(Error::PlayerNotFound)?;

        let card_id = player
            .remove_card_from_hand(card_index)
            .ok_or(Error::CardNotFound)?;

        let card = card_registry.get(&card_id).ok_or(Error::CardNotFound)?;
        if player.get_gold() <= card.cost.into() {
            return Err(Error::InsufficientGold);
        }

        player.remove_gold(card.cost.into());
        // Place onto the board
        self.place(card_id, position, player_id, card_registry, scheduler)?;
        Ok(())
    }

    fn is_on_player_side(&self, pos: I16Vec2, player_id: PlayerID) -> bool {
        let board_height = self.board.height();
        if player_id.get() == 0 {
            pos.y < board_height / 2
        } else {
            pos.y >= board_height / 2
        }
    }

    pub fn place(
        &mut self,
        card_id: CardID,
        index: I16Vec2,
        player_id: PlayerID,
        card_registry: &CardRegistry,
        scheduler: &mut GameScheduler,
    ) -> Result<(), Error> {
        println!("Placing card {:?} at index {:?}", card_id, index);
        match self.board.place(card_id, player_id, index) {
            Ok(id) => {
                let key = CardOnBoard::new(id, card_id, player_id);
                self.cards_placed.insert(key, index);
                card_registry
                    .get(&card_id)
                    .ok_or(Error::CardNotFound)?
                    .on_place(scheduler, self.turn_player_id(), id);
                scheduler.process_events(self)?;
                Ok(())
            }
            Err(err) => Err(Error::PlaceError(err)),
        }
    }

    pub fn process_turn_end(
        &mut self,
        scheduler: &mut GameScheduler,
        card_registry: &CardRegistry,
    ) -> Result<(), Error> {
        debug!(
            "Processing turn {:?} beginning ",
            scheduler.get_turn_count()
        );
        scheduler.advance_to_phase(Phase::End, self);
        for card in self.cards_placed.keys() {
            card_registry
                .get(&card.card_id)
                .ok_or(Error::CardNotFound)?
                .on_turn_start(scheduler, self.turn_player_id(), card.id);
        }

        scheduler.process_events(self)?;
        Ok(())
    }

    pub(crate) fn advance_turn(&mut self, scheduler: &mut GameScheduler) {
        self.change_turn_player();
        scheduler.advance_turn(self);
    }

    pub fn process_turn_begin(
        &mut self,
        scheduler: &mut GameScheduler,
        card_registry: &CardRegistry,
    ) -> Result<(), Error> {
        self.draw_cards(self.turn_player_id(), NUM_CARDS_AT_START)?;
        for card in self.cards_placed.keys() {
            println!("Processing card {:?}", card);
            card_registry
                .get(&card.card_id)
                .ok_or(Error::CardNotFound)?
                .on_turn_end(scheduler, self.turn_player_id(), card.id);
        }

        self.board.update_effects();
        scheduler.process_events(self)?;
        Ok(())
    }

    pub(crate) fn process_main_phase(
        &mut self,
        scheduler: &mut GameScheduler,
        card_registry: &CardRegistry,
    ) -> Result<(), Error> {
        scheduler.advance_to_phase(Phase::Main, self);
        let turn_player = self
            .get_player_mut(self.turn_player_id())
            .ok_or(Error::PlayerNotFound)?;
        Ok(())
    }

    pub(crate) fn add_effects(
        &mut self,
        effect: Effect,
        tiles: &[I16Vec2],
    ) -> Result<(), BoardError> {
        self.board.add_effects(effect, tiles)
    }
    pub(crate) fn remove_effects(
        &mut self,
        effect: Effect,
        tiles: &[I16Vec2],
    ) -> Result<(), BoardError> {
        self.board.remove_effects(effect, tiles)
    }
    pub fn draw_board(&self) {
        self.board.draw();
    }
    pub fn is_legal_move(&self, from: I16Vec2, to: I16Vec2, card: &Card) -> bool {
        card.movement.contains(&(from - to))
    }

    pub fn move_card(
        &mut self,
        from: I16Vec2,
        to: I16Vec2,
        card_registry: &CardRegistry,
    ) -> Result<(), Error> {
        let card_at_start = *self.get_card_at_index(&from).ok_or(Error::TileEmpty)?;

        let card = card_registry
            .get(&card_at_start.card_id)
            .ok_or(Error::CardNotFound)?;
        if !self.is_legal_move(from, to, card) {
            return Err(Error::InvalidMove);
        }
        let result = self.board.move_card(from, to).map_err(Error::PlaceError)?;
        let new_index = self
            .cards_placed
            .entry(result)
            .or_insert(I16Vec2::new(0, 0));
        *new_index = to;

        self.update_attack_values_for_card(card_at_start, from, to, card_registry);

        Ok(())
    }

    pub(crate) fn update_attack_values(&mut self, card_registry: &CardRegistry) {
        let mut removed = self.board.update_attack_values(card_registry);
        while !removed.is_empty() {
            removed = self.board.update_attack_values(card_registry);
        }
    }

    pub(crate) fn update_attack_values_for_card(
        &mut self,
        card_info: CardOnBoard,
        from: I16Vec2,
        to: I16Vec2,
        card_registry: &CardRegistry,
    ) {
        let removed = self
            .board
            .update_attack_values_for_card(card_info, from, to, card_registry);
        self.cards_placed.retain(|k, _| !removed.contains(k));
        //update the attack values for the cards affected by the removed cards
        if !removed.is_empty() {
            self.update_attack_values(card_registry);
        }
    }

    pub(crate) fn get_card_at_index(&self, from: &I16Vec2) -> Option<&CardOnBoard> {
        self.cards_placed
            .iter()
            .find_map(|(key, &val)| if val == *from { Some(key) } else { None })
    }

    pub(crate) fn get_card_owner(&self, id: InPlayID) -> Option<PlayerID> {
        self.cards_placed
            .iter()
            .find(|x| x.0.id == id)
            .map(|x| x.0.player_id)
    }

    pub(crate) fn get_card_index(&self, id: InPlayID) -> Option<I16Vec2> {
        self.cards_placed
            .iter()
            .find(|x| x.0.id == id)
            .map(|x| x.1.to_owned())
    }
}
