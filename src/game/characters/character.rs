use std::collections::HashMap;

use super::characters::*;

use crate::game::action::ActionRange;
use crate::game::action::ActionType;
use crate::game::action::NonReferencingInteraction;
use crate::game::characters::base_character::CharacterData;
use crate::game::damage_type::DamageType;
use crate::game::dice::combo::dice_combo::DiceCombo;
use crate::game::dice::dice_roller::DiceRollResult;
use crate::game::dice::die_face::DieFace;

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
    // TODO: some of these methods don't need self at all, figure out if it makes sense to remove it
    // and if yes, how, since you can't keep a method that doesn't take &self in an #[enum_dispatch] trait

    // allows characters to throw different dices, no character in the original rules has this ability
    // however, it makes it easy to implements some characters, while also keeping room open for characters
    // to naturally throw different dices
    fn get_dice_faces(&self) -> Vec<DieFace> {
        Archetype::get_standard_dice_faces()
    }

    fn get_dice_combos(&self) -> Vec<DiceCombo> {
        Archetype::get_standard_dice_combos()
    }

    fn get_max_rerolls(&self, settings_max_rerolls: u16) -> u16 {
        settings_max_rerolls
    }

    // consider mostly resolving this in the controller, with each PlayableCharacter only exposing methods that differentiate
    // behaviour between eachother
    fn handle_dice_roll(&mut self, roll: &DiceRollResult);

    fn take_damage(&mut self, amount: u16, _damage_type: DamageType);

    fn heal(&mut self, amount: u16);

    fn start_turn(&mut self);

    fn end_turn(&mut self);

    fn give_arrows(&mut self, amount: u16);

    fn activate_arrows(&mut self);

    fn is_alive(&self) -> bool;
}

#[enum_dispatch(PlayableCharacter)]
pub(crate) enum Archetype {
    SuzieLafette,
}

impl<'ch> Archetype {
    pub(crate) fn new(character_type: CharacterType) -> Box<dyn PlayableCharacter> {
        match character_type {
            CharacterType::SuzieLafette => Box::new(SuzieLafette {
                base_character: CharacterData::new(8),
            }),
        }
    }

    pub(crate) fn get_standard_dice_faces() -> Vec<DieFace> {
        Vec::from([
            DieFace::Shoot1,
            DieFace::Shoot2,
            DieFace::Arrow,
            DieFace::Dynamite,
            DieFace::Beer,
            DieFace::Gatling,
        ])
    }

    pub(crate) fn get_standard_dice_combos() -> Vec<DiceCombo> {
        vec![
            DiceCombo::new(
                [(DieFace::Dynamite, 3)].into_iter().collect(),
                vec![
                    NonReferencingInteraction::new(ActionType::Damage(DamageType::Dynamite, 1), ActionRange::Myself)
                ]
            ), 
            DiceCombo::new(
                [(DieFace::Gatling, 3)].into_iter().collect(),
                vec![
                    NonReferencingInteraction::new(ActionType::DiscardAllArrows, ActionRange::Myself),
                    NonReferencingInteraction::new(ActionType::Damage(DamageType::Gatling, 1), ActionRange::AnyoneButSelf)
                ]
            )
        ]
    }
}
