use super::dice_face::DiceFace;

pub(crate) struct DiceState {
    pub(crate) face: DiceFace,
    pub(crate) throwable: bool,
}

impl DiceState {
    pub(crate) fn new(face: DiceFace) -> DiceState {
        DiceState {
            face: face,
            throwable: false,
        }
    }
}
