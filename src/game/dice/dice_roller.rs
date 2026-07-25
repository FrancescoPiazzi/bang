use std::collections::{HashMap, HashSet};

use super::die::StatedDie;
use super::die_face::DiceFace;
use super::die_state::DieState;

pub(crate) struct DiceRoller {
    dice: Vec<StatedDie>,
    n_dice: usize,
    max_rerolls: u16,
}


pub(crate) struct DiceRollResult (pub(crate) HashMap<DiceFace, u16>);

impl DiceRoller {
    fn new(dice: Vec<HashSet<DiceFace>>, max_rerolls: u16) -> DiceRoller {
        DiceRoller {
            n_dice: dice.len(),
            max_rerolls: max_rerolls,
            dice: dice.into_iter().map(|faces| StatedDie{faces: faces, state: None}).collect(),
        }
    }

    fn throw_first(&mut self) -> DiceRollResult {
        // TODO: it may be convinient to initialize every possible DiceFace to 0
        // https://stackoverflow.com/questions/21371534/in-rust-is-there-a-way-to-iterate-through-the-values-of-an-enum
        
        let mut result: DiceRollResult = DiceRollResult::new();

        for die in self.dice.iter_mut() {
            let roll_result = DiceFace::Arrow; // TODO rand.pick(current_dice_state[i].diceType)

            die.state = Some(DieState::new(roll_result));

            result.0.entry(roll_result).and_modify(|count| *count+=1).or_insert(1);
        }

        return result;
    }

    fn throw_again(&mut self) -> DiceRollResult{

        let mut result: DiceRollResult = DiceRollResult::new();

        for die in self.dice.iter_mut() {
            
            if let Some(state) = &mut die.state {
                let roll_result = if state.is_throwable() {
                    DiceFace::Arrow // TODO rand.pick(current_dice_state[i].diceType)
                } else {
                    state.get_face()
                };

                state.set_face(roll_result);

                result.0.entry(roll_result).and_modify(|count| *count+=1).or_insert(1);
            } else {
                // TODO: logger.error("Unexpected: reroll requested for a die without a state")
            }
            
        }

        return result;
    }
}

impl DiceRollResult {
    fn new() -> DiceRollResult {
        DiceRollResult(HashMap::new())
    }
}

/* 
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
*/