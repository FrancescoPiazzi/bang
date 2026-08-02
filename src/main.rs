mod game;

use log::{error, info, debug, trace, warn};

use crate::game::characters::character::Character;
use crate::game::characters::characters::SuzieLafette;

fn main() {
    colog::default_builder().filter_level(log::LevelFilter::Trace).init();

    let mut character = SuzieLafette::new(9);

    character.give_arrows(10000);
    character.activate_arrows();

    if !character.is_alive() {
        println!("yay she's fucking dead");

        error!("🔥🔥🔥 error");
        warn!("warn");
        info!("info");
        debug!("debug");
        trace!("trace\non two lines\nor maybe three");
    }
}
