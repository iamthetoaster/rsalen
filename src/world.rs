use std::{fs::File, io::{self, BufRead}};
use crate::error::*;

// It seems there isn't a simple way to make simple static vec
// This likely needs to be static due to how builtins will work in future, but I think it's an acceptable singleton for this context.
static mut WORLD: World = World {
    map: None, 
    rsalen: Rsalen {
        row: 0,
        col: 0,
        dir: Direction::East,
    }
};


// Atomic Statements
pub fn move_forward() -> Result<(), CrashError> {
    unsafe {
        let direction = WORLD.rsalen.dir;
        let (mut row, mut col) = (
            WORLD.rsalen.row,
            WORLD.rsalen.col,
        );
        match direction {
            Direction::North => row -= 1,
            Direction::West => col -= 1,
            Direction::South => row += 1,
            Direction::East => col += 1,
        };
        WORLD.place_rsalen(row, col)
    }
}

pub fn turn_left() {
    unsafe {
        WORLD.turn_rsalen_left()
    }
}

pub fn drop_crumb() {
    unsafe {
        WORLD.map.as_mut().unwrap().drop_crumb(
            WORLD.rsalen.row,
            WORLD.rsalen.col,
        )
    }
}

pub fn pickup_crumb() -> Result<(), CrumbPickupError> {
    unsafe {
        WORLD.map.as_mut().unwrap().pickup_crumb(
            WORLD.rsalen.row,
            WORLD.rsalen.col,
        )
    }
}

// Atomic Queries
pub fn front_blocked() -> bool {
    unsafe {
        let direction = WORLD.rsalen.dir;
        let (mut row, mut col) = (
            WORLD.rsalen.row,
            WORLD.rsalen.col,
        );
        match direction {
            Direction::North => row -= 1,
            Direction::West => col -= 1,
            Direction::South => row += 1,
            Direction::East => col += 1,
        };
        match WORLD.map.as_ref().unwrap().at(row, col) {
            Ok(Tile::Empty) | Ok(Tile::Crumbs(_)) => false,
            _ => true,
        }
    }
}

pub fn on_crumb() -> bool {
    unsafe {
        let (row, col) = (
            WORLD.rsalen.row,
            WORLD.rsalen.col,
        );
        match WORLD.map.as_ref().unwrap().at(row, col) {
            Ok(Tile::Crumbs(_)) => true,
            _ => false,
        }
    }
}

// methods not relevant to Rsalen language but worth exporting
pub fn place_rsalen(row: usize, col: usize) -> Result<(), CrashError> {
    unsafe {
        WORLD.place_rsalen(row, col)
    }
}

pub fn direct_rsalen(dir: Direction) {
    unsafe {
        WORLD.direct_rsalen(dir)
    }
}

pub fn set_map(map: Map) {
    unsafe {
        WORLD.give_map(map)
    }
}

pub fn display() {
    unsafe {
        WORLD.display();
    }
}


// TODO: Consider moving methods all the way out into public functions.
struct World {
    pub map: Option<Map>,
    rsalen: Rsalen,
}

impl World {
    pub fn give_map(&mut self, map: Map) {
        self.map = Some(map);
    }

    pub fn place_rsalen(&mut self, row: usize, col: usize) -> Result<(), CrashError> {
        let destination = self.map.as_ref().unwrap().at(row, col);
        match destination {
            Ok(Tile::Wall) => Err(CrashError{ row, col, cause: CrashCause::Wall }),
            Err(_) => Err(CrashError{ row, col, cause: CrashCause::OutOfBounds }),
            _ => {
                self.rsalen.row = row;
                self.rsalen.col = col;
                Ok(())
            }
        }
    }
    
    pub fn direct_rsalen(&mut self, dir: Direction) {
        self.rsalen.dir = dir;
    }

    fn turn_rsalen_left(&mut self) {
        self.rsalen.dir = match &self.rsalen.dir {
            Direction::North => Direction::West,
            Direction::West => Direction::South,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
        };
    }

    pub fn display(&self) {
        println!("");
        let (r_row, r_col) = (self.rsalen.row, self.rsalen.col);
        for (row, row_vec) in self.map.as_ref().unwrap().contents.iter().enumerate() {
            for (col, element) in row_vec.iter().enumerate() {
                print!("{} ", if row == r_row && col == r_col {
                    // Symbols for Rsalen:
                    // ∧ v > < 
                    match self.rsalen.dir {
                        Direction::North => "∧",
                        Direction::West => "<",
                        Direction::South => "v",
                        Direction::East => ">",
                    }
                } else {
                    match element {
                        Tile::Empty => " ",
                        Tile::Wall => "X",
                        Tile::Crumbs(_) => ".",
                    }
                });
            }
            println!("");
        }
    }
}

struct Rsalen {
    row: usize,
    col: usize,
    dir: Direction,
}

#[derive(Debug,  Clone, Copy)]
pub enum Direction {
    North, West, South, East,
}

pub struct Map {
    pub rows: usize,
    pub cols: usize,
    contents: Vec<Vec<Tile>>,
}

// Map related code!

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
                let i = i + 1;
                cols = cols.max(i);
                row.push( match char {
                    'x'|'X'=> Tile::Wall,
                    ' ' => Tile::Empty,
                    '.' => Tile::Crumbs(1),
                    '1'..='9' => Tile::Crumbs(char as u32 - '0' as u32),
                    _ => panic!("Invalid character \"{}\" on line {}, column {}", char, rows, i)
                })
            }
            contents.push(row);
        }

        for row in &mut contents {
            while row.len() < cols {
                row.push(Tile::Empty);
            }
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

