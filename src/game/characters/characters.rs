use crate::game::dice::dice_roller::DiceRollResult;
use crate::game::{characters::base_character::BaseCharacter, damage_type::DamageType};

use super::character::Character;

pub(crate) struct SuzieLafette {
    base_character: BaseCharacter,
}

impl Character for SuzieLafette {
    fn new(max_hp: u16) -> impl Character {
        SuzieLafette {
            base_character: BaseCharacter::new(max_hp),
        }
    }

    fn build_dice_roller() {
        todo!()
    }

    fn handle_dice_roll(roll: &DiceRollResult) {
        todo!()
    }

    fn take_damage(&mut self, amount: u16, damage_type: DamageType) {
        self.base_character.take_damage(amount, damage_type);
    }

    fn heal(&mut self, amount: u16) {
        self.base_character.heal(amount);
    }

    fn start_turn() {}

    fn end_turn() {
        // if base_character.dice_roller has no Shoot1, Shoot2, or DoubleShot then heal
    }

    fn get_target_options() {
        todo!()
    }

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
