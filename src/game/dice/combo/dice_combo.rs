use std::collections::HashMap;

use super::super::dice_roller::DiceRollResult;
use super::super::die_face::DieFace;
use crate::game::action::Action;

pub(crate) struct DiceCombo {
    requisite: HashMap<DieFace, usize>,
    actions: Vec<Action>,
    end_rerolls_if_triggered: bool,
}

impl DiceCombo {
    pub(crate) fn new(
        hash_map: HashMap<DieFace, usize>,
        actions: Vec<Action>,
        end_rerolls_if_triggered: bool,
    ) -> DiceCombo {
        DiceCombo {
            requisite: hash_map,
            actions: actions,
            end_rerolls_if_triggered: end_rerolls_if_triggered,
        }
    }

    pub(crate) fn is_triggered(&self, result: &DiceRollResult) -> bool {
        self.requisite
            .iter()
            .all(|(face_required, count_required)| result.get(face_required) >= *count_required)
    }

    pub(crate) fn get_actions(&self) -> Vec<Action> {
        self.actions.clone()
    }

    pub(crate) fn end_rerolls_if_triggered(&self) -> bool {
        self.end_rerolls_if_triggered
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
            vec![],
            false,
        );
        let combo2 = DiceCombo::new(
            HashMap::from([(DieFace::Gatling, 3), (DieFace::Shoot1, 2), (DieFace::Shoot2, 1)]),
            vec![],
            false,
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

        assert!(combo1.is_triggered(&result1));
        assert!(!combo1.is_triggered(&result2));
        assert!(!combo1.is_triggered(&result3));

        assert!(!combo2.is_triggered(&result1));
        assert!(!combo2.is_triggered(&result2));
        assert!(combo2.is_triggered(&result3));
    }
}
