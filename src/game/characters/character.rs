use super::characters::*;

use crate::game::characters::base_character::CharacterData;
use crate::game::damage_type::DamageType;
use crate::game::dice::dice_roller::DiceRollResult;

use enum_dispatch::enum_dispatch;

#[derive(Clone)]
pub(crate) enum CharacterType {
    SuzieLafette,
}

/**
 * Implemented by every character that can be played
 */
#[enum_dispatch]
pub(crate) trait PlayableCharacter {
    // no idea what I was thinking when I put this here, not that it necessarily doesn't make sense, I just have no idea why
    fn build_dice_roller(&mut self);

    // consider mostly resolving this in the controller, with each PlayableCharacter only exposing methods that differentiate
    // behaviour between eachother
    fn handle_dice_roll(&mut self, roll: &DiceRollResult);

    fn take_damage(&mut self, amount: u16, _damage_type: DamageType);

    fn heal(&mut self, amount: u16);

    fn start_turn(&mut self);

    fn end_turn(&mut self);

    // hopefully this can be moved in the impl Archetype block
    // fn get_target_options(&self) -> Vec<Box<&PlayableCharacter>>;

    fn give_arrows(&mut self, amount: u16);

    fn activate_arrows(&mut self);

    fn is_alive(&self) -> bool;
}

#[enum_dispatch(PlayableCharacter)]
pub(crate) enum Archetype {
    SuzieLafette,
}

impl Archetype {
    pub(crate) fn new(character_type: CharacterType) -> Box<dyn PlayableCharacter> {
        match character_type {
            CharacterType::SuzieLafette => Box::new(SuzieLafette {
                base_character: CharacterData::new(8),
            }),
        }
    }
}
