use std::ops::Index;
use std::ops::IndexMut;

use super::dice_face::DiceFace;
use super::dice_state::DiceState;

pub(crate) struct DiceRoller {
    n_dice: usize,
    max_rerolls: u16,
    current_dice_state: Vec<DiceState>,
}

// TODO: if you wanna throw like a million dice for the hell of it
// a hashmap DiceFace -> u16 would be MUCH lighter
pub(crate) struct DiceRollResult (pub(crate) Vec<DiceFace>);

impl DiceRoller {
    fn new(n_dice: usize, max_rerolls: u16) -> DiceRoller {
        DiceRoller {
            n_dice: n_dice,
            max_rerolls: max_rerolls,
            current_dice_state: Vec::new(),
        }
    }

    fn throw(&self) -> DiceRollResult {
        // TODO: this can be prettier
        let mut result: DiceRollResult = DiceRollResult::new(self.n_dice);

        for i in 0..self.n_dice {
            result[i] = if self.current_dice_state[i].throwable {
                DiceFace::Arrow // TODO rand
            } else {
                self.current_dice_state[i].face
            };
        }

        return result;
    }
}

impl DiceRollResult {
    fn new(n_dice: usize) -> DiceRollResult {
        let mut faces: Vec<DiceFace> = Vec::new();
        faces.reserve(n_dice);
        DiceRollResult(faces)
    }
}

impl Index<usize> for DiceRollResult {
    type Output = DiceFace;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for DiceRollResult {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}
