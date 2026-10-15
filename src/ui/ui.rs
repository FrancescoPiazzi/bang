use std::io;

static TERMINATE_INPUT: &str = "-";

struct ui {
    lang: String,
}

impl ui {
    fn get_player_names() -> Vec<String> {
        let mut buffer = String::new();
        let stdin = io::stdin();

        print!("Specify player names, type '{}' to finish", TERMINATE_INPUT);

        stdin.read_line(&mut buffer).unwrap();

        todo!()
    }
}
