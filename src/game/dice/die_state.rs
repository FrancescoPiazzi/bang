use super::die_face::DieFace;

/* Temporary state of a die */
#[derive(Clone, Copy, Debug)]
pub(crate) struct DieState {
    dice_face: DieFace,
    is_locked_by_rules: bool,
    is_locked_by_player: bool,
}

impl DieState {
    pub(crate) fn new(dice_face: DieFace) -> DieState {
        DieState {
            dice_face: dice_face,
            is_locked_by_rules: dice_face.is_face_locking(),
            is_locked_by_player: false, // RULE 3.1 (must roll all dice the first throw)
        }
    }

    pub(crate) fn get_face(&self) -> DieFace {
        self.dice_face
    }

    pub(crate) fn set_face(&mut self, dice_face: DieFace) {
        self.dice_face = dice_face;
        self.is_locked_by_rules = dice_face.is_face_locking();
    }

    pub(crate) fn get_locked_by_player(&self) -> bool {
        self.is_locked_by_player
    }

    pub(crate) fn set_locked_by_player(&mut self, is_locked_by_player: bool) {
        self.is_locked_by_player = is_locked_by_player;
    }

    pub(crate) fn is_throwable(&self) -> bool {
        !self.is_locked_by_rules && !self.is_locked_by_player
    }
}
