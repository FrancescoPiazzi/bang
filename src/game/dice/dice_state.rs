use super::dice_face::DiceFace;

pub(crate) struct DiceState {
    pub(crate) dice_face: DiceFace,
    pub(crate) is_throwable: bool,
}

impl DiceState {
    pub(crate) fn new(dice_face: DiceFace) -> DiceState {
        DiceState {
            dice_face: dice_face,
            is_throwable: DiceState::is_face_rethrowable(dice_face),
        }
    }

    pub(crate) fn set_face(&mut self, dice_face: DiceFace) {
        self.dice_face = dice_face;
        self.is_throwable = DiceState::is_face_rethrowable(dice_face);
    }

    pub(crate) fn is_throwable(&self) -> bool{
        self.is_throwable
    }

    fn is_face_rethrowable(dice_face: DiceFace) -> bool{
        dice_face != DiceFace::Dynamite     // RULE 4.2
    }
}
