use crate::game::damage_type::DamageType;
use crate::game::dice::dice_roller::DiceRollResult;

/**
 * Methods characters recieve calls on from the engine.
 */
pub(crate) trait Character {
    fn new(max_hp: u16) -> impl Character;

    fn build_dice_roller();

    fn handle_dice_roll(roll: &DiceRollResult);

    fn take_damage(&mut self, amount: u16, _damage_type: DamageType);

    fn heal(&mut self, amount: u16);

    fn start_turn();

    fn end_turn();

    fn get_target_options();

    fn give_arrows(&mut self, amount: u16);

    fn activate_arrows(&mut self);

    fn is_alive(&self) -> bool;
}
