/* handles an entire game, which may span for multiple matches */

use std::fs::File;
use std::io::Read;
use std::path::Path;

use crate::game::dice::chooser::RandomChooser;
use crate::game::settings::*;

use log::{error, trace};

struct GameController {
    dice_chooser: RandomChooser,
}

impl GameController {
    pub(crate) fn run_match(&mut self) {
        let settings = Self::load_settings_from_file(None);

        // let match_controller = MatchController::new(
        //     settings,
        //     self.dice_chooser,
        //
        // );
    }

    pub(crate) fn load_settings_from_file(name: Option<String>) -> Settings {
        if let Some(name_inner) = name {
            // not gonna bother with combining paths with PathBuf
            let path_str = String::from("settings_profiles/") + &name_inner;

            let mut file = match File::open(Path::new(&path_str)) {
                Ok(file) => file,
                Err(why) => {
                    error!("io error when loading settings: {}, loading defaults instead", why);
                    return Settings::default();
                }
            };

            let mut s = String::new();
            if let Err(why) = file.read_to_string(&mut s) {
                error!("failed to open file reason: {}, loading defaults instead", why);
                return Settings::default();
            }

            let settings: Settings = match serde_json::from_str(&s) {
                Ok(settings) => settings,
                Err(error) => {
                    error!(
                        "incorrect format when loading settings: {}, loading defaults instead",
                        error
                    );
                    return Settings::default();
                }
            };

            settings
        } else {
            trace!("no settings file specified, loading defaults");
            Settings::default()
        }
    }
}
