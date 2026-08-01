use std::collections::{HashMap, HashSet};

use std::iter::Filter;
use std::slice::Iter;

use super::die::StatedDie;
use super::die_face::DieFace;
use super::die_state::DieState;

pub(crate) struct DiceRoller {
    dice: Vec<StatedDie>,
}

impl DiceRoller {
    pub(crate) fn new(dice: Vec<HashSet<DieFace>>) -> DiceRoller {
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

    pub(crate) fn throw(&mut self, dice_values_iter: &mut impl Iterator<Item = DieFace>) -> DiceRollResult {
        let mut result: DiceRollResult = DiceRollResult::new();

        for die in self.dice.iter_mut() {
            // TODO: if a die isn't throwable there is no need to sample the iterator, NOTE: changing this breaks 
            // some tests as it changes wich dice are picked
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
                let final_face = die.state.unwrap().get_face();
                result
                    .0
                    .entry(final_face)
                    .and_modify(|count| *count += 1)
                    .or_insert(1);
            } else {
                // TODO: logger.error("Unexpected: dice value iterator did not yeld any value")
            }
        }

        println!("throw result: {:?}", result);

        return result;
    }

    /* locks a certain amount of faces for a die */
    pub(crate) fn lock_dice_amount(&mut self, face: DieFace, amount: usize) -> Result<usize, usize> {
        self.lock_unlock_amount(face, Some(amount), true)
    }

    /* unlocks a certain amount of faces for a die */
    pub(crate) fn unlock_dice_amount(&mut self, face: DieFace, amount: usize) -> Result<usize, usize> {
        self.lock_unlock_amount(face, Some(amount), false)
    }

    /* locks all dice with a given face */
    pub(crate) fn lock_dice(&mut self, face: DieFace) -> Result<usize, usize> {
        self.lock_unlock(face, true)
    }

    /* unlocks all dice with a given face */
    pub(crate) fn unlock_dice(&mut self, face: DieFace) -> Result<usize, usize> {
        self.lock_unlock(face, false)
    }

    fn lock_unlock(&mut self, face: DieFace, lock: bool) -> Result<usize, usize> {
        self.lock_unlock_amount(face, None, lock)
    }

    fn lock_unlock_amount(&mut self, face: DieFace, amount: Option<usize>, lock: bool) -> Result<usize, usize> {
        let mut matching_dice: Vec<&mut StatedDie> = self.get_matching_dice_mut(face, lock);
        let actual_amount = amount.unwrap_or(matching_dice.len());

        // println!("dice matching {:?}: {:?}", face, matching_dice);

        if matching_dice.len() >= actual_amount {
            matching_dice
                .iter_mut()
                .take(actual_amount)
                .for_each(|die|
                    if let Some(ref mut state) = die.state {
                        state.set_locked_by_player(lock);
                    } else {
                        panic!("Unexpected: die does not have a state while trying to update its lock status");
                    }
                );

            // println!("matching_dice after lock/unlock: {:?}", cln.iter().map(|die| die.state).collect::<Vec<Option<DieState>>>());
            println!("dice after lock/unlock: {:?}", self.dice.iter().map(|die| die.state).collect::<Vec<Option<DieState>>>());
            
            Ok(actual_amount)
        } else {
            Err(matching_dice.len())
        }
    }


    fn get_matching_dice_mut(&mut self, face: DieFace, lock: bool) -> Vec<&mut StatedDie> {
        self.dice
            .iter_mut()
            .filter(move |die| 
                die.state.is_some_and(|state| 
                    state.get_face() == face && (lock ^ state.get_locked_by_player())
                )
            )
            .collect()
    }


    // TODO overload the above with something that locks all die of a type
}


#[derive(Debug)]
pub(crate) struct DiceRollResult(HashMap<DieFace, u16>);

impl DiceRollResult {
    pub(crate) fn new() -> DiceRollResult {
        DiceRollResult(HashMap::new())
    }

    pub(crate) fn from(hash_map: HashMap<DieFace, u16>) -> DiceRollResult {
        DiceRollResult(hash_map)
    }

    pub(crate) fn get(&self, face: &DieFace) -> u16 {
        *self.0.get(face).unwrap_or(&0)
    }
}

#[cfg(test)]
mod tests {
    use std::array;

    use crate::game::dice::die_face::DieFace::*;

    use super::*;

    static N_DICE_PER_THROW: usize = 5;
    static STANDARD_DIE_FACES: [DieFace; 6] = [Shoot1, Shoot2, Beer, Arrow, Gatling, Dynamite];
    static YELDED_FACES: [DieFace; 10] = [
        Dynamite, Shoot1, Dynamite, Arrow, Shoot1, 
        Shoot1, Beer, Arrow, Arrow, Dynamite,
    ];

    struct DieFaceGenerator {
        count: usize,
        looped_values: [DieFace; 10],
    }

    impl DieFaceGenerator {
        fn new() -> DieFaceGenerator {
            DieFaceGenerator {
                count: 0,
                looped_values: YELDED_FACES,
            }
        }
    }

    impl Iterator for DieFaceGenerator {
        type Item = DieFace;

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
        let dice: [HashSet<DieFace>; N_DICE_PER_THROW] = array::from_fn(|_| HashSet::from(STANDARD_DIE_FACES).clone());
        let mut dice_roller = DiceRoller::new(dice.into());

        let mut generator = DieFaceGenerator::new();

        let res = dice_roller.throw(&mut generator);
        println!("{:?}", res);
        assert_eq!(res.get(&Shoot1), 2);
        assert_eq!(res.get(&Shoot2), 0);
        assert_eq!(res.get(&Beer), 0);
        assert_eq!(res.get(&Arrow), 1);
        assert_eq!(res.get(&Gatling), 0);
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

    #[test]
    fn test_lock_unlock() {
        let dice: [HashSet<DieFace>; N_DICE_PER_THROW] = array::from_fn(|_| HashSet::from(STANDARD_DIE_FACES).clone());
        let mut dice_roller = DiceRoller::new(dice.into());

        let mut generator = DieFaceGenerator::new();
    
        let _ = dice_roller.throw(&mut generator);

        assert_eq!(dice_roller.lock_dice_amount(Shoot1, 3), Err(2));
        assert_eq!(dice_roller.lock_dice_amount(Shoot1, 1), Ok(1));
        assert_eq!(dice_roller.lock_dice(Dynamite), Ok(2));

        let res = dice_roller.throw(&mut generator);

        assert_eq!(res.get(&Shoot1), 1);
        assert_eq!(res.get(&Shoot2), 0);
        assert_eq!(res.get(&Beer), 0);
        assert_eq!(res.get(&Arrow), 1);
        assert_eq!(res.get(&Gatling), 0);
        assert_eq!(res.get(&Dynamite), 3);

        assert_eq!(dice_roller.unlock_dice(Shoot1), Ok(1));
        assert_eq!(dice_roller.unlock_dice(Dynamite), Ok(2));   // 3 dynamites, but only 2 of them were locked

        let res = dice_roller.throw(&mut generator);

        assert_eq!(res.get(&Shoot1), 1);
        assert_eq!(res.get(&Shoot2), 0);
        assert_eq!(res.get(&Beer), 0);
        assert_eq!(res.get(&Arrow), 1);
        assert_eq!(res.get(&Gatling), 0);
        assert_eq!(res.get(&Dynamite), 3);
    }
}
