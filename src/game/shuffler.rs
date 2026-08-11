use crate::game::characters::character::PlayableCharacter;
use crate::game::role::Role;

pub(crate) struct RandomShuffler;
pub(crate) struct MockShuffler;
pub(crate) struct ReverseShuffler;

pub(crate) trait Shuffler {
    fn shuffle_characters(&self, chars: &mut [Box<dyn PlayableCharacter>]);
    fn shuffle_roles(&self, roles: &mut [Role]);
}

impl Shuffler for RandomShuffler {
    fn shuffle_characters(&self, chars: &mut [Box<dyn PlayableCharacter>]) {
        use rand::seq::SliceRandom;
        chars.shuffle(&mut rand::rng());
    }

    fn shuffle_roles(&self, roles: &mut [Role]) {
        use rand::seq::SliceRandom;
        roles.shuffle(&mut rand::rng());
    }
}

impl Shuffler for MockShuffler {
    fn shuffle_characters(&self, _: &mut [Box<dyn PlayableCharacter>]) {}

    fn shuffle_roles(&self, _: &mut [Role]) {}
}

impl Shuffler for ReverseShuffler {
    fn shuffle_characters(&self, chars: &mut [Box<dyn PlayableCharacter>]) {
        chars.reverse();
    }

    fn shuffle_roles(&self, roles: &mut [Role]) {
        roles.reverse();
    }
}
