use std::cmp::{min, max};

use crate::game::damage_type::DamageType;
use crate::game::dice::dice_roller::DiceRoller;


pub(crate) struct BaseCharacter{
    alive: bool,

    max_hp: u16,
    hp: u16,
    arrows: u16,

    dice_roller: Option<DiceRoller>,
}



impl BaseCharacter{
    pub(crate) fn new(max_hp: u16) -> BaseCharacter {
        BaseCharacter{max_hp: max_hp, hp: max_hp, alive: true, arrows: 0, dice_roller: Option::None}
    }


    pub(crate) fn build_dice_roller(){
        
    }


    pub(crate) fn take_damage(&mut self, amount: u16, _damage_type: DamageType) -> bool{
        self.hp = max(self.hp as i16 - amount as i16, 0) as u16;
        self.alive = self.hp > 0;

        self.alive
    }


    pub(crate)fn heal(&mut self, amount: u16){
        self.hp = min(self.hp+amount, self.max_hp);
    }


    pub(crate) fn start_turn(){
        
    }


    pub(crate) fn end_turn(){

    }


    pub(crate) fn get_target_options(){

    }


    pub(crate) fn give_arrows(&mut self, amount: u16){
        self.arrows += amount;
    }


    pub(crate) fn activate_arrows(&mut self){
        self.take_damage(self.arrows, DamageType::ARROW);
        self.arrows = 0;
    }


    pub(crate) fn is_alive(&self) -> bool {
        self.alive
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_damage() {
        let mut ch = BaseCharacter::new(9);
        ch.take_damage(1, DamageType::TEST);
        assert_eq!(ch.hp, 8);
        assert_eq!(ch.max_hp, 9);
        assert!(ch.alive);

        ch.take_damage(8, DamageType::TEST);
        assert_eq!(ch.hp, 0);
        assert_eq!(ch.max_hp, 9);
        assert!(!ch.alive);
    }

    #[test]
    fn test_arrows() {
        let mut ch = BaseCharacter::new(9);
        ch.give_arrows(1);
        ch.activate_arrows();
        assert_eq!(ch.arrows, 0);
        assert_eq!(ch.hp, 8);
        ch.give_arrows(8);
        ch.activate_arrows();
        assert_eq!(ch.hp, 0);
        assert_eq!(ch.max_hp, 9);
        assert!(!ch.alive);
    }

    #[test]
    fn test_heal() {
        let mut ch = BaseCharacter::new(9);
        ch.heal(1);
        assert_eq!(ch.hp, 9);   // overheal
        ch.take_damage(5, DamageType::TEST);
        ch.heal(2);
        assert_eq!(ch.hp, 9-5+2);
    }
}