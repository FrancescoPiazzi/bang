use crate::game::{characters::character::PlayableCharacter, damage_type::DamageType, dice::die_face::DieFace};

// an action a player takes that must be resolved by the controller
// used to shoot, heal, give arrows, and interact with most of the game features
// note: use NonReferencingInteraction if possible to not pollute code with lifetimes
pub(crate) struct Interaction<'ch> {
    action_type: ActionType,
    source: ActionSource<'ch>,
    target: &'ch dyn PlayableCharacter,
}

impl<'ch> Interaction<'ch> {
    pub(crate) fn new(action_type: ActionType, source: ActionSource<'ch>, target: &'ch dyn PlayableCharacter) -> Interaction<'ch> {
        Interaction { action_type: action_type, source: source, target: target }
    }
}

// an action a player takes that doesn't have a specific target, 
// since this doesn't need to hold a reference to a character, 
// it doesn't pollute outside code with lifetimes, also allows for multiple targets
pub(crate) struct NonReferencingInteraction {
    action_type: ActionType,
    action_range: ActionRange,
}

impl NonReferencingInteraction {
    pub(crate) fn new(action_type: ActionType, action_range: ActionRange) -> NonReferencingInteraction {
        NonReferencingInteraction { action_type: action_type, action_range: action_range }
    }
}

// the action of locking or unlocking a certain amount of dice before rethrowing them.
// used by the actors to communicate their decisions to the controller and by the controller
// to notify other actors
pub(crate) struct DiceLockUpdate(Vec<DieLockUpdate>);

// update for a given die face, pass amount: None to lock or unlock all dice with the given face
pub(crate) struct DieLockUpdate {
    pub(crate) die_face: DieFace,
    pub(crate) amount: Option<u16>,
    pub(crate) lock_unlock: LockUnlock,
}

#[derive(Debug)]
pub(crate) enum ActionType {
    Damage(DamageType, u16),
    Heal(u16),
    GiveArrows(i16),
    DiscardAllArrows,
}

pub(crate) enum ActionSource<'ch> {
    Pile,   // interactions with the pile, i.e. healing TODO: this and None can probably be handled the same way
    Character(&'ch dyn PlayableCharacter),  // make somone else the source of an action, no idea if this is ever needed
}

#[derive(Debug)]
pub(crate) enum ActionRange {
    Myself,
    ExactDistance(usize), // used mainly when shooting
    DistanceRange(usize, usize),
    Anyone,
    AnyoneButSelf,
}

pub(crate) enum LockUnlock {
    Lock,
    Unlock,
}

impl ActionRange {
    // TODO: maybe return type can have a different lifetime that may last less than 'a
    // but I'm not gonna mess with them now that I haven't written the rest of the code yet
    pub(crate) fn get_targets<'a, T>(
        elements: &'a Vec<T>,
        starting_point: usize,
        action_range: ActionRange,
    ) -> Vec<&'a T> {
        match action_range {
            ActionRange::Myself => {
                Vec::from([&elements[starting_point]])
            }

            ActionRange::ExactDistance(distance_inner) => {
                // an exact distance range will always have at most 2 targets
                let ln = elements.len() as i64;
                let first: i64 = (starting_point as i64 - distance_inner as i64).rem_euclid(ln);
                let second: i64 = (starting_point as i64 + distance_inner as i64) % ln;

                if first != second {
                    Vec::from([&elements[first as usize], &elements[second as usize]])
                } else {
                    Vec::from([&elements[first as usize]])
                }
            }

            ActionRange::DistanceRange(min, mut max) => {
                if max > elements.len() {
                    max = elements.len();
                }

                (min..=max).map(|distance| Self::get_targets(elements, starting_point, ActionRange::ExactDistance(distance))).flatten().collect()
            }
            
            ActionRange::AnyoneButSelf => elements
                .iter()
                .enumerate()
                .filter(|(i, _)| starting_point != *i)
                .map(|(_, element)| element)
                .collect(),
            
            ActionRange::Anyone => elements.iter().collect(),
        }
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


// all these test assume internal functioning because of how they compare vectors but I can't be bothered to 
// do it properly (sort them first)
#[cfg(test)]
mod tests {
    use crate::game::action::ActionRange;

    #[test]
    fn test_action_range_all() {
        let elements = vec![1, 2, 3, 4, 5];
        let targets: Vec<i32> = ActionRange::get_targets(&elements, 0, ActionRange::Anyone)
            .into_iter()
            .map(|x| *x)
            .collect();
        assert_eq!(targets, elements);
    }

    #[test]
    fn test_action_range_all_but_self() {
        let elements = vec![1, 2, 3, 4, 5];
        let targets: Vec<i32> = ActionRange::get_targets(&elements, 1, ActionRange::AnyoneButSelf)
            .into_iter()
            .map(|x| *x)
            .collect();
        assert_eq!(targets, vec![1, 3, 4, 5]);
    }

    #[test]
    fn test_action_range_distance_exact() {
        let elements = vec![1, 2, 3, 4, 5];
        let targets: Vec<i32> = ActionRange::get_targets(&elements, 0, ActionRange::ExactDistance(2))
            .into_iter()
            .map(|x| *x)
            .collect();
        assert_eq!(targets, vec![4, 3]);

        let elements = vec![1, 2, 3, 4, 5];
        let targets: Vec<i32> = ActionRange::get_targets(&elements, 2, ActionRange::ExactDistance(1))
            .into_iter()
            .map(|x| *x)
            .collect();
        assert_eq!(targets, vec![2, 4]);
    }

    #[test]
    fn test_action_range_distance_range() {
        let elements = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let targets: Vec<i32> = ActionRange::get_targets(&elements, 0, ActionRange::DistanceRange(1, 3))
            .into_iter()
            .map(|x| *x)
            .collect();
        assert_eq!(targets, vec![9, 2, 8, 3, 7, 4]);
    }
}
