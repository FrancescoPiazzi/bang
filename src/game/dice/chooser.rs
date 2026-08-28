// trait for something that has to select a value from a collection
// for now only used to pick a face from the set of possible die faces
// of a given die, effectively rolling the die

pub(crate) trait Chooser<T: Clone> {
    fn choose(&self, iterator: &Vec<T>) -> Option<T>;
}

struct RandomChooser {}

impl<T: Clone> Chooser<T> for RandomChooser {
    fn choose(&self, iterator: &Vec<T>) -> Option<T> {
        use rand::seq::IndexedRandom;

        iterator.choose(&mut rand::rng()).cloned()
    }
}
