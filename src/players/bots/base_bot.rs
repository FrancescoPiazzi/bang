use std::collections::HashMap;

use crate::game::action::ActionType;
use crate::game::characters::character::PlayableCharacter;
use crate::game::dice::dice_roller::DiceRollResult;
use crate::game::dice::die_face::DieFace;
use crate::players::actor::Actor;

// The simplest bot possible, doesn't care about game state, never rerolls dice, and always selects the first option available for any targetting choice
#[derive(Clone)]
pub(crate) struct BaseBot {}

impl<'a> Actor<'a> for BaseBot {
    fn receive_match_state(&mut self, me: &Box<dyn PlayableCharacter>, everyone: &Vec<&Box<dyn PlayableCharacter>>) {}

    fn update_dice_locks(&mut self, dice_roll: &DiceRollResult) -> HashMap<DieFace, usize> {
        HashMap::new()
    }

    fn choose_target(
        &mut self,
        targets: &Vec<&'a Box<dyn PlayableCharacter>>,
        action_type: &ActionType,
    ) -> &'a Box<dyn PlayableCharacter> {
        targets.get(0).unwrap()
    }
}
