use state::State;

pub mod constraints;
pub mod state;

pub struct Config {
    puzzle: State,
}

impl From<String> for Config {
    fn from(puzzle: String) -> Self {
        Config {
            puzzle: State::from(puzzle.as_str()),
        }
    }
}

pub fn run(mut config: Config) {
    match config.puzzle.solve() {
        Ok(_) => println!("solution: {}", config.puzzle),
        Err(e) => println!("{e}"),
    }
}
