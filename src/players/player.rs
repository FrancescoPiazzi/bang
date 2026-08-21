use crate::game::characters::character::PlayableCharacter;
use crate::game::role::Role;
use crate::players::actor::Actor;

pub(crate) struct Player<'a> {
    pub(crate) role: Role,
    pub(crate) character: Box<dyn PlayableCharacter>,
    pub(crate) actor: Box<dyn Actor<'a>>,
}

impl<'a> Player<'_> {
    pub(crate) fn set_role(&mut self, role: Role) {
        self.role = role;
    }

    pub(crate) fn set_character(&mut self, character: Box<dyn PlayableCharacter>) {
        self.character = character;
    }

    pub(crate) fn get_role(&self) -> Role {
        self.role
    }

    pub(crate) fn get_character(&self) -> &Box<dyn PlayableCharacter> {
        &self.character
    }
}
