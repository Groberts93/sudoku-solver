use crate::constraints::Constraints;
use anyhow::{anyhow, Result};
use std::{collections::HashSet, error::Error, fmt::Display};
use thiserror::Error;

#[derive(Error, Debug)]
enum ConstraintError {
    #[error("cell at index {0} is already fully constrained as {1}")]
    Conflict(usize, u8),
}

#[derive(Debug)]
pub struct State {
    cells: Vec<GridCell>,
    constraints: Constraints,
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

        State {
            cells: cells,
            constraints: Constraints::new(),
        }
    }
}

impl State {
    fn total_entropy(&self) -> u32 {
        self.cells.iter().map(|x| x.entropy() as u32).sum()
    }

    fn iter_row(&self, row: usize) -> impl Iterator<Item = &GridCell> {
        self.cells.iter().skip(row * 9).take(9)
    }

    fn iter_col(&self, col: usize) -> impl Iterator<Item = &GridCell> {
        self.cells.iter().skip(col).step_by(9)
    }

    fn iter_block(&self, block: usize) -> impl Iterator<Item = &GridCell> {
        let (row_skip, column_skip) = (block / 3, block % 3);

        let mut inds = vec![];
        let mut out = vec![];
        let mut start = row_skip * 3 * 9 + column_skip * 3;

        for _ in 0..3 {
            for ii in start..start + 3 {
                inds.push(ii);
                out.push(self.cells.get(ii).unwrap());
            }
            start = start + 9;
        }

        out.into_iter()
    }

    fn apply_constraints(&mut self, val: u8, idx: usize) -> Result<(), ConstraintError> {
        // println!("applying constraint {val} from cell at index {idx}");
        let inds = self.constraints.get_constrained_inds(idx);

        for ind in inds {
            let cell = self
                .cells
                .get_mut(*ind)
                .expect("ind should always be valid");

            if !cell.deny(val) {
                return Err(ConstraintError::Conflict(
                    *ind,
                    cell.determined_value().expect("should be determined"),
                ));
            }
        }

        Ok(())
    }

    pub fn solve(&mut self) -> Result<(), String> {
        self.propagate_constraints().map_err(|e| e.to_string())?;

        Ok(())
    }

    fn propagate_constraints(&mut self) -> Result<(), ConstraintError> {
        let mut inds = self.find_fully_constrained_inds().into_iter();

        while let Some(index) = inds.next() {
            // println!("{index}");

            let val = self
                .cells
                .get(index)
                .expect("should be valid")
                .determined_value()
                .expect("should be determined");
            self.apply_constraints(val, index)?;
        }

        Ok(())
    }

    fn find_fully_constrained_inds(&self) -> Vec<usize> {
        self.cells
            .iter()
            .enumerate()
            .filter(|(i, c)| c.entropy() == 1)
            .map(|(i, _)| i)
            .collect()
    }
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let display: String = self
            .cells
            .iter()
            .map(|c| c.determined_value().unwrap_or(0).to_string())
            .collect();

        write!(f, "{}", display)
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
        if self.state.len() == 1 {
            if let Some(_) = self.state.get(&n) {
                return false;
            }
            return true;
        } else {
            self.state.remove(&n);
            return true;
        }
    }

    fn entropy(&self) -> u8 {
        self.state.len() as u8
    }

    fn determined_value(&self) -> Option<u8> {
        if self.state.len() == 1 {
            Some(*self.state.iter().next().unwrap())
        } else {
            None
        }
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
    use crate::state::State;

    #[test]
    fn can_alter_gridcell() {
        let mut gridcell = GridCell::new_collapsed(7);
        gridcell.allow(6);
        gridcell.allow(8);
        gridcell.deny(6);
        assert_eq!(gridcell, GridCell::from(vec![7, 8]));
    }

    #[test]
    fn can_compute_entropy() {
        let mut gridcell = GridCell::new_collapsed(3);
        gridcell.allow(6);
        assert_eq!(gridcell.entropy(), 2);
        gridcell.allow(7);
        assert_eq!(gridcell.entropy(), 3);
        gridcell.deny(3);
        assert_eq!(gridcell.entropy(), 2);
    }

    #[test]
    fn can_compute_total_entropy() {
        let state = State::from(
            "301086504046521070500000001400800002080347900009050038004090200008734090007208103",
        );
        assert_eq!(state.total_entropy(), 417);
        let state = State::from(
            "000030007480960501063570820009610203350097006000005094000000005804706910001040070",
        );
        assert_eq!(state.total_entropy(), 433);
    }

    #[test]
    fn can_iter_row() {
        let state = State::from(
            "301086504046521070500000001400800002080347900009050038004090200008734090007208103",
        );
        let mut iter = state.iter_row(8);
        // for _ in 0..=8 {
        //     println!("{}", iter.next().unwrap());
        // }
    }

    #[test]
    fn can_iter_col() {
        let state = State::from(
            "301086504046521070500000001400800002080347900009050038004090200008734090007208103",
        );
        let mut iter = state.iter_col(1);
        for _ in 0..=8 {
            // println!("{}", iter.next().unwrap());
        }
    }

    #[test]
    fn can_iter_block() {
        //     "
        //     301 086 504
        //     046 521 070
        //     500 000 001

        //     400 800 002
        //     080 347 900
        //     009 050 038

        //     004 090 200
        //     008 734 090
        //     007 208 103",

        let state = State::from(
            "301086504046521070500000001400800002080347900009050038004090200008734090007208103",
        );

        let mut iter = state.iter_block(2);

        assert_eq!(*iter.next().unwrap(), GridCell::new_collapsed(5));
        assert_eq!(*iter.next().unwrap(), GridCell::new());
        assert_eq!(*iter.next().unwrap(), GridCell::new_collapsed(4));
        assert_eq!(*iter.next().unwrap(), GridCell::new());
        assert_eq!(*iter.next().unwrap(), GridCell::new_collapsed(7));
        assert_eq!(*iter.next().unwrap(), GridCell::new());
    }

    #[test]
    fn can_solve() {
        let mut state = State::from(
            "301086504046521070500000001400800002080347900009050038004090200008734090007208103",
        );

        println!("{}", state.total_entropy());

        if let Err(e) = state.solve() {
            println!("{e}");
        }

        println!("{}", state.total_entropy());
        println!("{state}");
    }
}
