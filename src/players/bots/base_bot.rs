use crate::game::action::{ActionType, DiceLockUpdate};
use crate::game::characters::character::PlayableCharacter;
use crate::game::dice::dice_roller::DiceRollResult;
use crate::players::actor::Actor;

// The simplest bot possible, doesn't care about game state, never rerolls dice, and always selects the first option available for any targetting choice
#[derive(Clone)]
pub(crate) struct BaseBot {}

impl Actor for BaseBot {
    fn receive_match_state(&mut self, _me: &Box<dyn PlayableCharacter>, _everyone: &Vec<&Box<dyn PlayableCharacter>>) {}

    fn throw_dice_again(&mut self, _dice_roll: &DiceRollResult) -> bool {
        false
    }

    fn get_dice_locks(&mut self, _dice_roll: &DiceRollResult) -> DiceLockUpdate {
        DiceLockUpdate::default()
    }

    fn choose_target(&self, _targets: &Vec<&Box<dyn PlayableCharacter>>, _action_type: &ActionType) -> usize {
        0
    }

    fn recieve_action_decision(&mut self) {}

    fn recieve_reroll_decision(&mut self) {}
}
