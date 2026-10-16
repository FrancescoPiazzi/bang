use serde::Deserialize;

#[derive(Deserialize)]
pub(crate) struct Settings {
    pub(crate) n_players: usize, // TODO: remove this, infer it from actors len

    pub(crate) max_rerolls: u16,

    pub(crate) dice_thrown: usize,
    pub(crate) dice_throw_type: DiceThrowType,

    pub(crate) invalid_action_policy: InvalidActionPolicy,
}

impl Settings {
    pub(crate) fn with_players(players: usize) -> Settings {
        Settings {
            n_players: players,

            max_rerolls: 2,

            dice_thrown: 5,
            dice_throw_type: DiceThrowType::AutomaticForBots,

            invalid_action_policy: InvalidActionPolicy::Ignore,
        }
    }

    pub(crate) fn new(
        players: usize,
        max_rerolls: u16,
        dice_thrown: usize,
        dice_throw_type: DiceThrowType,
        invalid_action_policy: InvalidActionPolicy,
    ) -> Settings {
        Settings {
            n_players: players,

            max_rerolls: max_rerolls,

            dice_thrown: dice_thrown,
            dice_throw_type: dice_throw_type,

            invalid_action_policy: invalid_action_policy,
        }
    }
}

impl Default for Settings {
    fn default() -> Settings {
        Settings {
            n_players: 8,

            max_rerolls: 2,

            dice_thrown: 5,
            dice_throw_type: DiceThrowType::AutomaticForBots,

            invalid_action_policy: InvalidActionPolicy::Ignore,
        }
    }
}

#[derive(Deserialize)]
pub(crate) enum DiceThrowType {
    Automatic,
    AutomaticForBots,
    Manual,
}

#[derive(Deserialize)]
pub(crate) enum InvalidActionPolicy {
    Ignore,
    MarkAsDead,
}
