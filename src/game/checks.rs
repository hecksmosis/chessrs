use std::{
    fmt::{Display, Formatter, Result as fmtResult},
    ops::Index,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Checks(Vec<bool>);

impl Checks {
    pub fn default() -> Self {
        Checks(vec![false, false])
    }
}

impl FromIterator<bool> for Checks {
    fn from_iter<T: IntoIterator<Item = bool>>(iter: T) -> Self {
        Checks(iter.into_iter().collect())
    }
}

impl<T> Index<T> for Checks
where
    T: Into<usize>,
{
    type Output = bool;

    fn index(&self, index: T) -> &Self::Output {
        &self.0[index.into()]
    }
}

impl Display for Checks {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmtResult {
        if self.0[0] {
            write!(f, "White in check")
        } else if self.0[1] {
            write!(f, "Black in check")
        } else {
            Ok(())
        }
    }
}
