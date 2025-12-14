use log::debug;
use macroquad::math::I16Vec2;

use crate::game::{
    board::card_on_board::CreatureOnBoard,
    card::{creature::Creature, deck_builder::DeckBuilder, trap_card::Trap, Card, CardBehavior},
    phases::Phase,
};

use super::{
    actions::action_manager::ActionManager,
    board::{effect::Effect, place_error::BoardError, Board},
    card::{card_id::CardID, card_registry::CardRegistry, in_play_id::InPlayID},
    error::Error,
    player::{Player, PlayerID},
};

const NUM_CARDS_AT_START: u16 = 2;

pub struct GameContext {
    players: [Player; 2],
    turn_player: PlayerID,
}

impl GameContext {
    pub fn place_creature(
        &mut self,
        card_id: CardID,
        creature: &Creature,
        index: I16Vec2,
        card_registry: &CardRegistry,
    ) -> Result<InPlayID, Error> {
        println!("Placing card {:?} at index {:?}", creature.name(), index);

        // Ensure the tile is empty
        if self.board.get_card_at_index(&index).is_some() {
            return Err(Error::PlaceError(BoardError::TileOccupied));
        }

        let card_on_board =
            CreatureOnBoard::new(card_id, self.turn_player_id(), creature.movement_points);
        let in_play_id = self
            .board
            .place(index, card_on_board)
            .map_err(Error::PlaceError)?;

        self.update_attack_values(card_registry)?;
        Ok(in_play_id)
    }

    pub fn place_trap(
        &mut self,
        card_id: CardID,
        trap: &Trap,
        index: I16Vec2,
        card_registry: &CardRegistry,
    ) -> Result<(), Error> {
        println!("Placing trap {:?} at index {:?}", trap.name(), index);

        // Ensure the tile is empty
        if self.board.get_card_at_index(&index).is_some() {
            return Err(Error::PlaceError(BoardError::TileOccupied));
        }

        let card_on_board = CreatureOnBoard::new(card_id, self.turn_player_id(), 0);
        self.board
            .place(index, card_on_board)
            .map_err(Error::PlaceError)?;

        self.update_attack_values(card_registry)?;
        Ok(())
    }

    fn validate_card_play(
        &self,
        player_id: PlayerID,
        card_index: usize,
        card_registry: &CardRegistry,
    ) -> Result<(CardID, u16), Error> {
        let player = self.get_player(player_id).ok_or(Error::PlayerNotFound)?;

        let card_id = player
            .get_card_in_hand(card_index)
            .ok_or(Error::InvalidHandPosition(card_index))?;

        let card = card_registry.get(&card_id).ok_or(Error::CardNotFound)?;
        let cost = card.cost();

        if player.get_gold() <= cost.into() {
            return Err(Error::InsufficientGold);
        }

        Ok((card_id, cost))
    }

    pub(crate) fn execute_creature_placement(
        &mut self,
        player_id: PlayerID,
        card_index: usize,
        position: I16Vec2,
        card_registry: &CardRegistry,
    ) -> Result<CardID, Error> {
        let (card_id, cost) = self.validate_card_play(player_id, card_index, card_registry)?;

        let Card::Creature(creature) = card_registry.get(&card_id).ok_or(Error::CardNotFound)?
        else {
            return Err(Error::InvalidCardType);
        };
        self.place_creature(card_id, creature, position, card_registry)?;

        let player = self
            .get_player_mut(player_id)
            .ok_or(Error::PlayerNotFound)?;
        player.remove_card_from_hand(card_index);
        player.remove_gold(cost);
        Ok(card_id)
    }

    pub(crate) fn cast_spell_from_hand(
        &mut self,
        player_id: PlayerID,
        card_index: usize,
        card_registry: &CardRegistry,
    ) -> Result<CardID, Error> {
        let (card_id, cost) = self.validate_card_play(player_id, card_index, card_registry)?;

        // Deduct cost and remove card?
        let player = self
            .get_player_mut(player_id)
            .ok_or(Error::PlayerNotFound)?;
        player.remove_card_from_hand(card_index);
        player.remove_gold(cost);

        Ok(card_id)
    }

    pub(crate) fn execute_trap_placement(
        &mut self,
        player_id: PlayerID,
        card_index: usize,
        position: I16Vec2,
        card_registry: &CardRegistry,
    ) -> Result<CardID, Error> {
        let (card_id, cost) = self.validate_card_play(player_id, card_index, card_registry)?;

        let Card::Trap(trap) = card_registry.get(&card_id).ok_or(Error::CardNotFound)? else {
            return Err(Error::InvalidCardType);
        };

        self.place_trap(card_id, trap, position, card_registry)?;

        // Deduct cost and remove card
        let player = self
            .get_player_mut(player_id)
            .ok_or(Error::PlayerNotFound)?;

        player.remove_card_from_hand(card_index);
        player.remove_gold(cost);

        Ok(card_id)
    }

    pub(crate) fn get_board_mut(&mut self) -> &mut Board {
        &mut self.board
    }
}

impl GameContext {
    pub fn new(card_registry: &CardRegistry) -> Self {
        let players = [
            Player::new(PlayerID::new(0), DeckBuilder::standard_deck(card_registry)),
            Player::new(PlayerID::new(1), DeckBuilder::standard_deck(card_registry)),
        ];

        Self {
            players,
            turn_player: PlayerID::new(1),
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

    pub fn add_gold(&mut self, player_id: PlayerID, amount: u16) -> Result<(), Error> {
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

    pub fn is_on_player_side(&self, pos: I16Vec2, player_id: PlayerID) -> bool {
        let board_width = self.board.width();
        if player_id.get() == 0 {
            pos.x < board_width / 4
        } else {
            pos.x >= board_width - board_width / 4
        }
    }

    pub fn get_turn_player(&self) -> Option<&Player> {
        self.players.get(self.turn_player_id().index())
    }

    pub fn process_turn_end(
        &mut self,
        scheduler: &mut ActionManager,
        card_registry: &CardRegistry,
    ) -> Result<(), Error> {
        debug!(
            "Processing turn {:?} beginning ",
            scheduler.get_turn_count()
        );
        scheduler.advance_to_phase(Phase::End);

        scheduler.process_actions(self, card_registry)?;
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

    pub(crate) fn advance_turn(&mut self, scheduler: &mut ActionManager) {
        self.change_turn_player();
        scheduler.advance_turn();
    }

    pub fn process_turn_begin(
        &mut self,
        scheduler: &mut ActionManager,
        card_registry: &CardRegistry,
    ) -> Result<(), Error> {
        self.draw_cards(self.turn_player_id(), NUM_CARDS_AT_START)?;

        self.board
            .refresh_movement_points(self.turn_player_id(), card_registry)?;
        self.board.update_effects(self.turn_player);
        scheduler.process_actions(self, card_registry)?;
        Ok(())
    }

    pub(crate) fn process_main_phase(&self, scheduler: &mut ActionManager) -> Result<(), Error> {
        scheduler.advance_to_phase(Phase::Main);
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
    pub fn is_legal_move(&self, from: &I16Vec2, to: &I16Vec2, card: &Creature) -> bool {
        card.movement.contains(&(*from - *to))
    }

    pub fn move_card(
        &mut self,
        from: &I16Vec2,
        to: &I16Vec2,
        card_registry: &CardRegistry,
    ) -> Result<(), Error> {
        let card_at_start = self
            .board
            .get_card_at_index(from)
            .ok_or(Error::PlaceError(BoardError::TileEmpty(*from)))?;

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
        let mut removed_cards = self.board.update_attack_values(card_registry)?;

        while !removed_cards.is_empty() {
            removed_cards = self.board.update_attack_values(card_registry)?;
        }
        Ok(())
    }

    pub(crate) fn get_turn_player_mut(&mut self) -> Option<&mut Player> {
        self.players.get_mut(self.turn_player.index())
    }
}
