#[derive(PartialEq, Eq, Clone, Copy, Hash)]
pub(crate) enum DiceFace {
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
