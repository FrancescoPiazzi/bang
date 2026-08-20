use crate::game::characters::character::PlayableCharacter;
use crate::game::role::Role;

pub(crate) struct Player {
    pub(crate) role: Role,
    pub(crate) character: Box<dyn PlayableCharacter>,
}

impl Player {
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
