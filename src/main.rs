#![feature(once_cell)]
#![feature(string_remove_matches)]

pub mod file_utils;
pub mod parser;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        println!("Please pass name of the file as argument");
        return
    }

    std::process::exit(match parser::run(&args[1]) {
        Ok(_) => 0,
        Err(err) => {
            eprintln!("error: {:?}", err);
            1
        }
    });
}
