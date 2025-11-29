use macroquad::math::I16Vec2;

use crate::game::{
    board::effect::Effect,
    card::{card_id::CardID, card_registry::CardRegistry, in_play_id::InPlayID},
    error::Error,
    events::{action::Action, event::Event, execution_result::ExecutionResult},
    game_action::JanetAction,
    game_context::GameContext,
    player::PlayerID,
};

// Core action trait

pub trait GameAction {
    fn execute(
        &self,
        context: &mut GameContext,
        card_registry: &CardRegistry,
    ) -> Result<ExecutionResult, Error>;
    fn can_execute(&self, context: &GameContext) -> Result<(), Error>;
    fn has_targeting_type(&self) -> bool;
    fn targeting_type(&self) -> Option<TargetingType>;
    fn execute_with_targets(
        &self,
        context: &mut GameContext,
        card_registry: &CardRegistry,
        targets: &[I16Vec2],
    ) -> Result<Option<Event>, Error>;
}

pub trait GameActionWithTargets: GameAction {}

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
        target: TargetingType,
        amount: u16,
        source: InPlayID,
    },
    HealCreature {
        target: TargetingType,
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
        tiles: TargetingType,
        duration: u32,
    },
    SummonCreature {
        creature_id: CardID,
        position: I16Vec2,
        owner: PlayerID,
    },
    DestroyCreature {
        target: TargetingType,
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
    ForEachInArea {
        center: I16Vec2,
        radius: u8,
        action: Box<ActionEffect>,
    },
    ForEachCreature {
        filter: CreatureFilter,
        action: Box<ActionEffect>,
    },

    Custom {
        action: Box<JanetAction>,
        target: TargetingType,
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
    ) -> Result<ExecutionResult, Error> {
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
                Ok(ExecutionResult::Executed { event })
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
                Ok(ExecutionResult::Executed { event })
            }
            Self::EndTurn => Ok(ExecutionResult::Executed {
                event: Some(Event::TurnEnd),
            }),
            Self::WithTargets { action, targets } => {
                let event =
                    action.execute_with_targets(context, card_registry, targets.as_slice())?;
                Ok(ExecutionResult::Executed { event })
            }
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

    /// Returns true if this action effect requires targeting from the player
    fn has_targeting_type(&self) -> bool {
        match self {
            ActionEffect::DealDamage { target, .. }
            | ActionEffect::HealCreature { target, .. }
            | ActionEffect::ApplyEffect { tiles: target, .. }
            | ActionEffect::DestroyCreature { target } => target.requires_selection(),
            ActionEffect::PlaceCreature { .. }
            | ActionEffect::PlaceTrap { .. }
            | ActionEffect::CastSpell { .. }
            | ActionEffect::MoveCreature { .. }
            | ActionEffect::DrawCards { .. }
            | ActionEffect::AddGold { .. }
            | ActionEffect::SummonCreature { .. }
            | ActionEffect::WithTargets { .. } => false,
            ActionEffect::Sequence(actions) => {
                actions.iter().any(|action| action.has_targeting_type())
            }
            ActionEffect::Conditional {
                then_action,
                else_action,
                ..
            } => {
                then_action.has_targeting_type()
                    || else_action
                        .as_ref()
                        .map_or(false, |action| action.has_targeting_type())
            }
            ActionEffect::ForEachInArea { .. } => false,
            ActionEffect::ForEachCreature { .. } => false,
            ActionEffect::Custom {
                action,
                target: targeting,
            } => targeting.requires_selection(),
            ActionEffect::EndTurn => false,
        }
    }

    /// Returns the targeting type if this action requires targeting
    fn targeting_type(&self) -> Option<TargetingType> {
        if !self.has_targeting_type() {
            return None;
        }

        match self {
            ActionEffect::DealDamage { target, .. } => {
                if target.requires_selection() {
                    Some(target.clone())
                } else {
                    None
                }
            }
            ActionEffect::HealCreature { target, .. } => {
                if target.requires_selection() {
                    Some(target.clone())
                } else {
                    None
                }
            }
            ActionEffect::ApplyEffect { tiles, .. } => {
                if tiles.requires_selection() {
                    Some(tiles.clone())
                } else {
                    None
                }
            }
            ActionEffect::DestroyCreature { target } => {
                if target.requires_selection() {
                    Some(target.clone())
                } else {
                    None
                }
            }

            // For composite actions, return the first targeting type found
            // Note: This assumes only one action in a sequence requires targeting
            // You might want to handle this differently based on your game's needs
            ActionEffect::Sequence(actions) => {
                actions.iter().find_map(|action| action.targeting_type())
            }
            ActionEffect::Conditional {
                then_action,
                else_action,
                ..
            } => then_action.targeting_type().or_else(|| {
                else_action
                    .as_ref()
                    .and_then(|action| action.targeting_type())
            }),

            ActionEffect::Custom { target, .. } => {
                if target.requires_selection() {
                    Some(target.clone())
                } else {
                    None
                }
            }
,

            // These cases should never be reached due to has_targeting_type() check
            _ => None,
        }
    }
    fn execute_with_targets(
        &self,
        context: &mut GameContext,
        card_registry: &CardRegistry,
        targets: &[I16Vec2],
    ) -> Result<Option<Event>, Error> {
        match self {
            ActionEffect::DealDamage {
                target,
                amount,
                source,
            } if target.verify(targets) => {
                targets
                    .iter()
                    .for_each(|tile| context.get_board_mut().do_damage(tile, *amount));
                Ok(None)
            }
            ActionEffect::HealCreature {
                target,
                amount,
                source,
            } if target.verify(targets) => {
                targets
                    .iter()
                    .for_each(|tile| context.get_board_mut().heal_creature(tile, *amount));
                Ok(None)
            }
            ActionEffect::DestroyCreature { target } => {
                if target.verify(targets) {
                    targets
                        .iter()
                        .for_each(|tile| context.get_board_mut().destroy_card(tile));
                }
                Ok(None)
            }
            _ => Err(Error::ActionError("not implemented for ExecutionResult")),
        }
    }
}
