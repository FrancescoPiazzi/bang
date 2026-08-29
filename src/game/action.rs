use crate::game::{characters::character::PlayableCharacter, dice::die_face::DieFace, damage_type::DamageType};

// an action a player takes that must be resolved by the controller
// used to shoot, heal, give arrows, and interact with most of the game features
pub(crate) struct Interaction<'a> {
    action_type: ActionType,
    source: ActionSource<'a>,
    target: &'a dyn PlayableCharacter, // TODO pretty sure reference can be replaced by a Box here
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
    Shoot(DamageType, u16),
    Heal(u16),
    GiveArrows(u16),
}

pub(crate) enum ActionSource<'a> {
    Pile,
    Character(&'a dyn PlayableCharacter),
}

#[derive(Debug)]
pub(crate) enum ActionRange {
    ExactDistance(usize), // used mainly when shooting
    // DistanceRange(usize, usize),
    Anyone,          // used by some abilities
    AnyoneButSelf,   // used by nobody as far as I know but it may be useful, perhaps to implement gatling?
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
    pub(crate) fn new(lock_updates: Vec<DieLockUpdate>) -> DiceLockUpdate{
        DiceLockUpdate(lock_updates)
    }
}

impl DieLockUpdate {
    pub(crate) fn new(die_face: DieFace, amount: Option<u16>, lock_unlock: LockUnlock) -> DieLockUpdate{
        DieLockUpdate { die_face: die_face, lock_unlock: lock_unlock, amount: amount }
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
    fn test_action_range_distance() {
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
}
