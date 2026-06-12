mod game;


use crate::game::characters::base_character::BaseCharacter;


// use crate::game::characters::BaseCharacter;


fn main() {
    let mut character = BaseCharacter::new(9);

    character.give_arrows(10000);
    character.activate_arrows();

    if !character.is_alive() {
        println!("yay he's fucking dead");
    }
}
