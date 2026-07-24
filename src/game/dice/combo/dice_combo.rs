use std::collections::HashMap;


use super::super::dice_face::DiceFace;
use super::super::dice_roller::DiceRollResult;

pub(crate) trait DiceCombo {
    fn is_triggered(&self, result: &DiceRollResult) -> bool;
}

/* A simple dice combo, matching only a certain number of a specific face
all of the dice combos in the game are instances of this class */
struct SimpleDiceCombo {
    dice_face: DiceFace,
    n_dice: u16,
}

/* A more generic dice combo, allowing to match any combination of dice */
// TODO: I think this can be rewritten as Vec<SimpleDiceCombo> to avoid repeating
// the similar matching logic without losing efficiency but I'm not sure
struct GenericDiceCombo(HashMap<DiceFace, u16>);

impl DiceCombo for SimpleDiceCombo {
    fn is_triggered(&self, result: &DiceRollResult) -> bool {
        *result.0.get(&self.dice_face).unwrap_or(&0) >= self.n_dice
    }
}

impl DiceCombo for GenericDiceCombo {
    fn is_triggered(&self, result: &DiceRollResult) -> bool {
        self.0.iter().all(|entry|
            *result.0.get(&entry.0).unwrap_or(&0) >= *entry.1
        )
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_combo() {
        let combo1 = SimpleDiceCombo{dice_face: DiceFace::Dynamite, n_dice: 3};
        let combo2 = SimpleDiceCombo{dice_face: DiceFace::Gatling, n_dice: 3};

        let result1 = DiceRollResult{0: HashMap::from([
            (DiceFace::Dynamite, 3), 
            (DiceFace::Arrow, 1),
            (DiceFace::Shoot1, 2),
        ])};
        let result2 = DiceRollResult{0: HashMap::from([
            (DiceFace::Dynamite, 5), 
            (DiceFace::Gatling, 1),
        ])};
        let result3 = DiceRollResult{0: HashMap::from([
            (DiceFace::Gatling, 3), 
            (DiceFace::Arrow, 1),
            (DiceFace::Shoot1, 2),
        ])};

        assert_eq!(combo1.is_triggered(&result1), true);
        assert_eq!(combo1.is_triggered(&result2), true);
        assert_eq!(combo1.is_triggered(&result3), false);

        assert_eq!(combo2.is_triggered(&result1), false);
        assert_eq!(combo2.is_triggered(&result2), false);
        assert_eq!(combo2.is_triggered(&result3), true);
    }

    #[test]
    fn test_generic_combo() {
        let combo1 = GenericDiceCombo{0: HashMap::from([
            (DiceFace::Gatling, 1), 
            (DiceFace::Arrow, 1),
            (DiceFace::Beer, 1)
        ])};
        let combo2 = GenericDiceCombo{0: HashMap::from([
            (DiceFace::Gatling, 3), 
            (DiceFace::Shoot1, 2),
            (DiceFace::Shoot2, 1)
        ])};

        let result1 = DiceRollResult{0: HashMap::from([
            (DiceFace::Arrow, 3), 
            (DiceFace::Beer, 1),
            (DiceFace::Gatling, 2),
        ])};
        let result2 = DiceRollResult{0: HashMap::from([
            (DiceFace::Gatling, 1), 
            (DiceFace::Shoot1, 3),
            (DiceFace::Shoot2, 2),
        ])};
        let result3 = DiceRollResult{0: HashMap::from([
            (DiceFace::Gatling, 3), 
            (DiceFace::Shoot1, 2),
            (DiceFace::Shoot2, 1),
        ])};

        assert_eq!(combo1.is_triggered(&result1), true);
        assert_eq!(combo1.is_triggered(&result2), false);
        assert_eq!(combo1.is_triggered(&result3), false);

        assert_eq!(combo2.is_triggered(&result1), false);
        assert_eq!(combo2.is_triggered(&result2), false);
        assert_eq!(combo2.is_triggered(&result3), true);
    }
}