use std::collections::HashSet;

use super::die_face::DieFace;
use super::die_state::DieState;

/* A die with an optional state */
#[derive(Clone, Debug)]
pub(crate) struct StatedDie {
    pub(crate) faces: HashSet<DieFace>,
    pub(crate) state: Option<DieState>,
}
