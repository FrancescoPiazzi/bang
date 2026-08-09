mod game;
mod players;
mod ui;

use log::{debug, error, info, trace, warn};

fn main() {
    colog::default_builder().filter_level(log::LevelFilter::Trace).init();

    error!("🔥🔥🔥 error");
    warn!("warn");
    info!("info");
    debug!("debug");
    trace!("trace\non two lines\nor maybe three");
}
