use crate::game::action::{ActionType, DiceLockUpdate};
use crate::game::characters::character::PlayableCharacter;
use crate::game::dice::dice_roller::DiceRollResult;

// represents a person or bot playing the game.
// an actor defines the decision making process of a player during a match.
pub(crate) trait Actor<'a> {
    // update sent by the controller, does not require an answer
    fn receive_match_state(&mut self, me: &Box<dyn PlayableCharacter>, everyone: &Vec<&Box<dyn PlayableCharacter>>);

    // allows the actor to decide whether to throw the dice again, if true is returned, get_dice_locks will be called
    // immediately after to allow the actor to lock or unlock dice before the throw is performed
    fn throw_dice_again(&mut self, dice_roll: &DiceRollResult) -> bool;

    // allows the actor to lock/ulock dice given a result for the next throw
    fn get_dice_locks(&mut self, dice_roll: &DiceRollResult) -> DiceLockUpdate;

    // allows the bot to choose the target for a specific action, mainly following dice throw
    // but may also come from abilities, note that if more actions are required (i.e. 3 shots and 2 beers are rolled)
    // this method is called once per action, in the order in which they must be resolved
    // TODO: add the last dice throw result so bots can make wiser targeting decisions without having to store
    // the last dice throw result themselves
    fn choose_target(
        &mut self,
        targets: &Vec<&'a Box<dyn PlayableCharacter>>,
        action_type: &ActionType,
    ) -> &'a Box<dyn PlayableCharacter>;
}
