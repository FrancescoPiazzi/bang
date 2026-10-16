use std::collections::HashSet;

use crate::game::utils;
use crate::game::{characters::character::PlayableCharacter, damage_type::DamageType, dice::die_face::DieFace};

// an action a player takes that doesn't have a specific target,
// since this doesn't need to hold a reference to a character,
// it doesn't pollute outside code with lifetimes, also allows for multiple targets
pub(crate) struct Action {
    pub(crate) action_type: ActionType,
    pub(crate) action_range: ActionRange,
}

impl Action {
    pub(crate) fn new(action_type: ActionType, action_range: ActionRange) -> Action {
        Action {
            action_type: action_type,
            action_range: action_range,
        }
    }
}

// the action of locking or unlocking a certain amount of dice before rethrowing them.
// used by the actors to communicate their decisions to the controller and by the controller
// to notify other actors
#[derive(Debug)]
pub(crate) struct DiceLockUpdate(Vec<DieLockUpdate>);

// update for a given die face, pass amount: None to lock or unlock all dice with the given face
#[derive(Debug)]
pub(crate) struct DieLockUpdate {
    pub(crate) die_face: DieFace,
    pub(crate) amount: Option<u16>,
    pub(crate) lock_unlock: LockUnlock,
}

#[derive(Debug)]
pub(crate) enum ActionType {
    Damage(DamageType, u16),
    Heal(u16),
    GiveArrows(u16),
    DiscardAllArrows,
}

// pub(crate) enum ActionSource<'ch> {
//     Pile,   // interactions with the pile, i.e. healing
//     Character(&'ch dyn PlayableCharacter),  // make somone else the source of an action, no idea if this is ever needed
// }

#[derive(Debug)]
pub(crate) enum ActionRange {
    Myself,
    ExactDistance(usize), // used mainly when shooting
    DistanceRange(usize, usize),
    Anyone,
    AnyoneButSelf,
}

#[derive(Debug)]
pub(crate) enum LockUnlock {
    Lock,
    Unlock,
}

impl ActionRange {
    pub(crate) fn get_targets<T>(
        characters: &Vec<T>,
        starting_point: usize,
        action_range: ActionRange,
    ) -> Vec<&T> {
        match action_range {
            ActionRange::Myself => Vec::from([&characters[starting_point]]),

            ActionRange::ExactDistance(distance_inner) => {
                // an exact distance range will always have at most 2 targets
                let ln = characters.len() as i64;
                let first: i64 = (starting_point as i64 - distance_inner as i64).rem_euclid(ln);
                let second: i64 = (starting_point as i64 + distance_inner as i64) % ln;

                if first != second {
                    Vec::from([&characters[first as usize], &characters[second as usize]])
                } else {
                    Vec::from([&characters[first as usize]])
                }
            }

            ActionRange::DistanceRange(min, mut max) => {
                if max > characters.len() {
                    max = characters.len();
                }

                (min..=max)
                    .map(|distance| Self::get_targets(characters, starting_point, ActionRange::ExactDistance(distance)))
                    .flatten()
                    .collect()
            }

            ActionRange::AnyoneButSelf => characters
                .iter()
                .enumerate()
                .filter(|(i, _)| starting_point != *i)
                .map(|(_, element)| element)
                .collect(),

            ActionRange::Anyone => characters.iter().collect(),
        }
    }

    pub(crate) fn get_target_indexes(
        characters: &Vec<Box<dyn PlayableCharacter>>,
        starting_point: usize,
        action_range: ActionRange,
    ) -> HashSet<usize> {
        match action_range {
            ActionRange::Myself => HashSet::from([starting_point]),

            ActionRange::ExactDistance(distance) => Self::get_target_indexes(
                characters,
                starting_point,
                ActionRange::DistanceRange(distance, distance),
            ),

            ActionRange::DistanceRange(min, max) => {
                let mut res = HashSet::new();

                let mut remaining_left = max - min + 1;
                let mut remaining_right = max - min + 1;

                let mut pointer_left = Self::first_alive_index(characters, starting_point as i64 - 1, false);
                let mut pointer_right = Self::first_alive_index(characters, starting_point as i64 + 1, true);

                let mut alive_counted_left: usize = 1;
                let mut alive_counted_right: usize = 1;

                // let left_greater_right = pointer_left > pointer_right;

                while remaining_left > 0 || remaining_right > 0 {
                    // TODO: I think I can exit the loop early if the pointers swap themselves compared to how they were when starting
                    // but I'm not risking it yet
                    // if (pointer_left < pointer_right) ^ left_greater_right {
                    //     break;
                    // }

                    if (min..=max).contains(&alive_counted_left) {
                        res.insert(pointer_left as usize);
                        remaining_left -= 1;
                    }
                    if (min..=max).contains(&alive_counted_right) {
                        res.insert(pointer_right as usize);
                        remaining_right -= 1;
                    }

                    alive_counted_left += 1;
                    alive_counted_right += 1;

                    pointer_left = Self::first_alive_index(characters, pointer_left - 1, false);
                    pointer_right = Self::first_alive_index(characters, pointer_right + 1, true);
                }

                res
            }

            ActionRange::AnyoneButSelf => characters
                .iter()
                .enumerate()
                .filter(|(i, character)| character.is_alive() && *i != starting_point)
                .map(|(i, _)| i)
                .collect(),

            ActionRange::Anyone => {
                // quick trick: set starting_point to an invalid index so all the valid ones will be included
                Self::get_target_indexes(characters, characters.len(), ActionRange::AnyoneButSelf)
            }
        }
    }

    // returns the first alive character, starting from the given point, going forwards if direction is true, backwards otherwise
    // don't call this with the current character or it will just return it every time since it's an alive one
    fn first_alive_index(characters: &[Box<dyn PlayableCharacter>], start: i64, direction: bool) -> i64 {
        let ln = characters.len();
        let mut pointer: i64 = utils::loop_index(start, ln);

        while !characters[pointer as usize].is_alive() {
            pointer += if direction { 1 } else { -1 };
            pointer = utils::loop_index(pointer, ln);
        }

        pointer
    }
}

impl DiceLockUpdate {
    pub(crate) fn new(lock_updates: Vec<DieLockUpdate>) -> DiceLockUpdate {
        DiceLockUpdate(lock_updates)
    }
}

impl DieLockUpdate {
    pub(crate) fn new(die_face: DieFace, amount: Option<u16>, lock_unlock: LockUnlock) -> DieLockUpdate {
        DieLockUpdate {
            die_face: die_face,
            lock_unlock: lock_unlock,
            amount: amount,
        }
    }
}

// don't modify the lock state of the dice by default
impl Default for DiceLockUpdate {
    fn default() -> Self {
        DiceLockUpdate(Vec::new())
    }
}

impl IntoIterator for DiceLockUpdate {
    type Item = DieLockUpdate;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::game::action::ActionRange;
    use crate::game::characters::base_character::CharacterData;
    use crate::game::characters::character::PlayableCharacter;
    use crate::game::characters::characters::SuzieLafette;

    static N_PLAYERS: usize = 10;

    fn get_test_characters() -> [Box<dyn PlayableCharacter>; N_PLAYERS] {
        std::array::from_fn(|_| {
            Box::new(SuzieLafette {
                base_character: CharacterData::new(8),
            }) as Box<dyn PlayableCharacter>
        })
    }

    #[test]
    fn test_action_range_all() {
        let targets = ActionRange::get_target_indexes(&Vec::from(get_test_characters()), 0, ActionRange::Anyone);
        assert_eq!(targets, HashSet::from([0, 1, 2, 3, 4, 5, 6, 7, 8, 9]));
    }

    #[test]
    fn test_action_range_all_but_self() {
        let targets = ActionRange::get_target_indexes(&Vec::from(get_test_characters()), 1, ActionRange::AnyoneButSelf);
        assert_eq!(targets, HashSet::from([0, 2, 3, 4, 5, 6, 7, 8, 9]));
    }

    #[test]
    fn test_action_range_distance_exact() {
        let targets =
            ActionRange::get_target_indexes(&Vec::from(get_test_characters()), 2, ActionRange::ExactDistance(1));
        assert_eq!(targets, HashSet::from([1, 3]));

        let targets =
            ActionRange::get_target_indexes(&Vec::from(get_test_characters()), 0, ActionRange::ExactDistance(2));
        assert_eq!(targets, HashSet::from([2, 8]));
    }

    #[test]
    fn test_action_range_distance_range() {
        let targets =
            ActionRange::get_target_indexes(&Vec::from(get_test_characters()), 0, ActionRange::DistanceRange(1, 3));

        assert_eq!(targets, HashSet::from([7, 8, 9, 1, 2, 3]));
    }
}
