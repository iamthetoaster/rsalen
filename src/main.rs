use std::env;

use crate::map::{Map, MAP};

pub mod map;
pub mod error;

// Symbols for Rsalen:
// ∧ v > < 


fn main() {
    let mut args = env::args();
    // Tossing out the first environment argument.
    let _here =  args.next();
    // It may be useful to print
    // println!("{:?}", here);
    let mut code_file = None;
    let mut map_file = None;
    while let Some(arg) = args.next() {
        // -m options is used to set the map file; otherwise it will be a 10x10 empty grid.
        if arg.trim() == "-m" {
            map_file = args.next();
        } else {
            code_file = Some(arg)
        }
    }
    // Mutating statics is unsafe... unfortunate, but livable.
    unsafe { 
        MAP = Some(
            if let Some(filename) = &map_file {
                Map::from_file(filename)
            } else {
                Map::default()
            }
        );
    }

    // TODO: Implement Rsalen and builtin methods, test them.
    println!("{}, {:?}", code_file.unwrap(), map_file);
}

