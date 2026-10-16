use crate::players::player::Player;

pub(crate) fn loop_index(mut idx: i64, len: usize) -> i64 {
    if idx >= len as i64 {
        idx = 0;
    }
    if idx < 0 {
        idx = len as i64 - 1;
    }

    idx
}

// returns true if index points to a valid character that is alive
pub(crate) fn validate_index(idx: i64, characters: &[Player]) -> bool {
    idx >= 0 && idx < characters.len() as i64 && characters[idx as usize].get_character().is_alive()
}
