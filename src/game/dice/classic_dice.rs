use super::dice_face::DiceFace;
use super::dice::Dice;


pub(crate) struct ClassicDice {
    pub(crate) faces: Vec<DiceFace>
}


impl Dice for ClassicDice{
    fn throw(amount: u16) -> Vec<(DiceFace, u16)> {
        let mut res:Vec<(DiceFace, u16)> = Vec::new();
        res.push((DiceFace::ARROW, amount));
        res
    }
}