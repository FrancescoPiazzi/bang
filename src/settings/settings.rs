pub(crate) struct Settings {
    n_players: u8,

    dice_thrown: u32,
    dice_throw_type: DiceThrowType,
}


impl Default for Settings {
    fn default() -> Settings {
        Settings {
            n_players: 8,

            dice_thrown: 6,
            dice_throw_type: AUTOMATIC_FOR_BOTS
        }
    }
}


enum DiceThrowType {
    AUTOMATIC,
    AUTOMATIC_FOR_BOTS,
    MANUAL
}