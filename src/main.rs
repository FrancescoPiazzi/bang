mod game;

use crate::game::characters::character::Character;
use crate::game::characters::characters::SuzieLafette;

fn main() {
    let mut character = SuzieLafette::new(9);

    character.give_arrows(10000);
    character.activate_arrows();

    if !character.is_alive() {
        println!("yay she's fucking dead");
    }
}
