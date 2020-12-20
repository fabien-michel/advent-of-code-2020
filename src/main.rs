// use crate::day01::day01;
#[macro_use] extern crate lazy_static;

use std::time::Instant;


mod days;
mod utils;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let (day_number, puzzle_number) = parse_config(&args);

    let day_runs = match day_number {
        0 => days::get_all_day_runs(),
        _ => days::get_day_runs(day_number, puzzle_number),
    };

    for day_run in day_runs {

        let start = Instant::now();
        
        day_run();

        let elapsed = start.elapsed();
        
        println!("\nElapsed time: {:?}", elapsed);
    }
}

fn parse_config(args: &[String]) -> (usize, usize) {
    if args.len() < 2 {
        return (0, 0);
    }

    let day_number = args[1].parse::<usize>().unwrap();
    let mut puzzle_number = 0;
    if args.len() > 2 {
        puzzle_number = args[2].parse::<usize>().unwrap();
    }

    return (day_number, puzzle_number);
}
