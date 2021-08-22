use core::fmt;
use crate::world::WORLD;

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

pub struct CrashError {
    pub row: usize,
    pub col: usize
}

// TODO: Make the crash error contain a reason. Probably make CrashReason enum. Not doing it now because I can't be bothered and it isn't important.
// This will allow you to un-pub World::map in world.rs

impl fmt::Display for CrashError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let cause = unsafe { match &WORLD.map {
            Some(it) => it,
            _ => unreachable!(),
        }.at(self.row, self.col) };
        let cause = match cause {
            Ok(_) => "wall",
            Err(_) => "edge of world"
        };
        write!(f, "Attempted to move Rsalen to illegal location: {}, column: {}. Crashed into {}.", &self.row, &self.col, cause)
    }
}

