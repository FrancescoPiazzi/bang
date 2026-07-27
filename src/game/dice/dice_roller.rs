use std::collections::{HashMap, HashSet};

use super::die::StatedDie;
use super::die_face::DiceFace;
use super::die_state::DieState;

pub(crate) struct DiceRoller {
    dice: Vec<StatedDie>,
}

impl DiceRoller {
    pub(crate) fn new(dice: Vec<HashSet<DiceFace>>) -> DiceRoller {
        DiceRoller {
            dice: dice
                .into_iter()
                .map(|faces| StatedDie {
                    faces: faces,
                    state: None,
                })
                .collect(),
        }
    }

    fn throw(&mut self, dice_values_iter: &mut impl Iterator<Item = DiceFace>) -> DiceRollResult {
        let mut result: DiceRollResult = DiceRollResult::new();

        for die in self.dice.iter_mut() {
            // TODO: if a die isn't throwable there is no need to sample the iterator, NOTE: this breaks the second part of test_dice_throw()
            // as it changes wich dice are picked
            if let Some(roll_result) = dice_values_iter.next() {
                if let Some(state) = &mut die.state {
                    // die has a state -> it has already been thrown
                    if state.is_throwable() {
                        state.set_face(roll_result);
                    }
                } else {
                    // stateless die, never thrown, initialize it
                    die.state = Some(DieState::new(roll_result));
                }
                // update result object witht the last die throw
                result
                    .0
                    .entry(die.state.unwrap().get_face())
                    .and_modify(|count| *count += 1)
                    .or_insert(1);
            } else {
                // TODO: logger.error("Unexpected: dice value iterator did not yeld any value")
            }
        }

        return result;
    }

    // TODO: method to lock a dice based on player decision
}


#[derive(Debug)]
pub(crate) struct DiceRollResult(HashMap<DiceFace, u16>);

impl DiceRollResult {
    pub(crate) fn new() -> DiceRollResult {
        DiceRollResult(HashMap::new())
    }

    pub(crate) fn from(hash_map: HashMap<DiceFace, u16>) -> DiceRollResult {
        DiceRollResult(hash_map)
    }

    pub(crate) fn get(&self, face: &DiceFace) -> u16 {
        *self.0.get(face).unwrap_or(&0)
    }
}

#[cfg(test)]
mod tests {
    use std::array;

    use crate::game::dice::die_face::DiceFace::*;

    use super::*;

    static N_DICE_PER_THROW: usize = 5;
    static STANDARD_DIE_FACES: [DiceFace; 6] = [Shoot1, Shoot2, Beer, Arrow, Gatling, Dynamite];
    static YELDED_FACES: [DiceFace; 10] = [
        Dynamite, Shoot1, Dynamite, Arrow, Gatling, Shoot1, Beer, Arrow, Arrow, Dynamite,
    ];

    struct DiceFaceGenerator {
        count: usize,
        looped_values: [DiceFace; 10],
    }

    impl DiceFaceGenerator {
        fn new() -> DiceFaceGenerator {
            DiceFaceGenerator {
                count: 0,
                looped_values: YELDED_FACES,
            }
        }
    }

    impl Iterator for DiceFaceGenerator {
        type Item = DiceFace;

        fn next(&mut self) -> Option<Self::Item> {
            let nxt = self.looped_values[self.count];
            self.count = if self.count + 1 < self.looped_values.len() {
                self.count + 1
            } else {
                0
            };
            Some(nxt)
        }
    }

    #[test]
    fn test_dice_throw() {
        let dice: [HashSet<DiceFace>; N_DICE_PER_THROW] = array::from_fn(|_| HashSet::from(STANDARD_DIE_FACES).clone());
        let mut dice_roller = DiceRoller::new(dice.into());

        let mut generator = DiceFaceGenerator::new();

        let res = dice_roller.throw(&mut generator);
        println!("{:?}", res);
        assert_eq!(res.get(&Shoot1), 1);
        assert_eq!(res.get(&Shoot2), 0);
        assert_eq!(res.get(&Beer), 0);
        assert_eq!(res.get(&Arrow), 1);
        assert_eq!(res.get(&Gatling), 1);
        assert_eq!(res.get(&Dynamite), 2);

        let res = dice_roller.throw(&mut generator);
        // expect 3 dynamite because it is not rethrown, IMPORTANT: this test result changes if you
        // optimize and not read the iterator if the item is not throwable
        assert_eq!(res.get(&Shoot1), 0);
        assert_eq!(res.get(&Shoot2), 0);
        assert_eq!(res.get(&Beer), 1);
        assert_eq!(res.get(&Arrow), 1);
        assert_eq!(res.get(&Gatling), 0);
        assert_eq!(res.get(&Dynamite), 3);
    }
}
