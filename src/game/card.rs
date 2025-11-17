use crate::game::card::creature::Creature;
use crate::game::card::in_play_id::InPlayID;
use crate::game::card::spell_card::Spell;
use crate::game::card::trap_card::Trap;
use crate::game::events::event_scheduler::GameScheduler;
use crate::game::player::PlayerID;

pub mod abilities;
pub mod card_id;
pub mod card_reader;
pub mod card_registry;
pub mod creature;
pub mod in_play_id;
pub mod spell_card;
pub mod trap_card;

#[derive(Debug)]
pub enum Card {
    Creature(Creature),
    Spell(Spell),
    Trap(Trap),
}

pub trait Named {
    fn name(&self) -> &str;
}

pub trait Placeable {
    fn on_place(&self, scheduler: &mut GameScheduler, owner: PlayerID, id: InPlayID);
}

impl Named for Card {
    fn name(&self) -> &str {
        match self {
            Card::Creature(c) => c.name(),
            Card::Spell(c) => c.name(),
            Card::Trap(c) => c.name(),
        }
    }
}
