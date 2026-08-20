/* Core implementation of the game, handles turns, actions, and provides information about the game state */

use super::role::Role;
use crate::{
    game::{
        action::ActionRange,
        characters::character::{Archetype, CharacterType, PlayableCharacter},
        settings::Settings,
        shuffler::Shuffler,
    },
    players::player::Player,
};

use log::{error, info};

struct MatchController {
    settings: Settings,
    players: Vec<Player>,
    active_turn_index: usize,
}

impl MatchController {
    pub(crate) fn new(settings: Settings, shuffler: Box<dyn Shuffler>) -> MatchController {
        let roles = Role::roles_by_n_players(settings.n_players, &shuffler);
        let characters = MatchController::get_characters(settings.n_players, &shuffler);

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
            settings: settings,
            players: players,
            active_turn_index: first_turn_index,
        }
    }

    pub(crate) fn get_characters(n_players: usize, shuffler: &Box<dyn Shuffler>) -> Vec<Box<dyn PlayableCharacter>> {
        let characters = vec![CharacterType::SuzieLafette];

        if n_players > characters.len() {
            info!(
                "more players ({}) than different characters ({}), some characters will be assigned more than once",
                n_players,
                characters.len()
            );
        }

        let mut res = Vec::from(
            characters
                .into_iter()
                .cycle()
                .take(n_players)
                .map(|character| Archetype::new(character))
                .collect::<Vec<_>>(),
        );

        shuffler.shuffle_characters(&mut res);
        res
    }

    // returns the possible target options for the currently active player
    // TODO
    /*pub(crate) fn get_target_options(&self, range: ActionRange) -> Vec<Box<&dyn PlayableCharacter>>{
        ActionRange::get_targets(
            &self.players,
            self.active_turn_index, range
        ).iter().map(|player| **player.get_character()).collect()
    }
    */
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::game::shuffler::{MockShuffler, ReverseShuffler};

    use super::*;

    #[test]
    fn test_role_assignment() {
        let roles = count_roles(&MatchController::new(Settings::with_players(5), Box::new(MockShuffler)));
        assert_eq!(*roles.get(&Role::SHERIFF).unwrap(), 1);
        assert_eq!(*roles.get(&Role::DEPUTY).unwrap(), 1);
        assert_eq!(*roles.get(&Role::OUTLAW).unwrap(), 2);
        assert_eq!(*roles.get(&Role::RENEGADE).unwrap(), 1);

        let roles = count_roles(&MatchController::new(Settings::with_players(8), Box::new(MockShuffler)));
        assert_eq!(*roles.get(&Role::SHERIFF).unwrap(), 1);
        assert_eq!(*roles.get(&Role::DEPUTY).unwrap(), 2);
        assert_eq!(*roles.get(&Role::OUTLAW).unwrap(), 3);
        assert_eq!(*roles.get(&Role::RENEGADE).unwrap(), 2);

        let roles = count_roles(&MatchController::new(
            Settings::with_players(10),
            Box::new(MockShuffler),
        ));
        assert_eq!(*roles.get(&Role::SHERIFF).unwrap(), 1);
        assert_eq!(*roles.get(&Role::DEPUTY).unwrap(), 3);
        assert_eq!(*roles.get(&Role::OUTLAW).unwrap(), 4);
        assert_eq!(*roles.get(&Role::RENEGADE).unwrap(), 2);
    }

    // returns a HashMap mapping role to how many players have that role in the current controller
    fn count_roles(ctr: &MatchController) -> HashMap<Role, usize> {
        let mut res = HashMap::new();

        ctr.players.iter().for_each(|player| {
            res.entry(player.role).and_modify(|count| *count += 1).or_insert(1);
        });

        res
    }

    #[test]
    fn test_sheriff_starts() {
        let ctr = MatchController::new(Settings::default(), Box::new(MockShuffler));
        assert_eq!(ctr.players[ctr.active_turn_index].role, Role::SHERIFF);

        let ctr = MatchController::new(Settings::default(), Box::new(ReverseShuffler));
        assert_eq!(ctr.players[ctr.active_turn_index].role, Role::SHERIFF);
    }
}
