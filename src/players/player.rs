use crate::game::characters::character::PlayableCharacter;
use crate::game::role::Role;

pub(crate) struct Player {
    pub(crate) name: String,
    pub(crate) role: Role,
    pub(crate) character: Box<dyn PlayableCharacter>,
}
