use sudoku_solver::state::State;

fn main() {
    env_logger::init();
    let mut puzzle = State::from(
        "301086504046521070500000001400800002080347900009050038004090200008734090007208103",
    );

    match puzzle.solve() {
        Ok(_) => println!("solution: {puzzle}"),
        Err(e) => println!("{e}"),
    }
}
