use super::{
    board::Board,
    card::card_registry::CardRegistry,
    events::event_manager::EventManager,
    phases::Phase,
    player::{Player, PlayerID},
};

pub struct GameContext {
    pub players: [Player; 2],
    pub board: Board,
    pub game_state: Phase,
    turn_player: PlayerID,
    pub card_registry: CardRegistry,
}

impl GameContext {
    pub fn new(players: [Player; 2]) -> Self {
        Self {
            players,
            board: Board::new(),
            game_state: Phase::Start,
            turn_player: PlayerID::new(0),
            card_registry: CardRegistry::new(),
        }
    }

    pub fn turn_player_id(&self) -> PlayerID {
        self.turn_player
    }

    pub fn get_player(&mut self, id: PlayerID) -> &mut Player {
        self.players
            .iter_mut()
            .find(|p| p.id == id)
            .expect("Player not found")
    }

    pub fn other_player_id(&self) -> PlayerID {
        if self.players[0].id == self.turn_player {
            self.turn_player
        } else {
            self.players[1].id
        }
    }

    pub fn draw_cards(&mut self, player_id: PlayerID, num_cards: u16) {
        let player = self.get_player(player_id);

        for _ in 0..num_cards {
            if let Some(card) = player.draw_from_deck() {
                player.add_to_hand(card);
            }
        }
    }

    pub fn discard_cards(&mut self, player_id: PlayerID, num_cards: u16) {
        let player = self.get_player(player_id);

        for _ in 0..num_cards {
            player.discard_card()
        }
    }

    pub fn get_gold(&mut self, player_id: PlayerID, amount: i64) {
        let player = self.get_player(player_id);

        player.get_gold(amount as i32);
    }
}

impl crate::engine::janet_handler::types::janetenum::ToVoidPointer for GameContext {}
