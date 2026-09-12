#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug, PartialOrd, Ord)]
pub(crate) enum DieFace {
    Shoot1,
    Shoot2,
    Beer,
    Arrow,
    Dynamite,
    Gatling,

    NonBlockingDynamite,
    HealingGatling,
    Shoot1or2,
    Shoot2or3,
    // DoubleShot,
    // DobleBeer,
}

impl DieFace {
    pub(crate) fn is_face_locking(&self) -> bool {
        match *self {
            DieFace::Dynamite => true, // rule 4.2.1
            _ => false,
        }
    }
}
