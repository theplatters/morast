use std::collections::HashMap;

use log::debug;
use macroquad::math::I16Vec2;

use crate::game::phases::Phase;

use super::{
    board::{card_on_board::CardOnBoard, effect::Effect, place_error::BoardError, Board},
    card::{card_id::CardID, card_registry::CardRegistry, Card},
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
    pub current_selected_card: Option<CardOnBoard>,
    pub current_selected_index: Option<I16Vec2>,
}

impl GameContext {
    pub fn new(players: [Player; 2]) -> Self {
        Self {
            players,
            board: Board::new(),
            turn_player: PlayerID::new(1),
            cards_placed: HashMap::new(),
            current_selected_card: None,
            current_selected_index: None,
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

    pub fn shuffe_deck(&mut self, player_id: PlayerID) -> Option<()> {
        let player = self.get_player_mut(player_id)?;
        player.shuffle_deck();
        Some(())
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
                self.current_selected_card = Some(key);
                self.current_selected_index = Some(index);
                self.cards_placed.insert(key, index);
                card_registry
                    .get(&card_id)
                    .unwrap_or_else(|| panic!("Card {:?} not found", key))
                    .on_place(scheduler, self.turn_player_id().get() as i32);
                scheduler.process_events(self)?;
                self.current_selected_card = None;
                self.current_selected_index = None;
                Ok(())
            }
            Err(err) => Err(Error::PlaceError(err)),
        }
    }

    pub fn proces_turn_end(&mut self, scheduler: &mut GameScheduler, card_registry: &CardRegistry) {
        debug!(
            "Processing turn {:?} beginning ",
            scheduler.get_turn_count()
        );
        scheduler.advance_to_phase(Phase::End, self);
        for (card, index) in self.cards_placed.clone().iter() {
            self.current_selected_card = Some(card.to_owned());
            self.current_selected_index = Some(index.to_owned());
            card_registry
                .get(&card.card_id)
                .expect("Card not found")
                .on_turn_start(scheduler, self.turn_player_id().get() as i32);
        }

        self.current_selected_card = None;
        self.current_selected_index = None;
        scheduler.process_events(self);
    }

    pub fn proces_turn_begin(
        &mut self,
        scheduler: &mut GameScheduler,
        card_registry: &CardRegistry,
    ) {
        self.change_turn_player();
        scheduler.advance_turn(self);
        self.draw_cards(self.turn_player_id(), NUM_CARDS_AT_START)
            .expect("The turn player could not be found, which should never happen");
        for (card, index) in &self.cards_placed.clone() {
            println!("Processing card {:?}", card);
            self.current_selected_card = Some(card.to_owned());
            self.current_selected_index = Some(index.to_owned());
            card_registry
                .get(&card.card_id)
                .expect("Card not found")
                .on_turn_end(scheduler, self.turn_player_id().get() as i32);
        }

        self.current_selected_card = None;
        self.current_selected_index = None;
        scheduler.process_events(self);
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
    pub fn move_card(&mut self, from: I16Vec2, to: I16Vec2) -> Result<(), Error> {
        let result = self.board.move_card(from, to).map_err(Error::PlaceError)?;
        let new_index = self
            .cards_placed
            .entry(result)
            .or_insert(I16Vec2::new(0, 0));
        *new_index = to;
        Ok(())
    }

    pub(crate) fn update_attack_values(&mut self, card_registry: &CardRegistry) {
        let mut removed = self.board.update_attack_values(card_registry);
        while !removed.is_empty() {
            //TODO: Only update the card values for the deleted card
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
            //TODO: Only update the card values for the deleted card
            self.update_attack_values(card_registry);
        }
    }

    pub(crate) fn get_card_at_index(&self, from: &I16Vec2) -> Option<&CardOnBoard> {
        self.cards_placed
            .iter()
            .find_map(|(key, &val)| if val == *from { Some(key) } else { None })
    }
}

impl crate::engine::janet_handler::types::janetenum::ToVoidPointer for GameContext {}
