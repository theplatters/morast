use log::debug;
use macroquad::{math::I16Vec2, rand::ChooseRandom};

use crate::game::{
    board::card_on_board::CardOnBoard,
    card::{creature::Creature, Placeable},
    game_objects::player_base::PlayerBaseStatus,
    phases::Phase,
};

use super::{
    board::{effect::Effect, place_error::BoardError, Board},
    card::{card_id::CardID, card_registry::CardRegistry, in_play_id::InPlayID},
    error::Error,
    events::event_scheduler::GameScheduler,
    player::{Player, PlayerID},
};

const NUM_CARDS_AT_START: u16 = 2;

pub struct GameContext {
    players: [Player; 2],
    board: Board,
    turn_player: PlayerID,
}

impl GameContext {
    pub fn standard_deck(card_registry: &CardRegistry) -> Vec<CardID> {
        let mut deck = Vec::new();
        for key in card_registry.registered_ids() {
            deck.extend(std::iter::repeat_n(*key, 4));
        }
        deck.shuffle();
        deck
    }

    pub fn new(card_registry: &CardRegistry) -> Self {
        let players = [
            Player::new(PlayerID::new(0), GameContext::standard_deck(card_registry)),
            Player::new(PlayerID::new(1), GameContext::standard_deck(card_registry)),
        ];
        Self {
            players,
            board: Board::new(),
            turn_player: PlayerID::new(1),
        }
    }

    pub fn get_board(&self) -> &Board {
        &self.board
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
        if self.board.get_card_at_index(&position).is_some() {
            return Err(Error::PlaceError(BoardError::TileOccupied));
        }

        let player = self
            .get_player_mut(player_id)
            .ok_or(Error::PlayerNotFound)?;

        let card_id = player
            .get_card_in_hand(card_index)
            .ok_or(Error::CardNotFound)?;

        let card = card_registry.get(&card_id).ok_or(Error::CardNotFound)?;

        if player.get_gold() <= card.cost.into() {
            return Err(Error::InsufficientGold);
        }

        player.remove_card_from_hand(card_index);

        player.remove_gold(card.cost.into());
        // Place onto the board
        self.place(card_id, position, card_registry, scheduler)?;
        Ok(())
    }

    pub fn is_on_player_side(&self, pos: I16Vec2, player_id: PlayerID) -> bool {
        let board_width = self.board.width();
        if player_id.get() == 0 {
            pos.x < board_width / 4
        } else {
            pos.x >= board_width - board_width / 4
        }
    }

    pub fn place(
        &mut self,
        card_id: CardID,
        index: I16Vec2,
        card_registry: &CardRegistry,
        scheduler: &mut GameScheduler,
    ) -> Result<(), Error> {
        println!("Placing card {:?} at index {:?}", card_id, index);

        let card = card_registry
            .get_creature(&card_id)
            .ok_or(Error::CardNotFound)?;
        let card_on_board = CardOnBoard::new(card_id, self.turn_player_id(), card.movement_points);
        match self.board.place(index, card_on_board) {
            Ok(id) => {
                card.on_place(scheduler, self.turn_player_id(), id);
                scheduler.process_events(self)?;
            }
            Err(err) => Err(Error::PlaceError(err))?,
        }

        self.update_attack_values(card_registry)?;
        Ok(())
    }

    pub fn get_turn_player(&self) -> Option<&Player> {
        self.players.get(self.turn_player_id().index())
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
        for (id, card) in &self.board.cards_placed {
            card_registry
                .get_creature(&card.card_id)
                .ok_or(Error::CardNotFound)?
                .on_turn_start(scheduler, self.turn_player_id(), *id);
        }

        scheduler.process_events(self)?;
        match self.board.player_base_take_damage() {
            [PlayerBaseStatus::Alive, PlayerBaseStatus::Destroyed] => {
                println!("Player 1 wins");
                return Ok(());
            }

            [PlayerBaseStatus::Destroyed, PlayerBaseStatus::Alive] => {
                println!("Player 2 wins");
                return Ok(());
            }
            [PlayerBaseStatus::Destroyed, PlayerBaseStatus::Destroyed] => {
                println!("Draw");
                return Ok(());
            }

            _ => {}
        }
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
        for (id, card) in &self.board.cards_placed {
            println!("Processing card {:?}", card);
            card_registry
                .get_creature(&card.card_id)
                .ok_or(Error::CardNotFound)?
                .on_turn_end(scheduler, self.turn_player_id(), *id);
        }

        self.board
            .refresh_movement_points(self.turn_player_id(), card_registry)?;
        self.board.update_effects(self.turn_player);
        scheduler.process_events(self)?;
        Ok(())
    }

    pub(crate) fn process_main_phase(
        &mut self,
        scheduler: &mut GameScheduler,
    ) -> Result<(), Error> {
        scheduler.advance_to_phase(Phase::Main, self);
        Ok(())
    }

    pub(crate) fn add_effects(
        &mut self,
        effect: Effect,
        tiles: &[I16Vec2],
    ) -> Result<(), BoardError> {
        self.board.add_effects(effect, tiles)
    }
    pub(crate) fn _remove_effects(
        &mut self,
        effect: Effect,
        tiles: &[I16Vec2],
    ) -> Result<(), BoardError> {
        self.board.remove_effects(effect, tiles)
    }
    pub fn is_legal_move(&self, from: I16Vec2, to: I16Vec2, card: &Creature) -> bool {
        card.movement.contains(&(from - to))
    }

    pub fn move_card(
        &mut self,
        from: I16Vec2,
        to: I16Vec2,
        card_registry: &CardRegistry,
    ) -> Result<(), Error> {
        let card_at_start = *self
            .board
            .get_card_at_index(&from)
            .ok_or(Error::PlaceError(BoardError::TileEmpty))?;

        let card = card_registry
            .get_creature(&card_at_start.card_id)
            .ok_or(Error::CardNotFound)?;

        if !self.is_legal_move(from, to, card) {
            return Err(Error::InvalidMove);
        }
        self.board
            .move_card(from, to, self.turn_player)
            .map_err(Error::PlaceError)?;
        self.update_attack_values(card_registry)?;
        Ok(())
    }

    pub(crate) fn update_attack_values(
        &mut self,
        card_registry: &CardRegistry,
    ) -> Result<(), Error> {
        let mut removed = self.board.update_attack_values(card_registry)?;

        while !removed.is_empty() {
            removed = self.board.update_attack_values(card_registry)?;
        }
        Ok(())
    }

    pub(crate) fn get_card_owner(&self, id: InPlayID) -> Option<PlayerID> {
        self.board
            .cards_placed
            .iter()
            .find(|x| *x.0 == id)
            .map(|x| x.1.player_id)
    }

    pub(crate) fn get_turn_player_mut(&mut self) -> Option<&mut Player> {
        self.players.get_mut(self.turn_player.index())
    }
}
