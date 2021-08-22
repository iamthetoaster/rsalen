use std::env;
use world::Map;

use crate::world::Direction;

pub mod error;
pub mod world;



fn main() {
    let mut args = env::args();
    // Tossing out the first environment argument.
    let _here =  args.next();
    // It may be useful to print
    // println!("{:?}", here);
    let mut code_file = None;
    let mut map_file = None;
    let (mut start_row, mut start_col) = (0, 0);
    let mut start_dir = Direction::East;

    while let Some(arg) = args.next() {
        // TODO: Proper error reporting here
        match arg.trim() {
            "-m" => { map_file = args.next(); },
            "-r" => { start_row = args.next().unwrap_or("0".into()).trim().parse().unwrap() }
            "-c" => { start_col = args.next().unwrap_or("0".into()).trim().parse().unwrap() }
            "-p" => { 
                start_row = args.next().unwrap_or("0".into()).trim().parse().unwrap(); 
                start_col = args.next().unwrap_or("0".into()).trim().parse().unwrap(); 
            }
            "-d" => {
                start_dir = match args.next().unwrap_or("e".into()).trim().to_lowercase().as_str() {
                    "n" | "north" => Direction::North,
                    "s" | "south" => Direction::South,
                    "e" | "east" => Direction::East,
                    "w" | "west" => Direction::West,
                    x => panic!("Invalid direction: {}", x)
                };
            }
            _ => { code_file = Some(arg) }
            
        };
    }
    
    world::set_map(
        if let Some(filename) = &map_file {
            Map::from_file(filename)
        } else {
            Map::default()
        }
    );

    world::place_rsalen(start_row, start_col);
    world::direct_rsalen(start_dir);


    // TODO: Implement Rsalen and builtin methods, test them.
    println!("{}, {:?}", code_file.unwrap(), map_file);
    world::display(); 
}

