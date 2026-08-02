#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub(crate) enum DieFace {
    Shoot1,
    Shoot2,
    Beer,
    Arrow,
    Dynamite,
    Gatling,

    NonBlockingDynamite,
    HealingGatling,
    // DoubleShot
    // DobleBeer
}


impl DieFace {
    pub(crate) fn is_face_locking(&self) -> bool {
        match *self {
            DieFace::Dynamite => true,
            _ => false,
        }
    }
}