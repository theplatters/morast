use crate::game::{
    card::{
        abilities::Abilities, card_reader::CommonData, creature::Creature, spell_card::Spell,
        trap_card::Trap, Card,
    },
    error::Error,
    game_action::GameAction,
};
use macroquad::math::I16Vec2;

pub struct CardBuilder {
    name: Option<String>,
    cost: Option<u16>,
    description: Option<String>,
    // Creature-specific fields
    movement: Option<Vec<I16Vec2>>,
    movement_points: Option<u16>,
    attack: Option<Vec<I16Vec2>>,
    attack_strength: Option<u16>,
    defense: Option<u16>,
    abilities: Option<Vec<Abilities>>,
    // Action fields (shared between card types)
    place_action: Option<Vec<GameAction>>,
    turn_begin_action: Option<Vec<GameAction>>,
    turn_end_action: Option<Vec<GameAction>>,
    draw_action: Option<Vec<GameAction>>,
    discard_action: Option<Vec<GameAction>>,
    // Spell-specific fields
    on_play_action: Option<Vec<GameAction>>,
    // Trap-specific fields
    reveal_action: Option<Vec<GameAction>>,
    display_image_asset_string: Option<String>,
}

impl CardBuilder {
    pub fn new() -> Self {
        Self {
            name: None,
            cost: None,
            description: None,
            movement: None,
            movement_points: None,
            attack: None,
            attack_strength: None,
            defense: None,
            abilities: None,
            place_action: None,
            turn_begin_action: None,
            turn_end_action: None,
            draw_action: None,
            discard_action: None,
            on_play_action: None,
            reveal_action: None,
            display_image_asset_string: None,
        }
    }

    // Common fields
    pub fn name<S: Into<String>>(mut self, name: S) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn display_image_asset_string<S: Into<String>>(
        mut self,
        display_image_asset_string: S,
    ) -> Self {
        self.display_image_asset_string = Some(display_image_asset_string.into());
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

    // Action methods
    pub fn place_action(mut self, actions: Vec<GameAction>) -> Self {
        self.place_action = Some(actions);
        self
    }

    pub fn add_place_action(mut self, action: GameAction) -> Self {
        self.place_action.get_or_insert_with(Vec::new).push(action);
        self
    }

    pub fn turn_begin_action(mut self, actions: Vec<GameAction>) -> Self {
        self.turn_begin_action = Some(actions);
        self
    }

    pub fn add_turn_begin_action(mut self, action: GameAction) -> Self {
        self.turn_begin_action
            .get_or_insert_with(Vec::new)
            .push(action);
        self
    }

    pub fn turn_end_action(mut self, actions: Vec<GameAction>) -> Self {
        self.turn_end_action = Some(actions);
        self
    }

    pub fn add_turn_end_action(mut self, action: GameAction) -> Self {
        self.turn_end_action
            .get_or_insert_with(Vec::new)
            .push(action);
        self
    }

    pub fn draw_action(mut self, actions: Vec<GameAction>) -> Self {
        self.draw_action = Some(actions);
        self
    }

    pub fn add_draw_action(mut self, action: GameAction) -> Self {
        self.draw_action.get_or_insert_with(Vec::new).push(action);
        self
    }

    pub fn discard_action(mut self, actions: Vec<GameAction>) -> Self {
        self.discard_action = Some(actions);
        self
    }

    pub fn add_discard_action(mut self, action: GameAction) -> Self {
        self.discard_action
            .get_or_insert_with(Vec::new)
            .push(action);
        self
    }

    // Spell-specific methods
    pub fn on_play_action(mut self, actions: Vec<GameAction>) -> Self {
        self.on_play_action = Some(actions);
        self
    }

    pub fn add_on_play_action(mut self, action: GameAction) -> Self {
        self.on_play_action
            .get_or_insert_with(Vec::new)
            .push(action);
        self
    }

    // Trap-specific methods
    pub fn reveal_action(mut self, actions: Vec<GameAction>) -> Self {
        self.reveal_action = Some(actions);
        self
    }

    pub fn add_reveal_action(mut self, action: GameAction) -> Self {
        self.reveal_action.get_or_insert_with(Vec::new).push(action);
        self
    }

    // Build methods for each card type
    pub fn build_creature(self) -> Result<Card, Error> {
        let creature = Creature::new(
            self.name.ok_or(Error::Incomplete("Name is required"))?,
            self.movement.unwrap_or_default(),
            self.movement_points.unwrap_or(1),
            self.attack.unwrap_or_default(),
            self.attack_strength.unwrap_or(1),
            self.defense.unwrap_or(1),
            self.cost.unwrap_or(1),
            self.on_play_action.unwrap_or_default(),
            self.turn_begin_action.unwrap_or_default(),
            self.turn_end_action.unwrap_or_default(),
            self.draw_action.unwrap_or_default(),
            self.discard_action.unwrap_or_default(),
            self.abilities.unwrap_or_default(),
            self.description.unwrap_or("".to_string()),
            self.display_image_asset_string
                .unwrap_or("missing".to_string()),
        );
        Ok(Card::Creature(creature))
    }

    pub fn build_spell(self) -> Result<Card, Error> {
        let spell = Spell::new(
            self.name.ok_or(Error::Incomplete("Name is required"))?,
            self.description
                .ok_or(Error::Incomplete("Description is required"))?,
            self.cost.ok_or(Error::Incomplete("Cost is required"))?,
            self.on_play_action.unwrap_or_default(),
            self.display_image_asset_string
                .unwrap_or("missing".to_string()),
        );
        Ok(Card::Spell(spell))
    }

    pub fn build_trap(self) -> Result<Card, Error> {
        let trap = Trap::new(
            self.name.ok_or(Error::Incomplete("Name is required"))?,
            self.cost
                .ok_or(Error::Incomplete("Description is required"))?,
            self.description
                .ok_or(Error::Incomplete("Description is required"))?,
            self.place_action.unwrap_or_default(),
            self.reveal_action.unwrap_or_default(),
            self.display_image_asset_string
                .unwrap_or("missing".to_string()),
        );
        Ok(Card::Trap(trap))
    }

    pub(crate) fn common_data(mut self, common_data: CommonData) -> Self {
        self.name = Some(common_data.name);
        self.cost = Some(common_data.cost);
        self.description = Some(common_data.description);
        self.display_image_asset_string = Some(common_data.display_image_asset_string);
        self
    }
}

impl Default for CardBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// Add builder method to Card enum
impl Card {
    pub fn builder() -> CardBuilder {
        CardBuilder::new()
    }
}
