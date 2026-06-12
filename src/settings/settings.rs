struct Settings {
    n_players: u8,

    dice_thrown: u32,
    dice_throw_type: DiceThrow,
}


impl Default for Settings {

}


enum DiceThrow {
    AUTOMATIC,
    AUTOMATIC_FOR_BOTS,
    MANUAL
}