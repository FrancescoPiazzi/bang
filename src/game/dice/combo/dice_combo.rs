use std::collections::HashMap;

use super::super::dice_roller::DiceRollResult;
use super::super::die_face::DieFace;
use crate::game::action::NonReferencingInteraction;

pub(crate) struct DiceCombo{
    requisite: HashMap<DieFace, usize>, 
    actions: Vec<NonReferencingInteraction>
}

impl DiceCombo{
    pub(crate) fn new(hash_map: HashMap<DieFace, usize>, actions: Vec<NonReferencingInteraction>) -> DiceCombo {
        DiceCombo { requisite: hash_map, actions: actions}
    }

    pub(crate) fn is_triggered(&self, result: &DiceRollResult) -> bool {
        self.requisite
            .iter()
            .all(|(face_required, count_required)| result.get(&face_required) >= *count_required)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::*;

    #[test]
    fn test_generic_combo() {
        let combo1 = DiceCombo::new(
            HashMap::from([(DieFace::Gatling, 1), (DieFace::Arrow, 1), (DieFace::Beer, 1)]),
            vec![]
        );
        let combo2 = DiceCombo::new(
            HashMap::from([(DieFace::Gatling, 3), (DieFace::Shoot1, 2), (DieFace::Shoot2, 1)]),
            vec![]
        );

        let result1 = DiceRollResult::from(BTreeMap::from([
            (DieFace::Arrow, 3),
            (DieFace::Beer, 1),
            (DieFace::Gatling, 2),
        ]));
        let result2 = DiceRollResult::from(BTreeMap::from([
            (DieFace::Gatling, 1),
            (DieFace::Shoot1, 3),
            (DieFace::Shoot2, 2),
        ]));
        let result3 = DiceRollResult::from(BTreeMap::from([
            (DieFace::Gatling, 3),
            (DieFace::Shoot1, 2),
            (DieFace::Shoot2, 1),
        ]));

        assert_eq!(combo1.is_triggered(&result1), true);
        assert_eq!(combo1.is_triggered(&result2), false);
        assert_eq!(combo1.is_triggered(&result3), false);

        assert_eq!(combo2.is_triggered(&result1), false);
        assert_eq!(combo2.is_triggered(&result2), false);
        assert_eq!(combo2.is_triggered(&result3), true);
    }
}
