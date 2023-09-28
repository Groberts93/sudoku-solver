use std::collections::HashSet;

#[derive(Debug)]
pub struct State {
    cells: Vec<GridCell>,
}

impl From<&str> for State {
    fn from(value: &str) -> Self {
        let mut cells = vec![];
        for char in value.chars() {
            let digit = char.to_digit(10).expect("input should be digits only");
            if digit == 0 {
                cells.push(GridCell::new())
            } else {
                cells.push(GridCell::new_collapsed(digit as u8))
            }
        }

        State { cells: cells }
    }
}

#[derive(Debug)]
struct GridCell {
    state: HashSet<u8>,
}

impl GridCell {
    fn new() -> Self {
        GridCell {
            state: HashSet::from_iter(1..=9),
        }
    }

    fn new_collapsed(n: u8) -> Self {
        GridCell {
            state: HashSet::from_iter(n..=n),
        }
    }
}
