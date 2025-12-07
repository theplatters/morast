use std::{cmp::Ordering, ops::SubAssign};

use crate::{
    engine::janet_handler::types::janetenum::JanetEnum,
    game::{
        card::{self, creature::Creature, Card},
        error::Error,
        events::{
            action_builder::ActionPrototypeBuilder,
            action_effect::{ActionEffect, GameAction},
            event::Event,
            timing::ActionTiming,
        },
        phases::Phase,
        player::PlayerID,
    },
};

#[derive(Debug, Clone)]
pub struct Action {
    pub action: ActionEffect,
    pub timing: ActionTiming,
    pub speed: SpellSpeed,
    pub priority: u32,
    pub player: PlayerID,
    pub can_be_countered: bool,
}
impl PartialEq for Action {
    fn eq(&self, other: &Self) -> bool {
        self.timing == other.timing && self.speed == other.speed && self.priority == other.priority
    }
}

impl Eq for Action {}

impl PartialOrd for Action {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Action {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (other.timing, other.speed, other.priority).cmp(&(self.timing, self.speed, self.priority))
    }
}

impl GameAction for Action {
    fn execute(
        &self,
        context: &mut crate::game::game_context::GameContext,
        card_registry: &crate::game::card::card_registry::CardRegistry,
    ) -> Result<Option<Event>, crate::game::error::Error> {
        match &self.action {
            ActionEffect::PlaceCreature {
                card_index,
                position,
            } => {
                let card_id = context.execute_creature_placement(
                    self.player,
                    *card_index,
                    *position,
                    card_registry,
                )?;
                let event = Some(Event::CreaturePlayed {
                    card_id,
                    owner: self.player,
                });
                Ok(event)
            }
            ActionEffect::CastSpell { card_index } => {
                let card_id =
                    context.cast_spell_from_hand(self.player, *card_index, card_registry)?;
                let event = Some(Event::SpellPlayed {
                    card_id,
                    owner: self.player,
                });
                Ok(event)
            }
            ActionEffect::EndTurn => Ok(Some(Event::TurnEnd)),
            ActionEffect::PlaceTrap {
                card_index,
                position,
            } => {
                let card_id = context.execute_trap_placement(
                    self.player,
                    *card_index,
                    *position,
                    card_registry,
                )?;
                Ok(Some(Event::TrapPlaced {
                    card_id,
                    owner: self.player,
                }))
            }
            ActionEffect::MoveCreature { from, to } => {
                context.move_card(&from, &to, card_registry)?;
                Ok(None)
            }
            ActionEffect::DealDamage { target, amount } => todo!(),
            ActionEffect::HealCreature { target, amount } => todo!(),
            ActionEffect::DrawCards { player_id, count } => {
                context.draw_cards(*player_id, *count);
                Ok(Some(Event::CardsDrawn {
                    player_id: *player_id,
                    count: *count,
                }))
            }
            ActionEffect::AddGold { player_id, amount } => {
                context.add_gold(*player_id, *amount);
                Ok(Some(Event::GoldAdded {
                    player_id: *player_id,
                    amount: *amount,
                }))
            }
            ActionEffect::ApplyEffect {
                effect, targets, ..
            } => {
                context
                    .get_board_mut()
                    .add_effects(*effect, targets.as_slice());
                Ok(Some(Event::EffectAdded { effect: *effect }))
            }

            ActionEffect::SummonCreature {
                creature_id,
                position,
                owner,
            } => {
                let Some(Card::Creature(creature)) = card_registry.get(&creature_id) else {
                    return Err(Error::CardNotFound);
                };
                context.place_creature(*creature_id, creature, *position, card_registry)?;
                Ok(Some(Event::CreaturePlayed {
                    card_id: *creature_id,
                    owner: *owner,
                }))
            }
            ActionEffect::DestroyCreature { targets } => todo!(),
            ActionEffect::WithTargets { action, targets } => todo!(),
            ActionEffect::Sequence(action_effects) => todo!(),
            ActionEffect::Conditional {
                condition,
                then_action,
                else_action,
            } => todo!(),
            ActionEffect::Custom { action, target } => todo!(),
        }
    }

    fn can_execute(
        &self,
        context: &crate::game::game_context::GameContext,
    ) -> Result<(), crate::game::error::Error> {
        todo!()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum SpellSpeed {
    #[default]
    Slow = 1, // Can only be cast during main phase, when stack is empty
    Fast = 2,    // Can be cast anytime you have priority
    Instant = 3, // Can be cast anytime, even during opponent's turn
}

impl TryFrom<JanetEnum> for SpellSpeed {
    type Error = Error;

    fn try_from(value: JanetEnum) -> Result<Self, Self::Error> {
        match value {
            JanetEnum::Int(0) => Ok(SpellSpeed::Slow),
            JanetEnum::Int(1) => Ok(SpellSpeed::Fast),
            JanetEnum::Int(2) => Ok(SpellSpeed::Instant),
            JanetEnum::Int(num) => Err(Error::Cast(format!("Invalid SpellSpeed number: {}", num))),

            JanetEnum::UInt(0) => Ok(SpellSpeed::Slow),
            JanetEnum::UInt(1) => Ok(SpellSpeed::Fast),
            JanetEnum::UInt(2) => Ok(SpellSpeed::Instant),
            JanetEnum::UInt(num) => Err(Error::Cast(format!("Invalid SpellSpeed number: {}", num))),

            JanetEnum::String(s) => match s.as_str() {
                "slow " => Ok(SpellSpeed::Slow),
                "fast" => Ok(SpellSpeed::Fast),
                "instant" => Ok(SpellSpeed::Instant),
                _ => Err(Error::Cast(format!("Invalid SpellSpeed string: {}", s))),
            },
            _ => Err(Error::Cast(format!("Invalid SpellSpeed type "))),
        }
    }
}
