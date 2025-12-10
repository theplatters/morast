use std::collections::HashMap;

use crate::game::{
    actions::{action::Action, action_context::ActionContext},
    card::{card_registry::CardRegistry, in_play_id::InPlayID, Card},
    error::Error,
    events::event::Event,
    turn_controller::TurnController,
};

pub struct EventManager {
    event_listeners: HashMap<Event, InPlayID>,
}

impl EventManager {
    pub async fn process_event(
        &self,
        event: Event,
        turn_controller: &mut TurnController,
        card_registry: &CardRegistry,
    ) -> Result<Vec<Action>, Error> {
        match event {
            Event::SpellPlayed { owner, card_id } => {
                let Some(Card::Spell(card)) = card_registry.get(&card_id) else {
                    return Err(Error::CardNotFound);
                };

                if card.on_play_action().requires_selection() {
                    turn_controller.request_action_context(card.on_play_action());
                    Ok(vec![])
                } else {
                    let base_context = ActionContext::new().with_player(owner);
                    Ok(vec![card
                        .on_play_action()
                        .clone()
                        .finalize(&base_context)
                        .map_err(Error::ActionBuilderError)?])
                }
            }
            Event::CreaturePlayed {
                owner,
                card_id,
                position,
            } => {
                let Card::Creature(card) =
                    card_registry.get(&card_id).ok_or(Error::CardNotFound)?
                else {
                    return Err(Error::CardNotFound);
                };

                match card.on_play_action() {
                    Some(action) if action.requires_selection() => {
                        turn_controller.request_action_context(action);
                        Ok(vec![])
                    }
                    Some(action) => {
                        let base_context = ActionContext::new()
                            .with_player(owner)
                            .with_caster_position(position);
                        Ok(vec![action
                            .clone()
                            .finalize(&base_context)
                            .map_err(Error::ActionBuilderError)?])
                    }
                    None => Ok(Vec::new()),
                }
            }
            Event::TrapPlaced {
                owner,
                card_id,
                position,
            } => {
                let Card::Trap(card) = card_registry.get(&card_id).ok_or(Error::CardNotFound)?
                else {
                    return Err(Error::CardNotFound);
                };
                match card.on_play_action() {
                    Some(action) => {
                        turn_controller.request_action_context(action);
                        Ok(vec![])
                    }
                    None => Ok(Vec::new()),
                }
            }
            Event::TurnEnd => todo!(),
            Event::EffectAdded { effect } => Ok(Vec::new()),
            Event::GoldAdded { player_id, amount } => todo!(),
            Event::CardsDrawn { player_id, count } => todo!(),
        }
    }

    pub(crate) fn new() -> Self {
        Self {
            event_listeners: HashMap::new(),
        }
    }
}

impl Default for EventManager {
    fn default() -> Self {
        Self::new()
    }
}
