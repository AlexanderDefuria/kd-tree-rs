#[derive(Debug, PartialEq)]
pub enum Dim {
    X,
    Y,
}

impl Dim {
    const DIMENSIONS: usize = 2;

    pub(crate) fn from_depth(mut n: usize) -> Dim {
        n = n.rem_euclid(Dim::DIMENSIONS);
        assert!(n < Dim::DIMENSIONS);
        match n {
            0 => Dim::X,
            1 => Dim::Y,
            _ => panic!("Higher dimensions not implemented"),
        }
    }
}
