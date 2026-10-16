use log::warn;

use crate::game::{
    role::Role::{Deputy, Outlaw, Renegade, Sheriff},
    shuffler::Shuffler,
};

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
pub(crate) enum Role {
    Sheriff,
    Deputy,
    Outlaw,
    Renegade,
}

impl Role {
    pub(crate) fn roles_by_n_players(n: usize, shuffler: &Box<dyn Shuffler>) -> Vec<Role> {
        let mut roles = Role::roles_by_n_players_inner(n);
        shuffler.shuffle_roles(&mut roles);
        roles
    }

    fn roles_by_n_players_inner(n: usize) -> Vec<Role> {
        // rule 2.1
        match n {
            0 => {
                warn!("requested roles for 0 players, not much of a game is it?");
                Vec::new()
            }
            1 => {
                warn!("requested roles for 1 player, having friends is a requirement to play this game");
                Vec::from([Sheriff])
            }
            2 => {
                warn!("requested roles for 2 players, having friends (plural) is a requirement to play this game");
                Vec::from([Sheriff, Outlaw])
            }
            3 => {
                warn!(
                    "requested roles for 3 players, official rules cover this differently but I can't be bothered for now"
                );
                Vec::from([Sheriff, Outlaw, Renegade])
            }
            4 => Vec::from([Sheriff, Outlaw, Outlaw, Renegade]),
            5 => Vec::from([Sheriff, Deputy, Outlaw, Outlaw, Renegade]),
            6 => Vec::from([Sheriff, Deputy, Outlaw, Outlaw, Outlaw, Renegade]),
            7 => Vec::from([Sheriff, Deputy, Deputy, Outlaw, Outlaw, Outlaw, Renegade]),
            8 => Vec::from([Sheriff, Deputy, Deputy, Outlaw, Outlaw, Outlaw, Renegade, Renegade]),
            _ => {
                warn!(
                    "requested roles for a shit ton of people, official rules don't cover this, I'll make up something"
                );

                let res = Role::roles_by_n_players_inner(8);
                let roles_to_add = [Outlaw, Deputy, Renegade].into_iter().cycle();

                res.into_iter().chain(roles_to_add.take(n - 8)).collect()
            }
        }
    }
}
