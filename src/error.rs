use core::fmt;

#[derive(Debug, Clone)]
pub struct OutOfBoundsError {
    pub row: usize,
    pub col: usize,
}

impl fmt::Display for OutOfBoundsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Position at row: {}, column: {} is beyond this map's boundaries.", &self.row, &self.col)
    }
}

pub struct CrumbPickupError {
    pub row: usize,
    pub col: usize
}

impl fmt::Display for CrumbPickupError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Attempted to pickup crumb at at row: {}, column: {}, where none were present.", &self.row, &self.col)
    }
}