use super::die_face::DiceFace;

/* Temporary state of a die */
#[derive(Clone, Copy, Debug)]
pub(crate) struct DieState {
    dice_face: DiceFace,
    is_throwable: bool,
    // TODO: is_locked_by_player or something among the lines to not rethrow dices the player wants to keep
}

impl DieState {
    pub(crate) fn new(dice_face: DiceFace) -> DieState {
        DieState {
            dice_face: dice_face,
            is_throwable: DieState::is_face_rethrowable(dice_face),
        }
    }

    pub(crate) fn get_face(&self) -> DiceFace {
        self.dice_face
    }

    pub(crate) fn set_face(&mut self, dice_face: DiceFace) {
        self.dice_face = dice_face;
        self.is_throwable = DieState::is_face_rethrowable(dice_face);
    }

    pub(crate) fn is_throwable(&self) -> bool {
        self.is_throwable
    }

    fn is_face_rethrowable(dice_face: DiceFace) -> bool {
        dice_face != DiceFace::Dynamite // RULE 4.2
    }
}
