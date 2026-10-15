/* Core implementation of the game, handles turns, actions, and provides information about the game state */

use super::role::Role;
use crate::{
    game::{
        action::{Action, ActionRange, ActionType},
        characters::character::{Archetype, CharacterType, PlayableCharacter},
        damage_type::DamageType,
        dice::{chooser::RandomChooser, dice_roller::DiceRoller, die_face::DieFace},
        settings::Settings,
        shuffler::Shuffler,
    },
    players::{actor::Actor, player::Player},
};

use log::{error, info};

pub(crate) struct MatchController {
    settings: Settings,
    dice_chooser: RandomChooser,
    players: Vec<Player>,
    active_turn_index: usize,
}

impl MatchController {
    pub(crate) fn new(settings: Settings, actors: Vec<Box<dyn Actor>>, shuffler: Box<dyn Shuffler>) -> MatchController {
        let n_players = actors.len();
        let roles = Role::roles_by_n_players(n_players, &shuffler);
        let characters = MatchController::get_characters(n_players, &shuffler);

        let players: Vec<Player> = roles
            .into_iter()
            .zip(characters)
            .zip(actors)
            .map(|((role, character), actor)| Player {
                role: role,
                character: character,
                actor: actor,
            })
            .collect();

        // rule 2.5
        // TODO: an index is annoying to compute and there's no way to check its validity at compile time
        // use an infinite iterator created with .cycle() over the players instead
        let first_turn_index = players
            .iter()
            .enumerate()
            .filter(|(_i, player)| player.role == Role::SHERIFF)
            .map(|(i, _player)| i)
            .next()
            .unwrap_or_else(|| {
                error!("Failed to find the sheriff, first player begins");
                0
            });

        MatchController {
            settings: settings,
            dice_chooser: RandomChooser {},
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
    pub(crate) fn get_target_options(&self, range: ActionRange) -> Vec<&Box<dyn PlayableCharacter>> {
        ActionRange::get_targets(&self.players, self.active_turn_index, range)
            .iter_mut()
            .map(|player| player.get_character())
            .collect()
    }

    // executes the turn of the current player, recomputes the current player at the end
    pub(crate) fn turn(&mut self) {
        // get player for this turn
        /*
        let current_player_probably = self.players.get_mut(self.active_turn_index);

        let current_player = if let Some(player) = current_player_probably {
            player
        } else {
            error!(
                "Invalid active turn index (index: {}, players: {}), setting it to 0",
                self.active_turn_index,
                self.players.len()
            );
            self.active_turn_index = 0;
            self.players.get_mut(0).unwrap()
        };

        let current_character: &mut Box<dyn PlayableCharacter + 'static> = &mut current_player.character;
        let current_actor: &mut Box<dyn Actor<'a> + 'static> = &mut current_player.actor;*/

        // get the dice that player has to throw and build a dice roller object with them
        let dice_faces = self.get_current_character().get_dice_faces();
        let mut dice_roller = DiceRoller::new(
            vec![dice_faces]
                .into_iter()
                .cycle()
                .take(self.settings.dice_thrown)
                .collect(),
        );

        // TODO: if expansion is enabled, give possibility to swap dice
        // (perhaps put the code to do it before building the dice roller)

        // first roll (don't let player lock dice here for rule 3.1)
        let mut dice_roll = dice_roller.throw(&mut self.dice_chooser);

        // handle eventual rerolls
        let mut rerolls_left = self.get_current_character().get_max_rerolls(self.settings.max_rerolls);
        while rerolls_left > 0 {
            if self.get_current_actor_mut().throw_dice_again(&dice_roll) {
                dice_roller.update_dice_locks(self.get_current_actor_mut().get_dice_locks(&dice_roll));
                dice_roll = dice_roller.throw(&mut self.dice_chooser);

                // handle combos
                for combo in self.get_current_character().get_dice_combos() {
                    if combo.is_triggered(&dice_roll) {
                        for action in combo.actions {
                            self.resolve_action(action);
                            if self.is_match_terminated() {
                                return;
                            }
                        }
                    }
                }
            } else {
                break;
            }
            rerolls_left -= 1;
        }

        // resolve dice, following the order in rule 4
        for die_face in dice_roll.into_iter() {
            match die_face {
                DieFace::Shoot1 => {
                    self.resolve_action(Action::new(
                        ActionType::Damage(DamageType::Bullet, 1),
                        ActionRange::ExactDistance(1),
                    ));
                }
                DieFace::Shoot2 => {
                    self.resolve_action(Action::new(
                        ActionType::Damage(DamageType::Bullet, 1),
                        ActionRange::ExactDistance(2),
                    ));
                }
                DieFace::Beer => {
                    self.resolve_action(Action::new(ActionType::Heal(1), ActionRange::Anyone));
                }
                DieFace::Arrow => {
                    self.resolve_action(Action::new(ActionType::GiveArrows(1), ActionRange::Myself));
                }
                DieFace::Dynamite => {}
                DieFace::Gatling => {}
                DieFace::NonBlockingDynamite => {}
                DieFace::HealingGatling => {}
                DieFace::Shoot1or2 => {
                    self.resolve_action(Action::new(
                        ActionType::Damage(DamageType::Bullet, 1),
                        ActionRange::DistanceRange(1, 2),
                    ));
                }
                DieFace::Shoot2or3 => {
                    self.resolve_action(Action::new(
                        ActionType::Damage(DamageType::Bullet, 1),
                        ActionRange::DistanceRange(1, 2),
                    ));
                }
            };

            if self.is_match_terminated() {
                return;
            }
        }

        self.active_turn_index += 1;
        // a bit extra but this handles increasing the turn index by more than one
        // in order to skip the next player's turn if it's ever needed.
        while self.active_turn_index >= self.players.len() {
            self.active_turn_index -= self.players.len();
        }
    }

    fn get_current_character(&self) -> &Box<dyn PlayableCharacter> {
        &self.get_current_player().get_character()
    }

    fn get_current_actor(&self) -> &Box<dyn Actor> {
        &self.get_current_player().get_actor()
    }

    fn get_current_player(&self) -> &Player {
        self.players.get(self.active_turn_index).unwrap_or_else(|| {
            error!(
                "Invalid active turn index (index: {}, players: {}), falling back to index 0",
                self.active_turn_index,
                self.players.len()
            );
            &self.players[0]
        })
    }

    fn get_current_character_mut(&mut self) -> &mut Box<dyn PlayableCharacter> {
        self.get_current_player_mut().get_character_mut()
    }

    fn get_current_actor_mut(&mut self) -> &mut Box<dyn Actor> {
        self.get_current_player_mut().get_actor_mut()
    }

    fn get_current_player_mut(&mut self) -> &mut Player {
        if self.active_turn_index < self.players.len() {
            &mut self.players[self.active_turn_index]
        } else {
            error!(
                "Invalid active turn index (index: {}, players: {}), falling back to index 0",
                self.active_turn_index,
                self.players.len()
            );
            &mut self.players[0]
        }
    }

    fn resolve_action(&mut self, action: Action) {
        match action.action_type {
            ActionType::Damage(damage_type, amount) => {
                let options = self.get_target_options(action.action_range);
                let target_idx = self
                    .get_current_actor()
                    .choose_target(&options, &ActionType::Damage(DamageType::Bullet, amount));
                // TODO: validate index
                let target = self.players[target_idx].get_character_mut();
                target.take_damage(amount, damage_type)
            }

            ActionType::Heal(amount) => {
                let options = self.get_target_options(action.action_range);
                let target_idx = self
                    .get_current_actor()
                    .choose_target(&options, &ActionType::Heal(amount));
                // TODO: validate index
                let target = self.players[target_idx].get_character_mut();
                target.heal(amount)
            }

            ActionType::GiveArrows(amount) => {
                self.get_current_character_mut().give_arrows(amount);
            }
            ActionType::DiscardAllArrows => {
                self.get_current_character_mut().discard_all_arrows();
            }
        }
    }

    // TODO: return Option<Vec<Roles that won>>
    fn is_match_terminated(&self) -> bool {
        // sheriff is dead
        let sheriff_probably = self.players.iter().filter(|player| player.role == Role::SHERIFF).next();

        let Some(sheriff) = sheriff_probably else {
            error!("Failed to find the sheriff, pretend it never existed and don't end the match");
            return false;
        };

        if !sheriff.character.is_alive() {
            return true;
        }

        // only sheriff and deputy are left
        self.players
            .iter()
            .filter(|player| player.role != Role::SHERIFF && player.role != Role::DEPUTY)
            .all(|player| player.character.is_alive())
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::game::shuffler::{MockShuffler, ReverseShuffler};
    use crate::players::bots::base_bot::BaseBot;

    use super::*;

    #[test]
    fn test_role_assignment() {
        let actors = generate_base_bot_actors(5);
        let roles = count_roles(&MatchController::new(
            Settings::with_players(5),
            actors,
            Box::new(MockShuffler),
        ));
        assert_eq!(*roles.get(&Role::SHERIFF).unwrap(), 1);
        assert_eq!(*roles.get(&Role::DEPUTY).unwrap(), 1);
        assert_eq!(*roles.get(&Role::OUTLAW).unwrap(), 2);
        assert_eq!(*roles.get(&Role::RENEGADE).unwrap(), 1);

        let actors = generate_base_bot_actors(8);
        let roles = count_roles(&MatchController::new(
            Settings::with_players(8),
            actors,
            Box::new(MockShuffler),
        ));
        assert_eq!(*roles.get(&Role::SHERIFF).unwrap(), 1);
        assert_eq!(*roles.get(&Role::DEPUTY).unwrap(), 2);
        assert_eq!(*roles.get(&Role::OUTLAW).unwrap(), 3);
        assert_eq!(*roles.get(&Role::RENEGADE).unwrap(), 2);

        let actors = generate_base_bot_actors(10);
        let roles = count_roles(&MatchController::new(
            Settings::with_players(10),
            actors,
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
        let actors = generate_base_bot_actors(5);
        let ctr = MatchController::new(Settings::default(), actors, Box::new(MockShuffler));
        assert_eq!(ctr.players[ctr.active_turn_index].role, Role::SHERIFF);

        let actors = generate_base_bot_actors(5);
        let ctr = MatchController::new(Settings::default(), actors, Box::new(ReverseShuffler));
        assert_eq!(ctr.players[ctr.active_turn_index].role, Role::SHERIFF);
    }

    fn generate_base_bot_actors<'a>(n: usize) -> Vec<Box<dyn Actor>> {
        let actors_template: Vec<BaseBot> = vec![BaseBot {}; n];
        actors_template
            .iter()
            .map(|a| Box::new(a.clone()) as Box<dyn Actor>)
            .collect()
    }
}
