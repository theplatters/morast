use macroquad::math::I16Vec2;

use crate::game::{
    board::effect::Effect,
    card::{card_id::CardID, card_registry::CardRegistry, in_play_id::InPlayID},
    error::Error,
    events::{action::Action, event::Event},
    game_context::GameContext,
    janet_action::JanetAction,
    player::PlayerID,
};

// Core action trait

pub trait GameAction {
    fn execute(
        &self,
        context: &mut GameContext,
        card_registry: &CardRegistry,
    ) -> Result<Option<Event>, Error>;
    fn can_execute(&self, context: &GameContext) -> Result<(), Error>;
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum TargetingType {
    None,       // No targeting needed
    SingleTile, // Click a tile
    Tiles { amount: u8 },
    Area { radius: u8 }, // Area around clicked tile
    Line { length: u8 }, // Line from caster
    Caster,              // Targets the card itself
    AreaAroundCaster { radius: u8 },
    AllEnemies, // All enemy units
}

impl TargetingType {
    pub fn requires_selection(&self) -> bool {
        matches!(
            self,
            Self::SingleTile | Self::Tiles { .. } | Self::Area { .. } | Self::Line { .. }
        )
    }

    pub(crate) fn required_targets(&self) -> u8 {
        if let Self::Tiles { amount } = self {
            *amount
        } else if matches!(
            self,
            Self::SingleTile | Self::Area { .. } | Self::Line { .. }
        ) {
            1
        } else {
            0
        }
    }

    pub fn verify(&self, targets: &[I16Vec2]) -> bool {
        todo!()
    }
}

#[derive(Debug, Clone)]
pub enum ActionEffect {
    // Basic game actions
    PlaceCreature {
        card_index: usize,
        position: I16Vec2,
        player_id: PlayerID,
    },
    CastSpell {
        card_index: usize,
        player_id: PlayerID,
    },
    PlaceTrap {
        card_index: usize,
        position: I16Vec2,
        player_id: PlayerID,
    },

    MoveCreature {
        from: I16Vec2,
        to: I16Vec2,
        player_id: PlayerID,
    },
    EndTurn,

    // Atomic game effects (what cards actually do)
    DealDamage {
        target: Vec<I16Vec2>,
        amount: u16,
        source: InPlayID,
    },
    HealCreature {
        target: Vec<I16Vec2>,
        amount: u16,
        source: InPlayID,
    },
    DrawCards {
        player_id: PlayerID,
        count: u16,
    },
    AddGold {
        player_id: PlayerID,
        amount: i64,
    },
    ApplyEffect {
        effect: Effect,
        targets: Vec<I16Vec2>,
        duration: u32,
    },
    SummonCreature {
        creature_id: CardID,
        position: I16Vec2,
        owner: PlayerID,
    },
    DestroyCreature {
        targets: Vec<I16Vec2>,
    },

    WithTargets {
        action: Box<Action>,
        targets: Vec<I16Vec2>,
    },

    // Composite actions
    Sequence(Vec<ActionEffect>),

    Conditional {
        condition: Condition,
        then_action: Box<ActionEffect>,
        else_action: Option<Box<ActionEffect>>,
    },

    // Targeting actions
    Custom {
        action: Box<JanetAction>,
        target: Vec<I16Vec2>,
    },
}

#[derive(Debug, Clone)]
pub enum Condition {
    TileOccupied(I16Vec2),
    CreatureHasHealth { target: I16Vec2, min_health: u16 },
    PlayerHasGold { player_id: PlayerID, min_gold: i64 },
    // ... more conditions
}

#[derive(Debug, Clone)]
pub enum CreatureFilter {
    OwnedBy(PlayerID),
    WithinRange { center: I16Vec2, radius: u8 },
    HasTag(String),
    // ... more filters
}

impl GameAction for ActionEffect {
    fn execute(
        &self,
        context: &mut GameContext,
        card_registry: &CardRegistry,
    ) -> Result<Option<Event>, Error> {
        match self {
            ActionEffect::PlaceCreature {
                card_index,
                position,
                player_id,
            } => {
                let card_id = context.execute_creature_placement(
                    *player_id,
                    *card_index,
                    *position,
                    card_registry,
                )?;
                let event = Some(Event::CreaturePlayed {
                    card_id,
                    owner: *player_id,
                });
                Ok(event)
            }
            ActionEffect::CastSpell {
                card_index,
                player_id,
            } => {
                let card_id =
                    context.cast_spell_from_hand(*player_id, *card_index, card_registry)?;
                let event = Some(Event::SpellPlayed {
                    card_id,
                    owner: *player_id,
                });
                Ok(event)
            }
            Self::EndTurn => Ok(Some(Event::TurnEnd)),
            _ => Err(Error::Incomplete(
                "Wrong action type, you should use a ConcreteAction",
            )),
        }
    }

    fn can_execute(&self, context: &GameContext) -> Result<(), Error> {
        match self {
            _ => todo!(),
        }
    }
}
