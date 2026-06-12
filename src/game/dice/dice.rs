use super::dice_face::DiceFace;


pub(crate) trait Dice{
    fn throw(amount: u16) -> Vec<(DiceFace, u16)>;
} 