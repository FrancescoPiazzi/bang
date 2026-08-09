use crate::game::dice::dice_roller::DiceRollResult;
use crate::game::{characters::base_character::CharacterData, damage_type::DamageType};

use super::character::PlayableCharacter;

pub(crate) struct SuzieLafette {
    pub(crate) base_character: CharacterData,
}

impl PlayableCharacter for SuzieLafette {
    fn build_dice_roller(&mut self) {}

    fn handle_dice_roll(&mut self, roll: &DiceRollResult) {}

    fn take_damage(&mut self, amount: u16, damage_type: DamageType) {
        self.base_character.take_damage(amount, damage_type);
    }

    fn heal(&mut self, amount: u16) {
        self.base_character.heal(amount);
    }

    fn start_turn(&mut self) {}

    fn end_turn(&mut self) {
        // if base_character.dice_roller has no Shoot1, Shoot2, or DoubleShot then heal
    }

    /*fn get_target_options(&self) -> {

    }*/

    fn give_arrows(&mut self, amount: u16) {
        self.base_character.give_arrows(amount);
    }

    fn activate_arrows(&mut self) {
        self.base_character.activate_arrows();
    }

    fn is_alive(&self) -> bool {
        self.base_character.is_alive()
    }
}
