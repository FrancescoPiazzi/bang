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
