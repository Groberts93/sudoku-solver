use clap::Parser;

use log::LevelFilter;
use sudoku_solver::{self, Config};

#[derive(Parser, Debug)]
struct Cli {
    #[arg(short, long)]
    puzzle: String,

    #[arg(short, long, default_value = "warn")]
    log: LevelFilter,
}

fn main() {
    let cli = Cli::parse();

    env_logger::Builder::new().filter_level(cli.log).init();
    let config = Config::from(cli.puzzle);

    sudoku_solver::run(config);
}
