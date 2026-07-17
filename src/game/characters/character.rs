use crate::game::damage_type::DamageType;

pub(crate) trait Character{
    fn new(max_hp: u16) -> impl Character;

    fn build_dice_roller();

    fn take_damage(&mut self, amount: u16, _damage_type: DamageType);


    fn heal(&mut self, amount: u16);


    fn start_turn();


    fn end_turn();


    fn get_target_options();


    fn give_arrows(&mut self, amount: u16);


    fn activate_arrows(&mut self);


    fn is_alive(&self) -> bool;
}