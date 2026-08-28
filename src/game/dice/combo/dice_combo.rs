use std::collections::HashMap;

use super::super::dice_roller::DiceRollResult;
use super::super::die_face::DieFace;

pub(crate) struct DiceCombo(HashMap<DieFace, usize>);

impl DiceCombo {
    pub(crate) fn from(hash_map: HashMap<DieFace, usize>) -> DiceCombo {
        DiceCombo { 0: hash_map }
    }

    fn is_triggered(&self, result: &DiceRollResult) -> bool {
        self.0
            .iter()
            .all(|(face_required, count_required)| result.get(&face_required) >= *count_required)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generic_combo() {
        let combo1 = DiceCombo {
            0: HashMap::from([(DieFace::Gatling, 1), (DieFace::Arrow, 1), (DieFace::Beer, 1)]),
        };
        let combo2 = DiceCombo {
            0: HashMap::from([(DieFace::Gatling, 3), (DieFace::Shoot1, 2), (DieFace::Shoot2, 1)]),
        };

        let result1 = DiceRollResult::from(HashMap::from([
            (DieFace::Arrow, 3),
            (DieFace::Beer, 1),
            (DieFace::Gatling, 2),
        ]));
        let result2 = DiceRollResult::from(HashMap::from([
            (DieFace::Gatling, 1),
            (DieFace::Shoot1, 3),
            (DieFace::Shoot2, 2),
        ]));
        let result3 = DiceRollResult::from(HashMap::from([
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
