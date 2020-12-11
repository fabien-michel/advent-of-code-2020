use std::fs::File;
use std::fs::read_to_string;
use std::io::{self, BufRead};
use std::path::Path;

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn read_file<P>(filename: P) -> String
where
    P: AsRef<Path>,
{
    read_to_string(filename).unwrap()
}


pub fn print_day_banner(day_num: i32, puzzle_number:i8) {
    println!("🌲🌲🌲🌲🌲🌲🌲🌲🌲🌲🌲🌲");
    println!("        Day {}#{}\n", day_num, puzzle_number);
}