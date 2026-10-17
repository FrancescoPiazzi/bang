/* Core implementation of the game, handles turns, actions, and provides information about the game state */

use super::role::Role;
use crate::{
    game::{
        action::{Action, ActionRange, ActionType},
        characters::character::{Archetype, CharacterType, PlayableCharacter},
        damage_type::DamageType,
        dice::{
            chooser::RandomChooser,
            dice_roller::{DiceRollResult, DiceRoller},
            die_face::DieFace,
        },
        settings::Settings,
        shuffler::Shuffler,
        utils,
    },
    players::{actor::Actor, player::Player},
};

use log::{error, info, debug, trace};

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
        let first_turn_index = players
            .iter()
            .enumerate()
            .filter(|(_i, player)| player.role == Role::Sheriff)
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

        let mut res = characters
            .into_iter()
            .cycle()
            .take(n_players)
            .map(|character| Archetype::new(character))
            .collect::<Vec<_>>();

        shuffler.shuffle_characters(&mut res);
        res
    }

    // executes the turn of the current player, recomputes the current player at the end
    pub(crate) fn turn(&mut self) -> Result<Option<Role>, TurnError> {
        debug!("new turn, players: \n{:?}", self.players);

        self.get_current_character_mut().start_turn();

        let dice_roll_phase_result = self.dice_roll_phase()?;

        trace!("dice roll phase result: {:?}", dice_roll_phase_result);

        match dice_roll_phase_result {
            DiceRollPhaseResult::Winner(winner) => {
                return Ok(Some(winner));
            }
            DiceRollPhaseResult::Roll(dice_roll) => {
                let result = self.dice_resolution_phase(dice_roll)?;
                if let Some(_) = result {
                    return Ok(result);
                }

                if self.get_current_character().is_alive() {
                    self.get_current_character_mut().end_turn();
                }

                self.advance_turn_idx_phase()?;

                Ok(None)
            }
        }
    }

    fn dice_roll_phase(&mut self) -> Result<DiceRollPhaseResult, TurnError> {
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
        let mut rerolls_left: i16 = self.get_current_character().get_max_rerolls(self.settings.max_rerolls) as i16;
        while rerolls_left > 0 {
            if self.get_current_actor_mut().throw_dice_again(&dice_roll) {
                dice_roller.update_dice_locks(self.get_current_actor_mut().get_dice_locks(&dice_roll));
                dice_roll = dice_roller.throw(&mut self.dice_chooser);

                // handle combos
                for combo in self.get_current_character().get_dice_combos() {
                    if combo.is_triggered(&dice_roll) {
                        for action in combo.get_actions() {
                            let resolve_result = self.resolve_action(action);

                            match resolve_result {
                                Ok(None) => {}
                                Ok(Some(winner)) => {
                                    return Ok(DiceRollPhaseResult::Winner(winner));
                                }
                                Err(error) => {
                                    return Err(error);
                                }
                            }
                        }
                        if combo.end_rerolls_if_triggered() {
                            rerolls_left = 0;
                        }
                    }
                }
            } else {
                break;
            }
            rerolls_left -= 1;
        }

        Ok(DiceRollPhaseResult::Roll(dice_roll))
    }

    // resolve dice, following the order in rule 4
    fn dice_resolution_phase(&mut self, dice_roll: DiceRollResult) -> Result<Option<Role>, TurnError> {
        for die_face in dice_roll.into_iter() {
            let resolve_result = match die_face {
                DieFace::Shoot1 => self.resolve_action(Action::new(
                    ActionType::Damage(DamageType::Bullet, 1),
                    ActionRange::ExactDistance(1),
                ))?,
                DieFace::Shoot2 => self.resolve_action(Action::new(
                    ActionType::Damage(DamageType::Bullet, 1),
                    ActionRange::ExactDistance(2),
                ))?,
                DieFace::Beer => self.resolve_action(Action::new(ActionType::Heal(1), ActionRange::Anyone))?,
                DieFace::Arrow => self.resolve_action(Action::new(ActionType::GiveArrows(1), ActionRange::Myself))?,
                DieFace::Dynamite => None,
                DieFace::Gatling => None,
                DieFace::NonBlockingDynamite => None,
                DieFace::HealingGatling => None,
                DieFace::Shoot1or2 => self.resolve_action(Action::new(
                    ActionType::Damage(DamageType::Bullet, 1),
                    ActionRange::DistanceRange(1, 2),
                ))?,
                DieFace::Shoot2or3 => self.resolve_action(Action::new(
                    ActionType::Damage(DamageType::Bullet, 1),
                    ActionRange::DistanceRange(1, 2),
                ))?,
            };

            if let Some(winner) = resolve_result {
                return Ok(Some(winner));
            }
        }

        Ok(None)
    }

    fn advance_turn_idx_phase(&mut self) -> Result<(), TurnError> {
        self.active_turn_index += 1;
        let mut iterations = 0;

        // a bit extra but this handles increasing the turn index by more than one
        // in order to skip the next player's turn if it's ever needed.
        while self.active_turn_index >= self.players.len() || !self.get_current_character().is_alive() {
            self.active_turn_index += 1;
            if self.active_turn_index >= self.players.len() {
                self.active_turn_index -= self.players.len();
            }

            // quick check to not get stuck in the loop
            iterations += 1;
            if iterations > self.players.len() {
                error!("Cannot find next player to give turn to, match should have ended");
                return Err(TurnError::CannotFindNext);
            }
        }

        Ok(())
    }

    // returns the possible target options for the currently active player
    fn get_target_options(&self, range: ActionRange) -> Vec<&Box<dyn PlayableCharacter>> {
        ActionRange::get_targets(&self.players, self.active_turn_index, range)
            .iter_mut()
            .map(|player| player.get_character())
            .collect()
    }

    fn get_current_character(&self) -> &Box<dyn PlayableCharacter> {
        self.get_current_player().get_character()
    }

    fn get_current_actor(&self) -> &Box<dyn Actor> {
        self.get_current_player().get_actor()
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

    fn resolve_action(&mut self, action: Action) -> Result<Option<Role>, TurnError> {
        match action.action_type {
            ActionType::Damage(damage_type, amount) => {
                self.get_target(action)?.take_damage(amount, damage_type);
            }
            ActionType::Heal(amount) => {
                self.get_target(action)?.heal(amount);
            }
            ActionType::GiveArrows(amount) => {
                self.get_current_character_mut().give_arrows(amount);
            }
            ActionType::DiscardAllArrows => {
                self.get_current_character_mut().discard_all_arrows();
            }
        }

        Ok(self.is_match_terminated())
    }

    // requests the current actor for a target, giving it the action to perform, and validates the response
    fn get_target(&mut self, action: Action) -> Result<&mut Box<dyn PlayableCharacter>, TurnError> {
        let options = self.get_target_options(action.action_range);

        let target_idx = self.get_current_actor().choose_target(&options, &action.action_type);

        if !utils::validate_index(target_idx as i64, &self.players) {
            match self.settings.invalid_action_policy {
                super::settings::InvalidActionPolicy::Ignore => return Err(TurnError::InvalidTarget(target_idx)),
                super::settings::InvalidActionPolicy::MarkAsDead => {
                    self.get_current_character_mut().set_alive(false);
                    return Err(TurnError::InvalidTarget(target_idx));
                }
            }
        }

        Ok(self.players[target_idx].get_character_mut())
    }

    // returns Some(Role) if Role has won, None if match has not ended. If Sheriff / Deputy win, the returned Role is Sheriff
    fn is_match_terminated(&self) -> Option<Role> {
        trace!("checking if match has ended");
        
        // check if sheriff is dead
        let sheriff_probably = self.players.iter().filter(|player| player.role == Role::Sheriff).next();

        let Some(sheriff) = sheriff_probably else {
            error!("Failed to find the sheriff, pretend it never existed and don't end the match");
            return None;
        };

        if !sheriff.character.is_alive() {
            trace!("sheriff is dead, checking if there are any outlaws");
            let any_outlaw_alive = self
                .players
                .iter()
                .filter(|player| player.role == Role::Outlaw)
                .any(|player| player.get_character().is_alive());
            if any_outlaw_alive {
                trace!("there are, outlaws win");
                return Some(Role::Outlaw);
            } else {
                trace!("there are not, renegades win");
                // TODO if I'm not mistaken, if there are more than one renegade alive, the outlaws win by the rules
                // this is both to verify and to think because it seems like a pretty stupid rule, perhaps add it to the settings
                return Some(Role::Renegade);
            }
        }

        let only_good_guys_left = self
            .players
            .iter()
            .filter(|player| player.get_character().is_alive())
            .all(|player| player.role != Role::Sheriff && player.role != Role::Deputy);
        
        if only_good_guys_left {
            trace!("sheriffs and deputies are the only ones alive"); 
            Some(Role::Sheriff) 
        } else {
            trace!("no winner yet"); 
            None 
        }
    }
}

#[derive(Debug)]
pub(crate) enum TurnError {
    CannotFindNext,
    InvalidTarget(usize),
}

#[derive(Debug)]
enum DiceRollPhaseResult {
    Roll(DiceRollResult),
    // it is possible for the match to terminate during a dice roll phase,
    // for example a gatling is rolled and it kills the sheriff, or a 1hp sheriff rolls three dynamites
    Winner(Role),
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
        assert_eq!(*roles.get(&Role::Sheriff).unwrap(), 1);
        assert_eq!(*roles.get(&Role::Deputy).unwrap(), 1);
        assert_eq!(*roles.get(&Role::Outlaw).unwrap(), 2);
        assert_eq!(*roles.get(&Role::Renegade).unwrap(), 1);

        let actors = generate_base_bot_actors(8);
        let roles = count_roles(&MatchController::new(
            Settings::with_players(8),
            actors,
            Box::new(MockShuffler),
        ));
        assert_eq!(*roles.get(&Role::Sheriff).unwrap(), 1);
        assert_eq!(*roles.get(&Role::Deputy).unwrap(), 2);
        assert_eq!(*roles.get(&Role::Outlaw).unwrap(), 3);
        assert_eq!(*roles.get(&Role::Renegade).unwrap(), 2);

        let actors = generate_base_bot_actors(10);
        let roles = count_roles(&MatchController::new(
            Settings::with_players(10),
            actors,
            Box::new(MockShuffler),
        ));
        assert_eq!(*roles.get(&Role::Sheriff).unwrap(), 1);
        assert_eq!(*roles.get(&Role::Deputy).unwrap(), 3);
        assert_eq!(*roles.get(&Role::Outlaw).unwrap(), 4);
        assert_eq!(*roles.get(&Role::Renegade).unwrap(), 2);
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
        assert_eq!(ctr.players[ctr.active_turn_index].role, Role::Sheriff);

        let actors = generate_base_bot_actors(5);
        let ctr = MatchController::new(Settings::default(), actors, Box::new(ReverseShuffler));
        assert_eq!(ctr.players[ctr.active_turn_index].role, Role::Sheriff);
    }

    fn generate_base_bot_actors<'a>(n: usize) -> Vec<Box<dyn Actor>> {
        let actors_template: Vec<BaseBot> = vec![BaseBot {}; n];
        actors_template
            .iter()
            .map(|a| Box::new(a.clone()) as Box<dyn Actor>)
            .collect()
    }
}
