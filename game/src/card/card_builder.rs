use bevy::math::I16Vec2;

use crate::{
    actions::GameAction,
    card::{
        Card, abilities::Abilities, card_reader::CommonData, creature::Creature, spell_card::Spell,
        trap_card::Trap,
    },
    error::GameError,
};

// Common fields shared by all card types
pub struct CardBuilder {
    name: Option<String>,
    cost: Option<u16>,
    description: Option<String>,
    display_image_asset_string: Option<String>,
}

// Type-specific builders that contain the common builder
pub struct CreatureBuilder {
    common: CardBuilder,
    movement: Option<Vec<I16Vec2>>,
    movement_points: Option<u16>,
    attack: Option<Vec<I16Vec2>>,
    attack_strength: Option<u16>,
    defense: Option<u16>,
    abilities: Option<Vec<Abilities>>,
    on_play_action: Option<GameAction>,
    turn_begin_action: Option<GameAction>,
    turn_end_action: Option<GameAction>,
    draw_action: Option<GameAction>,
    discard_action: Option<GameAction>,
}

pub struct SpellBuilder {
    common: CardBuilder,
    on_play_action: Option<GameAction>,
}

pub struct TrapBuilder {
    common: CardBuilder,
    on_play_action: Option<GameAction>,
    reveal_action: Option<GameAction>,
}

impl CardBuilder {
    pub fn new() -> Self {
        Self {
            name: None,
            cost: None,
            description: None,
            display_image_asset_string: None,
        }
    }

    pub fn creature(self) -> CreatureBuilder {
        CreatureBuilder {
            common: self,
            movement: None,
            movement_points: None,
            attack: None,
            attack_strength: None,
            defense: None,
            abilities: None,
            on_play_action: None,
            turn_begin_action: None,
            turn_end_action: None,
            draw_action: None,
            discard_action: None,
        }
    }

    pub fn spell(self) -> SpellBuilder {
        SpellBuilder {
            common: self,
            on_play_action: None,
        }
    }

    pub fn trap(self) -> TrapBuilder {
        TrapBuilder {
            common: self,
            on_play_action: None,
            reveal_action: None,
        }
    }

    // Common field methods
    pub fn name<S: Into<String>>(mut self, name: S) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn cost(mut self, cost: u16) -> Self {
        self.cost = Some(cost);
        self
    }

    pub fn description<S: Into<String>>(mut self, description: S) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn display_image_asset_string<S: Into<String>>(mut self, asset: S) -> Self {
        self.display_image_asset_string = Some(asset.into());
        self
    }

    pub fn common_data(mut self, common_data: CommonData) -> Self {
        self.name = Some(common_data.name);
        self.cost = Some(common_data.cost);
        self.description = Some(common_data.description);
        self.display_image_asset_string = Some(common_data.display_image_asset_string);
        self
    }
}

impl CreatureBuilder {
    // Delegate common methods to the inner builder
    pub fn name<S: Into<String>>(mut self, name: S) -> Self {
        self.common = self.common.name(name);
        self
    }

    pub fn cost(mut self, cost: u16) -> Self {
        self.common = self.common.cost(cost);
        self
    }

    pub fn description<S: Into<String>>(mut self, description: S) -> Self {
        self.common = self.common.description(description);
        self
    }

    pub fn display_image_asset_string<S: Into<String>>(mut self, asset: S) -> Self {
        self.common = self.common.display_image_asset_string(asset);
        self
    }

    pub fn common_data(mut self, common_data: CommonData) -> Self {
        self.common = self.common.common_data(common_data);
        self
    }

    // Creature-specific methods
    pub fn movement(mut self, movement: Vec<I16Vec2>) -> Self {
        self.movement = Some(movement);
        self
    }

    pub fn movement_points(mut self, movement_points: u16) -> Self {
        self.movement_points = Some(movement_points);
        self
    }

    pub fn attack_pattern(mut self, attack: Vec<I16Vec2>) -> Self {
        self.attack = Some(attack);
        self
    }

    pub fn attack_strength(mut self, attack_strength: u16) -> Self {
        self.attack_strength = Some(attack_strength);
        self
    }

    pub fn defense(mut self, defense: u16) -> Self {
        self.defense = Some(defense);
        self
    }

    pub fn abilities(mut self, abilities: Vec<Abilities>) -> Self {
        self.abilities = Some(abilities);
        self
    }

    pub fn add_ability(mut self, ability: Abilities) -> Self {
        self.abilities.get_or_insert_with(Vec::new).push(ability);
        self
    }

    // Optional on_play_action for creatures
    pub fn on_play_action_option(mut self, action: Option<GameAction>) -> Self {
        self.on_play_action = action;
        self
    }

    pub fn turn_begin_action_option(mut self, action: Option<GameAction>) -> Self {
        self.turn_begin_action = action;
        self
    }

    pub fn turn_end_action_option(mut self, action: Option<GameAction>) -> Self {
        self.turn_end_action = action;
        self
    }

    pub fn draw_action_option(mut self, action: Option<GameAction>) -> Self {
        self.draw_action = action;
        self
    }

    pub fn discard_action_option(mut self, action: Option<GameAction>) -> Self {
        self.discard_action = action;
        self
    }

    // Optional on_play_action for creatures
    pub fn on_play_action(mut self, action: GameAction) -> Self {
        self.on_play_action = Some(action);
        self
    }

    pub fn turn_begin_action(mut self, action: GameAction) -> Self {
        self.turn_begin_action = Some(action);
        self
    }

    pub fn turn_end_action(mut self, action: GameAction) -> Self {
        self.turn_end_action = Some(action);
        self
    }

    pub fn draw_action(mut self, action: GameAction) -> Self {
        self.draw_action = Some(action);
        self
    }

    pub fn discard_action(mut self, action: GameAction) -> Self {
        self.discard_action = Some(action);
        self
    }

    pub fn build(self) -> Result<Card, GameError> {
        let creature = Creature::new(
            self.common
                .name
                .ok_or(GameError::Incomplete("Name is required"))?,
            self.movement.unwrap_or_default(),
            self.movement_points.unwrap_or(1),
            self.attack.unwrap_or_default(),
            self.attack_strength.unwrap_or(1),
            self.defense.unwrap_or(1),
            self.common.cost.unwrap_or(1),
            self.on_play_action,
            self.turn_begin_action,
            self.turn_end_action,
            self.draw_action,
            self.discard_action,
            self.abilities.unwrap_or_default(),
            self.common.description.unwrap_or("".to_string()),
            self.common
                .display_image_asset_string
                .unwrap_or("missing".to_string()),
        );
        Ok(Card::Creature(creature))
    }
}

impl SpellBuilder {
    // Delegate common methods
    pub fn name<S: Into<String>>(mut self, name: S) -> Self {
        self.common = self.common.name(name);
        self
    }

    pub fn cost(mut self, cost: u16) -> Self {
        self.common = self.common.cost(cost);
        self
    }

    pub fn description<S: Into<String>>(mut self, description: S) -> Self {
        self.common = self.common.description(description);
        self
    }

    pub fn display_image_asset_string<S: Into<String>>(mut self, asset: S) -> Self {
        self.common = self.common.display_image_asset_string(asset);
        self
    }

    pub fn common_data(mut self, common_data: CommonData) -> Self {
        self.common = self.common.common_data(common_data);
        self
    }

    // Required on_play_action for spells
    pub fn on_play_action(mut self, action: GameAction) -> Self {
        self.on_play_action = Some(action);
        self
    }

    pub fn build(self) -> Result<Card, GameError> {
        let spell = Spell::new(
            self.common
                .name
                .ok_or(GameError::Incomplete("Name is required"))?,
            self.common
                .description
                .ok_or(GameError::Incomplete("Description is required"))?,
            self.common
                .cost
                .ok_or(GameError::Incomplete("Cost is required"))?,
            self.on_play_action.ok_or(GameError::Incomplete(
                "On play action is required for spells",
            ))?,
            self.common
                .display_image_asset_string
                .unwrap_or("missing".to_string()),
        );
        Ok(Card::Spell(spell))
    }
}

impl TrapBuilder {
    // Delegate common methods
    pub fn name<S: Into<String>>(mut self, name: S) -> Self {
        self.common = self.common.name(name);
        self
    }

    pub fn cost(mut self, cost: u16) -> Self {
        self.common = self.common.cost(cost);
        self
    }

    pub fn description<S: Into<String>>(mut self, description: S) -> Self {
        self.common = self.common.description(description);
        self
    }

    pub fn display_image_asset_string<S: Into<String>>(mut self, asset: S) -> Self {
        self.common = self.common.display_image_asset_string(asset);
        self
    }

    pub fn common_data(mut self, common_data: CommonData) -> Self {
        self.common = self.common.common_data(common_data);
        self
    }

    // Optional on_play_action for traps
    pub fn on_play_action(mut self, action: Option<GameAction>) -> Self {
        self.on_play_action = action;
        self
    }

    pub fn reveal_action(mut self, action: GameAction) -> Self {
        self.reveal_action = Some(action);
        self
    }

    pub fn reveal_action_optional(mut self, action: Option<GameAction>) -> Self {
        self.reveal_action = action;
        self
    }

    pub fn build(self) -> Result<Card, GameError> {
        let trap = Trap::new(
            self.common
                .name
                .ok_or(GameError::Incomplete("Name is required"))?,
            self.common
                .cost
                .ok_or(GameError::Incomplete("Cost is required"))?,
            self.common
                .description
                .ok_or(GameError::Incomplete("Description is required"))?,
            self.on_play_action,
            self.reveal_action,
            self.common
                .display_image_asset_string
                .unwrap_or("missing".to_string()),
        );
        Ok(Card::Trap(trap))
    }
}

impl Default for CardBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl Card {
    pub fn builder() -> CardBuilder {
        CardBuilder::new()
    }
}
