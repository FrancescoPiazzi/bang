pub(crate) enum DiceFace{
    SHOOT_1,
    SHOOT_2,
    BEER,
    ARROW,
    DYNAMITE(fn() -> (), fn() -> ()),
    GATLING(fn() -> (), fn() -> ())
    // DOUBLE_SHOT
    // DOUBLE_BEER
}
