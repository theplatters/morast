use macroquad::math::I16Vec2;

use crate::game::phases::Phase;

use super::{
    board::{card_on_board::CardOnBoard, Board},
    card::{
        card_id::CardID,
        card_registry::{CardRegistry},
    },
    error::Error,
    events::event_scheduler::GameScheduler,
    player::{Player, PlayerID},
};

const NUM_CARDS_AT_START: u16 = 2;
pub struct GameContext {
    players: [Player; 2],
    board: Board,
    turn_player: PlayerID,
    cards_placed: Vec<CardOnBoard>,
    pub current_selected_card: Option<CardOnBoard>,
}

impl GameContext {
    pub fn new(players: [Player; 2]) -> Self {
        Self {
            players,
            board: Board::new(),
            turn_player: PlayerID::new(1),
            cards_placed: Vec::new(),
            current_selected_card: None,
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
    ) -> Result<(), Error> {
        match self.board.place(card_id, player_id, index, card_registry) {
            Ok(_) => {
                self.cards_placed.push(CardOnBoard::new(card_id, player_id));
                Ok(())
            }
            Err(err) => Err(Error::PlaceError(err)),
        }
    }

    pub fn proces_turn_end(&mut self, scheduler: &mut GameScheduler, card_registry: &CardRegistry) {
        println!(
            "Processing turn {:?} beginning ",
            scheduler.get_turn_count()
        );
        scheduler.advance_to_phase(Phase::End, self);
        for card in &self.cards_placed.clone() {
            println!("Processing card {:?}", card);
            self.current_selected_card = Some(*card);
            card_registry
                .get(&card.card_id)
                .expect("Card not found")
                .on_turn_start(self, scheduler);
        }

        scheduler.process_events(self);
    }

    pub fn proces_turn_begin(
        &mut self,
        scheduler: &mut GameScheduler,
        card_registry: &CardRegistry,
    ) {
        println!(
            "Processing turn {:?} beginning ",
            scheduler.get_turn_count()
        );
        self.change_turn_player();
        scheduler.advance_turn(self);
        self.draw_cards(self.turn_player_id(), NUM_CARDS_AT_START);
        for card in &self.cards_placed.clone() {
            println!("Processing card {:?}", card);
            self.current_selected_card = Some(*card);
            card_registry
                .get(&card.card_id)
                .expect("Card not found")
                .on_turn_end(self, scheduler);
        }

        scheduler.process_events(self);
    }
}

impl crate::engine::janet_handler::types::janetenum::ToVoidPointer for GameContext {}
