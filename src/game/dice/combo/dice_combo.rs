use std::collections::HashSet;

use super::super::dice_face::DiceFace;
use super::super::dice_roller::DiceRollResult;

pub(crate) trait DiceCombo {
    fn is_triggered(result: DiceRollResult) -> bool;
}

struct SimpleDiceCombo {
    dice_face: DiceFace,
    n_dices: u16,
}

struct GenericDiceCombo(HashSet<(DiceFace, u16)>);

impl DiceCombo for SimpleDiceCombo {
    fn is_triggered(result: DiceRollResult) -> bool {
        todo!()
    }
}

impl DiceCombo for GenericDiceCombo {
    fn is_triggered(result: DiceRollResult) -> bool {
        todo!()
    }
}
