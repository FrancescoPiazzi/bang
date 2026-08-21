use crate::game::characters::character::PlayableCharacter;

struct Action<'a> {
    action_type: ActionType,
    source: ActionSource<'a>,
    target: &'a dyn PlayableCharacter,
}

#[derive(Debug)]
pub(crate) enum ActionType {
    Shoot(u32),
    Heal(u32),
    GiveArrows(u32),
}

enum ActionSource<'a> {
    Pile,
    Character(&'a dyn PlayableCharacter),
}

#[derive(Debug)]
pub(crate) enum ActionRange {
    Distance(usize), // used mainly when shooting
    Anyone,          // used by some abilities
    AnyoneButSelf,   // used by nobody as far as I know but it may be useful
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
            ActionRange::Distance(distance_inner) => {
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
