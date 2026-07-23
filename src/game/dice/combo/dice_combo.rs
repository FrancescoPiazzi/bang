use std::collections::HashSet;

use super::super::dice_face::DiceFace;
use super::super::dice_roller::DiceRollResult;

pub(crate) trait DiceCombo {
    fn is_triggered(&self, result: &DiceRollResult) -> bool;
}

struct SimpleDiceCombo {
    dice_face: DiceFace,
    n_dices: usize,
}

struct GenericDiceCombo(HashSet<(DiceFace, usize)>);

impl DiceCombo for SimpleDiceCombo {
    fn is_triggered(&self, result: &DiceRollResult) -> bool {
        result.0.iter().filter(|face| **face == self.dice_face).count() >= self.n_dices
    }
}

impl DiceCombo for GenericDiceCombo {
    fn is_triggered(&self, result: &DiceRollResult) -> bool {
        todo!()
    }
}
