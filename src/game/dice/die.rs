use std::collections::HashSet;

use super::dice_face::DiceFace;
use super::dice_state::DiceState;


pub(crate) struct Die{
    pub(crate) faces: HashSet<DiceFace>, 
    pub(crate) state: Option<DiceState>
}