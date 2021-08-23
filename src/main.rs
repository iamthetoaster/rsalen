use std::env;
use logos::Logos;
use world::Map;

use crate::world::Direction;

pub mod error;
pub mod world;
pub mod parse;

fn main() {
    let mut args = env::args();
    // Tossing out the first environment argument (directory)
    let _here =  args.next();
    // It may be useful to print
    // println!("{:?}", here);
    let mut code_file = None;
    let mut map_file = None;
    let (mut start_row, mut start_col) = (0, 0);
    let mut start_dir = Direction::East;
    let mut interpreter_mode = false;

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
            "-i" => {
                interpreter_mode = true
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

    //  TODO: Handle error here better
    match world::place_rsalen(start_row, start_col){
        Ok(_) => (),
        Err(e) => panic!("{}", e),
    };
    world::direct_rsalen(start_dir);

    match code_file {
        Some(_) => {},
        None => if !interpreter_mode { panic!("No input file") },
    }
    // TODO: Implement Rsalen and builtin methods, test them.
    if interpreter_mode {
        loop {
            world::display();
    
            let mut command = Default::default(); 
            match std::io::stdin().read_line(&mut command) {
                Ok(_) => (),
                Err(e) => panic!("{}", e),
            }
            
            match command.trim() {
                "move_forward" => world::move_forward().unwrap(),
                "turn_left" => world::turn_left(),
                "drop_crumb" => world::drop_crumb(),
                "pickup_crumb" => world::pickup_crumb().unwrap(),
                "front_blocked?" => println!("{}", world::front_blocked()),
                "on_crumb?" => println!("{}", world::on_crumb()),
                "quit" => break,
                c => println!("Unknown command: {}", c),
            }
        }
    }

    println!("woggis; procedure {{}}{{}} _pram? .");
    let lex = parse::Token::lexer("woggis; procedure {}{} _pram? .");
    for item in lex {
        println!("{:?}", &item);
    }
}
