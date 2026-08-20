use log::warn;

use crate::game::{
    role::Role::{DEPUTY, OUTLAW, RENEGADE, SHERIFF},
    shuffler::Shuffler,
};

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
pub(crate) enum Role {
    SHERIFF,
    DEPUTY,
    OUTLAW,
    RENEGADE,
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
                Vec::from([SHERIFF])
            }
            2 => {
                warn!("requested roles for 2 players, having friends (plural) is a requirement to play this game");
                Vec::from([SHERIFF, OUTLAW])
            }
            3 => {
                warn!(
                    "requested roles for 3 players, official rules cover this differently but I can't be bothered for now"
                );
                Vec::from([SHERIFF, OUTLAW, RENEGADE])
            }
            4 => Vec::from([SHERIFF, OUTLAW, OUTLAW, RENEGADE]),
            5 => Vec::from([SHERIFF, DEPUTY, OUTLAW, OUTLAW, RENEGADE]),
            6 => Vec::from([SHERIFF, DEPUTY, OUTLAW, OUTLAW, OUTLAW, RENEGADE]),
            7 => Vec::from([SHERIFF, DEPUTY, DEPUTY, OUTLAW, OUTLAW, OUTLAW, RENEGADE]),
            8 => Vec::from([SHERIFF, DEPUTY, DEPUTY, OUTLAW, OUTLAW, OUTLAW, RENEGADE, RENEGADE]),
            _ => {
                warn!(
                    "requested roles for a shit ton of people, official rules don't cover this, I'll make up something"
                );

                let res = Role::roles_by_n_players_inner(8);
                let roles_to_add = [OUTLAW, DEPUTY, RENEGADE].into_iter().cycle();

                res.into_iter().chain(roles_to_add.take(n - 8)).collect()
            }
        }
    }
}
