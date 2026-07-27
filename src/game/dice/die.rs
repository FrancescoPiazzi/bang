use std::collections::HashSet;

use super::die_face::DiceFace;
use super::die_state::DieState;

/* A die with an optional state */
#[derive(Debug)]
pub(crate) struct StatedDie {
    pub(crate) faces: HashSet<DiceFace>,
    pub(crate) state: Option<DieState>,
}
