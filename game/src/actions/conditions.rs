use bevy::ecs::{component::Component, entity::Entity};
use bevy::log::warn;
use serde::{Deserialize, Serialize};

use super::{
    targeting::{
        CreatureTarget, IsTargetSelectMode, MultiTarget, Or, PlayerTarget, SingleTarget,
        TargetSelector, TileTarget,
    },
    value_source::ValueEvalParams,
};

use crate::{actions::value_source::ValueSource, board::effect::EffectType};

#[derive(Component, Debug, Clone)]
pub enum Condition {
    /// Always true
    Always,

    /// Always false
    Never,

    /// Check if a value comparison is true
    Compare {
        left: ValueSource,
        op: CompareOp,
        right: ValueSource,
    },

    /// Check if a tile has an effect
    HasEffect {
        selector: TargetSelector<TileTarget, SingleTarget>,
        effect: EffectType,
    },

    PlayerCondition(PlayerCondition),
    CreatureCondition(CreatureCondition),

    /// Logical operations
    And(Box<Condition>, Box<Condition>),
    Or(Box<Condition>, Box<Condition>),
    Not(Box<Condition>),
}

impl Condition {
    pub fn eval(&self, params: &mut ValueEvalParams, caster: Entity) -> bool {
        match self {
            Condition::Always => true,
            Condition::Never => false,
            Condition::Compare { left, op, right } => {
                let l = left.eval(params, caster);
                let r = right.eval(params, caster);
                match op {
                    CompareOp::Equal => l == r,
                    CompareOp::NotEqual => l != r,
                    CompareOp::Greater => l > r,
                    CompareOp::GreaterOrEqual => l >= r,
                    CompareOp::Less => l < r,
                    CompareOp::LessOrEqual => l <= r,
                }
            }
            Condition::HasEffect { selector, effect } => {
                let tiles = selector.selection.find_suitable(params, caster);
                let Some(&tile_entity) = tiles.first() else {
                    return false;
                };
                let Ok(tile) = params.tiles.get(tile_entity) else {
                    return false;
                };
                tile.children.iter().any(|&child| {
                    params
                        .effects
                        .get(child)
                        .is_ok_and(|tile_effect| *tile_effect == *effect)
                })
            }
            Condition::PlayerCondition(pc) => eval_player_condition(pc, params, caster),
            Condition::CreatureCondition(cc) => eval_creature_condition(cc, params, caster),
            Condition::And(a, b) => a.eval(params, caster) && b.eval(params, caster),
            Condition::Or(a, b) => a.eval(params, caster) || b.eval(params, caster),
            Condition::Not(c) => !c.eval(params, caster),
        }
    }
}

#[derive(Debug, Clone)]
pub enum PlayerCondition {
    /// Check player resources
    HasMinGold {
        player: TargetSelector<PlayerTarget, SingleTarget>,
        amount: u16,
    },
    HasMaxGold {
        player: TargetSelector<PlayerTarget, SingleTarget>,
        amount: u16,
    },

    HasMinHealt {
        player: TargetSelector<PlayerTarget, SingleTarget>,
        amount: u16,
    },
    HasMaxHealth {
        player: TargetSelector<PlayerTarget, SingleTarget>,
        amount: u16,
    },

    /// Check deck/hand state
    DeckHasCards {
        player: TargetSelector<PlayerTarget, SingleTarget>,
        count: u16,
    },
    SelectorHasCount {
        selector: TargetSelector<PlayerTarget, Or<SingleTarget, MultiTarget>>,
        count: u16,
    },
}

fn eval_player_condition(
    condition: &PlayerCondition,
    params: &mut ValueEvalParams,
    caster: Entity,
) -> bool {
    match condition {
        PlayerCondition::HasMinGold { player, amount } => {
            let players = player.selection.find_suitable(params, caster);
            let Some(&p) = players.first() else {
                return false;
            };
            let Ok(player) = params.player.get(p) else {
                return false;
            };
            player.resources.gold >= *amount
        }
        PlayerCondition::HasMaxGold { player, amount } => {
            let players = player.selection.find_suitable(params, caster);
            let Some(&p) = players.first() else {
                return false;
            };
            let Ok(player) = params.player.get(p) else {
                return false;
            };
            player.resources.gold <= *amount
        }
        PlayerCondition::HasMinHealt { player, amount } => {
            let players = player.selection.find_suitable(params, caster);
            let Some(&p) = players.first() else {
                return false;
            };
            let Ok(player) = params.player.get(p) else {
                return false;
            };
            player.resources.health >= *amount
        }
        PlayerCondition::HasMaxHealth { player, amount } => {
            let players = player.selection.find_suitable(params, caster);
            let Some(&p) = players.first() else {
                return false;
            };
            let Ok(player) = params.player.get(p) else {
                return false;
            };
            player.resources.health <= *amount
        }
        PlayerCondition::DeckHasCards { .. } => {
            warn!("DeckHasCards condition not yet implemented; treating as false");
            false
        }
        PlayerCondition::SelectorHasCount { selector, count } => {
            selector.selection.find_suitable(params, caster).len() as u16 >= *count
        }
    }
}

#[derive(Debug, Clone)]
pub enum CreatureCondition {
    NotMoved {
        creature: TargetSelector<CreatureTarget, Or<SingleTarget, MultiTarget>>,
    },
    FullHealth {
        creature: TargetSelector<CreatureTarget, Or<SingleTarget, MultiTarget>>,
    },
    SelectorHasCount {
        selector: TargetSelector<CreatureTarget, Or<SingleTarget, MultiTarget>>,
        count: u16,
    },
}

fn eval_creature_condition(
    condition: &CreatureCondition,
    params: &mut ValueEvalParams,
    caster: Entity,
) -> bool {
    match condition {
        CreatureCondition::NotMoved { .. } => {
            warn!("CreatureCondition::NotMoved not yet implemented; treating as false");
            false
        }
        CreatureCondition::FullHealth { creature } => {
            let creatures = creature.selection.find_suitable(params, caster);
            let Some(&c) = creatures.first() else {
                return false;
            };
            let Ok(creature) = params.creatures.get(c) else {
                return false;
            };
            // Health is the max-HP component; full health means current HP has not dropped.
            creature.current_defense.0 >= creature.health.value()
        }
        CreatureCondition::SelectorHasCount { selector, count } => {
            selector.selection.find_suitable(params, caster).len() as u16 >= *count
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CompareOp {
    Equal,
    NotEqual,
    Greater,
    GreaterOrEqual,
    Less,
    LessOrEqual,
}
