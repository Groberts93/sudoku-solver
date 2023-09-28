use sudoku_solver::state::State;

fn main() {
    let puzzle =
        "301086504046521070500000001400800002080347900009050038004090200008734090007208103";
    print_puzzle(puzzle);
}

fn print_puzzle(puzzle: &str) {
    assert!(puzzle.len() == 9 * 9);

    let out = (0..9)
        .map(|i| format!("{}", &puzzle[i * 9..(i + 1) * 9]))
        .collect::<Vec<String>>()
        .join("\n");

    let state = State::from(puzzle);

    println!("{state:?}");

    println!("{out}");
}
