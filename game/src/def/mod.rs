//! Data-level intermediate representation (IR) for card definitions.
//!
//! `CardDef` and friends are plain serde-deserializable data loaded from RON
//! files in `assets/cards`. They are converted into the runtime ECS model
//! (`ValueSource`, `Condition`, `AnyTargetSelector`, ...) by [`convert`].
pub mod card;
pub mod condition;
pub mod convert;
pub mod effect;
pub mod loader;
pub mod selector;
pub mod trigger;
pub mod value;
pub mod value_expr;

#[cfg(test)]
mod card_tests;

#[cfg(test)]
mod ron_roundtrip_tests {
    use super::{
        card::{CardDef, CardKindDef, CreatureStatsDef, PatternDef},
        condition::{ConditionDef, CreatureConditionDef, PlayerConditionDef},
        effect::{ChoiceOptionDef, EffectDef, StatModifierDef},
        selector::{CardinalityDef, FilterDef, SelectionDef, SelectorDef, SelectorKindDef},
        trigger::{AbilityDef, TriggerDef},
        value::ValueDef,
    };
    use crate::{
        actions::{conditions::CompareOp, value_source::StatType},
        board::effect::EffectType,
        card::abilities::Abilities,
    };

    fn roundtrip<T>(value: &T)
    where
        T: serde::Serialize + for<'de> serde::Deserialize<'de> + PartialEq + std::fmt::Debug,
    {
        let s = ron::to_string(value).expect("serialize");
        let back: T = ron::de::from_str(&s).expect("deserialize");
        assert_eq!(&back, value, "roundtrip mismatch; ron was:\n{}", s);
    }

    fn sample_selector() -> SelectorDef {
        SelectorDef {
            kind: SelectorKindDef::Creature,
            cardinality: CardinalityDef::Multi,
            selection: SelectionDef::AllEnemy,
            filters: vec![FilterDef::DamagedOnly, FilterDef::MinAttack(ValueDef::Constant(2))],
        }
    }

    #[test]
    fn value_def_roundtrip() {
        roundtrip(&ValueDef::Constant(7));
        roundtrip(&ValueDef::Expr("attack(caster) * 2 + 1".into()));
        roundtrip(&ValueDef::Add(
            Box::new(ValueDef::Constant(1)),
            Box::new(ValueDef::Random {
                min: Box::new(ValueDef::Constant(1)),
                max: Box::new(ValueDef::Constant(6)),
            }),
        ));
        roundtrip(&ValueDef::Count(Box::new(sample_selector())));
        roundtrip(&ValueDef::CreatureStat {
            selector: Box::new(sample_selector()),
            stat: StatType::MaxHealth,
        });
        roundtrip(&ValueDef::Min(
            Box::new(ValueDef::Divide(
                Box::new(ValueDef::Constant(4)),
                Box::new(ValueDef::Constant(2)),
            )),
            Box::new(ValueDef::Max(
                Box::new(ValueDef::Constant(1)),
                Box::new(ValueDef::Constant(3)),
            )),
        ));
    }

    #[test]
    fn selector_def_roundtrip() {
        roundtrip(&sample_selector());
        roundtrip(&SelectorDef {
            kind: SelectorKindDef::Tile,
            cardinality: CardinalityDef::Single,
            selection: SelectionDef::ChooseArea {
                radius: ValueDef::Constant(1),
            },
            filters: vec![FilterDef::EmptyOnly],
        });
        roundtrip(&SelectorDef {
            kind: SelectorKindDef::Player,
            cardinality: CardinalityDef::Any,
            selection: SelectionDef::Owner,
            filters: vec![FilterDef::MinGold(ValueDef::Constant(3))],
        });
        roundtrip(&SelectorDef {
            kind: SelectorKindDef::Hand,
            cardinality: CardinalityDef::Multi,
            selection: SelectionDef::AllCards,
            filters: vec![FilterDef::ExcludeSpells, FilterDef::MaxCost(ValueDef::Constant(2))],
        });
    }

    #[test]
    fn condition_def_roundtrip() {
        roundtrip(&ConditionDef::Always);
        roundtrip(&ConditionDef::Compare {
            left: ValueDef::Count(Box::new(sample_selector())),
            op: CompareOp::GreaterOrEqual,
            right: ValueDef::Constant(3),
        });
        roundtrip(&ConditionDef::HasEffect {
            selector: sample_selector(),
            effect: EffectType::Weakening,
        });
        roundtrip(&ConditionDef::Player(PlayerConditionDef::HasMinGold {
            player: sample_selector(),
            amount: 4,
        }));
        roundtrip(&ConditionDef::Creature(CreatureConditionDef::NotMoved {
            creature: sample_selector(),
        }));
        roundtrip(&ConditionDef::And(
            Box::new(ConditionDef::Always),
            Box::new(ConditionDef::Not(Box::new(ConditionDef::Never))),
        ));
    }

    #[test]
    fn effect_def_roundtrip() {
        roundtrip(&EffectDef::DealDamage {
            selector: sample_selector(),
            amount: ValueDef::Expr("2".into()),
        });
        roundtrip(&EffectDef::ApplyEffect {
            selector: sample_selector(),
            effect: EffectType::Slow,
            duration: 2,
        });
        roundtrip(&EffectDef::ModifyStats {
            selector: sample_selector(),
            modifier: StatModifierDef::Both {
                attack: 1,
                health: -1,
            },
        });
        roundtrip(&EffectDef::MoveCreature {
            selector: sample_selector(),
            direction: [1, 0],
            absolute: false,
        });
        roundtrip(&EffectDef::If {
            condition: ConditionDef::Always,
            then: vec![EffectDef::DestroyCreature {
                selector: sample_selector(),
            }],
            otherwise: vec![EffectDef::DrawCards {
                player: sample_selector(),
                amount: ValueDef::Constant(1),
            }],
        });
        roundtrip(&EffectDef::Choose {
            options: vec![
                ChoiceOptionDef {
                    label: "Enrage".into(),
                    effects: vec![EffectDef::ModifyStats {
                        selector: sample_selector(),
                        modifier: StatModifierDef::Attack(2),
                    }],
                },
                ChoiceOptionDef {
                    label: "Fortify".into(),
                    effects: vec![EffectDef::Heal {
                        selector: sample_selector(),
                        amount: ValueDef::Constant(2),
                    }],
                },
            ],
        });
    }

    #[test]
    fn ability_def_roundtrip() {
        roundtrip(&AbilityDef {
            trigger: TriggerDef::OnTurnEnd,
            condition: ConditionDef::Always,
            speed: Default::default(),
            timing: Default::default(),
            effects: vec![EffectDef::AddGold {
                player: sample_selector(),
                amount: ValueDef::Constant(4),
            }],
        });
        roundtrip(&TriggerDef::OnPlay);
        roundtrip(&TriggerDef::OnReveal);
    }

    #[test]
    fn card_def_roundtrip() {
        roundtrip(&CardDef {
            name: "soldier".into(),
            cost: 1,
            description: "A soldier".into(),
            display_image: "missing".into(),
            kind: CardKindDef::Creature(CreatureStatsDef {
                attack: 3,
                defense: 3,
                movement_points: 3,
                movement: PatternDef::Plus(1),
                attack_pattern: PatternDef::Union(vec![PatternDef::Cross(1), PatternDef::Plus(2)]),
                abilities: vec![Abilities::Flying, Abilities::Digging],
            }),
            abilities: vec![],
        });
        roundtrip(&CardDef {
            name: "wind".into(),
            cost: 2,
            description: "Blows away a card".into(),
            display_image: "missing".into(),
            kind: CardKindDef::Spell,
            abilities: vec![AbilityDef {
                trigger: TriggerDef::OnPlay,
                condition: ConditionDef::Always,
                speed: Default::default(),
                timing: Default::default(),
                effects: vec![EffectDef::MoveCreature {
                    selector: sample_selector(),
                    direction: [1, 0],
                    absolute: false,
                }],
            }],
        });
        roundtrip(&CardKindDef::Trap);
        roundtrip(&PatternDef::Offsets(vec![[5, 0], [-2, 3]]));
    }
}
