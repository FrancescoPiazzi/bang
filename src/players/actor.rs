use std::collections::HashMap;

use crate::game::action::ActionType;
use crate::game::characters::character::PlayableCharacter;
use crate::game::dice::dice_roller::DiceRollResult;
use crate::game::dice::die_face::DieFace;

// represents a person or bot playing the game.
// an actor defines the decision making process of a player during a match.
pub(crate) trait Actor<'a> {
    fn receive_match_state(&mut self, me: &Box<dyn PlayableCharacter>, everyone: &Vec<&Box<dyn PlayableCharacter>>);

    fn update_dice_locks(&mut self, dice_roll: &DiceRollResult) -> HashMap<DieFace, usize>;

    fn choose_target(
        &mut self,
        targets: &Vec<&'a Box<dyn PlayableCharacter>>,
        action_type: &ActionType,
    ) -> &'a Box<dyn PlayableCharacter>;
}
