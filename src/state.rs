use std::{collections::HashSet, fmt::Display};

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

#[derive(Debug, PartialEq)]
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

    fn allow(&mut self, n: u8) -> bool {
        self.state.insert(n)
    }

    fn deny(&mut self, n: u8) -> bool {
        self.state.remove(&n)
    }
}

impl Display for GridCell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out = vec![];
        for ii in 1..=9 {
            let str = match self.state.get(&ii) {
                Some(i) => i.to_string(),
                None => "·".to_string(),
            };
            out.push(format!("{} ", str));
            if ii % 3 == 0 {
                out.push("\n".to_string());
            }
        }

        let out: String = out.into_iter().collect();
        write!(f, "{}", out)
    }
}

impl From<Vec<u8>> for GridCell {
    fn from(value: Vec<u8>) -> Self {
        GridCell {
            state: HashSet::from_iter(value.into_iter()),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::state::GridCell;

    #[test]
    fn can_display_gridcell() {
        let mut gridcell = GridCell::new_collapsed(5);
        gridcell.allow(2);
        println!("{gridcell}");
    }

    #[test]
    fn can_alter_gridcell() {
        let mut gridcell = GridCell::new_collapsed(7);
        gridcell.allow(6);
        gridcell.allow(8);
        gridcell.deny(6);
        assert_eq!(gridcell, GridCell::from(vec![7, 8]));
    }
}
