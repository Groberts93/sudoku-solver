use crate::constraints::Constraints;
use anyhow::Result;
use log::info;
use std::{collections::HashSet, fmt::Display};
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
        let mut applied_inds: HashSet<usize> = HashSet::new();
        let mut iteration = 0;

        while applied_inds.len() != 81 {
            let inds: Vec<usize> = self
                .find_fully_constrained_inds()
                .into_iter()
                .filter(|x| !applied_inds.contains(x))
                .collect();
            let mut inds = inds.into_iter();

            info!(
                "beginning iteration {}, entropy: {}, applied: {}",
                iteration,
                self.total_entropy(),
                applied_inds.len()
            );

            while let Some(index) = inds.next() {
                let val = self
                    .cells
                    .get(index)
                    .expect("should be valid")
                    .determined_value()
                    .expect("should be determined");
                self.apply_constraints(val, index)?;

                applied_inds.insert(index);
            }
            iteration += 1;
        }

        Ok(())
    }

    fn find_fully_constrained_inds(&self) -> Vec<usize> {
        self.cells
            .iter()
            .enumerate()
            .filter(|(_, c)| c.entropy() == 1)
            .map(|(i, _)| i)
            .collect()
    }

    fn total_entropy(&self) -> u32 {
        self.cells.iter().map(|x| x.entropy() as u32).sum()
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

    #[allow(dead_code)]
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
    fn can_solve() {
        // case 1: valid
        let mut state = State::from(
            "301086504046521070500000001400800002080347900009050038004090200008734090007208103",
        );

        assert_eq!(state.solve(), Ok(()));
        assert_eq!(
            format!("{state}"),
            "371986524846521379592473861463819752285347916719652438634195287128734695957268143"
                .to_string()
        );

        // case 2: valid
        let mut state = State::from(
            "000030007480960501063570820009610203350097006000005094000000005804706910001040070",
        );

        assert_eq!(state.solve(), Ok(()));
        assert_eq!(
            format!("{state}"),
            "925831467487962531163574829749618253352497186618325794276189345834756912591243678"
                .to_string()
        );

        // case 3: invalid, edited case 2
        let mut state = State::from(
            "000040007480960501063570820009610203350097006000005094000000005804706910001040070",
        );

        assert_eq!(
            state.solve(),
            Err("cell at index 76 is already fully constrained as 4".to_string())
        );
    }

    #[test]
    fn can_find_constrained_inds() {
        let state = State::from(
            "301086504046521070500000001400800002080347900009050038004090200008734090007208103",
        );

        assert_eq!(
            state.find_fully_constrained_inds(),
            vec![
                0, 2, 4, 5, 6, 8, 10, 11, 12, 13, 14, 16, 18, 26, 27, 30, 35, 37, 39, 40, 41, 42,
                47, 49, 52, 53, 56, 58, 60, 65, 66, 67, 68, 70, 74, 75, 77, 78, 80
            ]
        );
    }
}
