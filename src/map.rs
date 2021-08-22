use std::{fs::File, io::{self, BufRead}};

use crate::error::*;


// Slightly annoying that it need be done this way at the moment, but it seems easier than figuring out how to const it?
// This likely needs to be static due to how builtins will work in future, but I think it's an acceptable singleton for this context.
pub static mut MAP: Option<Map> = None;

pub struct Map {
    pub rows: usize,
    pub cols: usize,
    contents: Vec<Vec<Tile>>,
}

impl Map {
    pub fn from_file(filename: &str) -> Self {
        let mut rows = 0;
        let mut cols = 0;
        let mut contents = Vec::new();

        let file = File::open(filename);
        let file = match file { 
            Ok(f) => f,
            Err(why) => panic!("Couldn't open {}: {}", filename, why),
        };

        for line in io::BufReader::new(file).lines() {
            rows += 1;
            let mut row = Vec::new();
            for (i, char) in line.unwrap().chars().enumerate() {
                cols = cols.max(i);
                row.push( match char {
                    'x'|'X'=> Tile::Wall,
                    ' ' => Tile::Empty,
                    '.' => Tile::Crumbs(1),
                    '1'..='9' => Tile::Crumbs(char as u32 - '0' as u32),
                    _ => panic!("Invalid character \"{}\" on line {}, column {}", char, rows, i + 1, )
                })
            }
            contents.push(row);
        }

        Self{
            rows, 
            cols,
            contents,
        }
    }

    // The below functions will be useful later.

    pub fn at(&self, row: usize, col: usize) -> Result<Tile, OutOfBoundsError> {
        if (0..self.rows).contains(&row) && (0..self.cols).contains(&col) {
            Ok(self.contents[row][col])
        } else {
            Err(OutOfBoundsError{ row, col })
        }
    }

    pub fn drop_crumb(&mut self, row: usize, col: usize) {
        if !((0..self.rows).contains(&row) && (0..self.cols).contains(&col)) {
            return;
        }

        match self.contents[row][col] {
            Tile::Empty => {
                self.contents[row][col] = Tile::Crumbs(1);
            },
            Tile::Crumbs(n) => {
                self.contents[row][col] = Tile::Crumbs(n + 1);
            },
            Tile::Wall => (),
        }
    }

    pub fn pickup_crumb(&mut self, row: usize, col: usize) -> Result<(), CrumbPickupError> {
        if !((0..self.rows).contains(&row) && (0..self.cols).contains(&col)) {
            return Err(CrumbPickupError{row, col});
        }

        match self.contents[row][col] {
            Tile::Crumbs(n) => {
                self.contents[row][col] = if n > 1 { 
                    Tile::Crumbs(n - 1)
                } else {
                    Tile::Empty
                };
                Ok(())
            },
            _ => Err(CrumbPickupError{row, col}),
        }
    }
}

impl Default for Map {
    fn default() -> Self {
        Self {
            rows: 10,
            cols: 10,
            contents: vec![
                vec![Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, ],
                vec![Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, ],
                vec![Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, ],
                vec![Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, ],
                vec![Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, ],
                vec![Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, ],
                vec![Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, ],
                vec![Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, ],
                vec![Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, ],
                vec![Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, ],
            ],
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Tile {
    Empty,
    Wall,
    Crumbs(u32),
}

