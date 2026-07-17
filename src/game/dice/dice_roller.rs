use super::dice_face::DiceFace;


pub(crate) struct DiceRoller{
    max_rerolls: u8,

    current_state: Vec<(DiceFace, bool)>
}


impl DiceRoller{
    fn new(max_rerolls: u8, dices: Vec<DiceFace>) -> DiceRoller{
        let initial_state: Vec<(DiceFace, bool)> = dices.into_iter().map(|dice| {(dice, true)}).collect();
        DiceRoller { max_rerolls: max_rerolls, current_state: initial_state }
    }

    
    fn throw() {

    }
}