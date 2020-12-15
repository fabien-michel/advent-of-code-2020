use crate::utils::print_day_banner;
use crate::utils::read_lines;

#[derive(Debug)]
struct Instruction {
    action: char,
    value: isize,
}

pub fn day12_01() {
    print_day_banner(12, 1);
    let instructions = load_instructions();
    let mut x: isize = 0;
    let mut y: isize = 0;
    let mut orientation: isize = 0;

    for instruction in instructions {
        match instruction.action {
            'N' => y += instruction.value,
            'S' => y -= instruction.value,
            'E' => x += instruction.value,
            'W' => x -= instruction.value,
            'L' => orientation = new_orientation(orientation, -instruction.value),
            'R' => orientation = new_orientation(orientation, instruction.value),
            'F' => {
                let (nx, ny) = move_forward(x, y, orientation, instruction.value);
                x = nx;
                y = ny;
            }
            _ => {
                panic!()
            }
        }
    }
    let distance = x.abs() + y.abs();
    println!("Distance: {}", distance);
}

fn new_orientation(orientation: isize, degrees: isize) -> isize {
    let degrees = if degrees < 0 { degrees + 360 } else { degrees };
    (orientation + degrees) % 360
}

fn move_forward(x: isize, y: isize, orientation: isize, value: isize) -> (isize, isize) {
    match orientation {
        0 => return (x + value, y),
        90 => return (x, y - value),
        180 => return (x - value, y),
        270 => return (x, y + value),
        _ => panic!(),
    }
}

pub fn day12_02() {
    print_day_banner(12, 2);
    let instructions = load_instructions();
    let mut ship_x: isize = 0;
    let mut ship_y: isize = 0;
    let mut wp_x: isize = 10;
    let mut wp_y: isize = 1;
    for instruction in instructions {
        match instruction.action {
            'N' => wp_y += instruction.value,
            'S' => wp_y -= instruction.value,
            'E' => wp_x += instruction.value,
            'W' => wp_x -= instruction.value,
            'L' => {
                let (nx, ny) = rotate_waypoint(wp_x, wp_y, -instruction.value);
                wp_x = nx;
                wp_y = ny;
            }
            'R' => {
                let (nx, ny) = rotate_waypoint(wp_x, wp_y, instruction.value);
                wp_x = nx;
                wp_y = ny;
            }
            'F' => {
                ship_x += wp_x * instruction.value;
                ship_y += wp_y * instruction.value;
            }
            _ => {
                panic!()
            }
        }
        // println!("{}{} > ({},{}) ({}, {})", instruction.action, instruction.value, ship_x, ship_y, wp_x, wp_y);
    }
    let distance = ship_x.abs() + ship_y.abs();
    println!("Distance: {}", distance);
}

fn rotate_waypoint(wp_x: isize, wp_y: isize, degrees: isize) -> (isize, isize) {
    return match degrees {
        0 => (wp_x, wp_y),
        90 | -270 => (wp_y, -wp_x),
        180 | -180 => (-wp_x, -wp_y),
        270 | -90 => (-wp_y, wp_x),
        _ => panic!(),
    };
}

fn load_instructions() -> Vec<Instruction> {
    return read_lines("./inputs/12")
        .unwrap()
        .filter_map(Result::ok)
        .map(parse_line)
        .collect();
}

fn parse_line(line: String) -> Instruction {
    return Instruction {
        action: line.chars().nth(0).unwrap(),
        value: line.as_str()[1..].parse::<isize>().unwrap(),
    };
}
