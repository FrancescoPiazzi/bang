mod game;
mod players;
mod ui;

use crate::game::game_controller::GameController;


fn main() {
    colog::default_builder().filter_level(log::LevelFilter::Trace).init();

    let mut controller = GameController::new();
    controller.run_match();
}
