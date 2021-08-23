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

#[derive(Debug, Clone)]
pub struct CrumbPickupError {
    pub row: usize,
    pub col: usize
}

impl fmt::Display for CrumbPickupError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Attempted to pickup crumb at at row: {}, column: {}, where none were present.", &self.row, &self.col)
    }
}

#[derive(Debug, Clone)]
pub enum CrashCause {
    Wall, OutOfBounds,
}

#[derive(Debug, Clone)]
pub struct CrashError {
    pub row: usize,
    pub col: usize,
    pub cause: CrashCause,

}

impl fmt::Display for CrashError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let cause = match self.cause {
            CrashCause::Wall => "wall",
            CrashCause::OutOfBounds => "world boundary",
        };
        write!(f, "Attempted to move Rsalen to illegal location: {}, column: {}. Crashed into {}.", &self.row, &self.col, cause)
    }
}

