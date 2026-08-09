use std::collections::HashMap;

use crate::game::role::Role;

pub(crate) struct PlayerData {
    name: String,

    win_lose: HashMap<Role, (u16, u16)>, // role -> (wins, losses)
}
