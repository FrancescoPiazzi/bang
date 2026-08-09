/* Core implementation of the game, handles turns, actions, and provides information about the game state */

use super::role::Role;
use crate::{
    game::characters::{
        character::{Archetype, CharacterType, PlayableCharacter}
    },
    players::{player::Player},
};

use log::{error, info};

struct MatchController {
    players: Vec<Player>,
    active_turn_index: usize,
}

impl MatchController {
    pub(crate) fn new(n_players: usize) -> MatchController {
        let roles = Role::roles_by_n_players(n_players);
        let characters = MatchController::get_characters(n_players);

        let players: Vec<Player> = roles
            .into_iter()
            .zip(characters)
            .map(|(role, character)| Player {
                role: role,
                character: character,
            })
            .collect();

        // rule 2.5
        let first_turn_index = players
            .iter()
            .enumerate()
            .filter(|(i, player)| player.role == Role::SHERIFF)
            .map(|(i, player)| i)
            .next()
            .unwrap_or_else(|| {
                error!("Failed to find the sheriff, first player begins");
                0
            });

        MatchController {
            players: players,
            active_turn_index: first_turn_index,
        }
    }

    pub(crate) fn get_characters(n_players: usize) -> Vec<Box<dyn PlayableCharacter>> {
        let characters = vec![CharacterType::SuzieLafette];

        if n_players > characters.len() {
            info!(
                "more players ({}) than different characters ({}), some characters will be assigned more than once",
                n_players,
                characters.len()
            );
        }

        characters
            .into_iter()
            .cycle()
            .take(n_players)
            .map(|character| Archetype::new(character))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use std::array;

    use crate::game::dice::die_face::DieFace::*;

    use super::*;

    #[test]
    fn test_start_game_from_sheriff() {}
}
