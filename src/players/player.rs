use crate::game::characters::character::PlayableCharacter;
use crate::game::role::Role;
use crate::players::actor::Actor;

pub(crate) struct Player {
    pub(crate) role: Role,
    pub(crate) character: Box<dyn PlayableCharacter>,
    pub(crate) actor: Box<dyn Actor>,
}

impl Player {
    pub(crate) fn new(role: Role, character: Box<dyn PlayableCharacter>, actor: Box<dyn Actor>) -> Player {
        Player {
            role: role,
            character: character,
            actor: actor,
        }
    }

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

    pub(crate) fn get_character_mut(&mut self) -> &mut Box<dyn PlayableCharacter> {
        &mut self.character
    }

    pub(crate) fn get_actor(&self) -> &Box<dyn Actor> {
        &self.actor
    }

    pub(crate) fn get_actor_mut(&mut self) -> &mut Box<dyn Actor> {
        &mut self.actor
    }
}
