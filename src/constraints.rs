use csv::{ByteRecord, ReaderBuilder};

pub struct Constraints {
    inds: Vec<Vec<u8>>,
}

impl Constraints {
    pub fn new() -> Self {
        let reader = ReaderBuilder::new()
            .has_headers(false)
            .from_reader(include_bytes!("../assets/constraints.csv").as_slice());

        let records: Vec<Vec<u8>> = reader
            .into_records()
            .map(|x| {
                x.expect("should be static csv")
                    .into_iter()
                    .map(|y| y.parse::<u8>().expect("should be decodable as u8"))
                    .collect()
            })
            .collect();

        Constraints { inds: records }
    }

    pub fn get_constrained_inds(&self, ind: u8) -> &[u8] {
        self.inds[ind as usize].as_slice()
    }
}

#[cfg(test)]
mod test {
    use super::Constraints;

    #[test]
    fn can_read_constraints() {
        let c = Constraints::new();

        assert_eq!(c.get_constrained_inds(0).len(), 20);

        assert_eq!(c.get_constrained_inds(5)[7], 8);
        assert_eq!(c.get_constrained_inds(2)[3], 4);
        assert_eq!(c.get_constrained_inds(0)[0], 1);
        assert_eq!(c.get_constrained_inds(19)[11], 24);
    }
}
