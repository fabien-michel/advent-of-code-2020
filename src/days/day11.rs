use crate::utils::print_day_banner;
use crate::utils::read_lines;

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
enum Location {
    Floor,
    Empty,
    Occupied,
}

type SeatsMap = Vec<Vec<Location>>;

const DIRECTIONS_SET: [(isize, isize); 8] = [
    (-1, -1), // Top Left
    (0, -1),  // Top
    (1, -1),  // Top Right
    (-1, 0),  // Left
    (1, 0),   // Right
    (-1, 1),  // Bottom Left
    (0, 1),   // Bottom
    (1, 1),   // Bottom Right
];

pub fn day11_01() {
    print_day_banner(11, 1);
    let mut seats_map = load_seats_map();
    let mut previous_occupied_seats = 0;
    loop {
        // print!(".");
        // io::stdout().flush().unwrap();
        let (new_seats_map, occupied_seats) = do_iteration(&seats_map, rules_01);
        seats_map = new_seats_map;
        if occupied_seats == previous_occupied_seats {
            break;
        }
        previous_occupied_seats = occupied_seats;
    }
    // print_seats_map(&seats_map);
    println!("Occupied seats: {}", previous_occupied_seats);
}

pub fn day11_02() {
    print_day_banner(11, 2);
    let mut seats_map = load_seats_map();
    let mut previous_occupied_seats = 0;
    loop {
        // print!(".");
        // io::stdout().flush().unwrap();
        let (new_seats_map, occupied_seats) = do_iteration(&seats_map, rules_02);
        seats_map = new_seats_map;
        if occupied_seats == previous_occupied_seats {
            break;
        }
        previous_occupied_seats = occupied_seats;
    }
    println!("Occupied seats: {}", previous_occupied_seats);
}

fn do_iteration(
    seats_map: &SeatsMap,
    rules: fn(seats_map: &SeatsMap, x: usize, y: usize, location: &Location) -> Location,
) -> (SeatsMap, u32) {
    let mut new_seats_map: SeatsMap = vec![];
    let mut occupied_seats: u32 = 0;
    for (y, seats_row) in seats_map.iter().enumerate() {
        let mut new_seats_row: Vec<Location> = vec![];
        for (x, location) in seats_row.iter().enumerate() {
            let new_location: Location = rules(seats_map, x, y, location);
            if new_location == Location::Occupied {
                occupied_seats += 1;
            }

            new_seats_row.push(new_location);
        }
        new_seats_map.push(new_seats_row);
    }
    return (new_seats_map, occupied_seats);
}

fn rules_01(seats_map: &SeatsMap, x: usize, y: usize, location: &Location) -> Location {
    if location == &Location::Floor {
        return *location;
    }
    let adjacents_occupied_count = count_occupied_adjacents(&seats_map, x, y, Some(4));
    if location == &Location::Empty && adjacents_occupied_count == 0 {
        return Location::Occupied;
    } else if location == &Location::Occupied && adjacents_occupied_count >= 4 {
        return Location::Empty;
    }
    return *location;
}

fn rules_02(seats_map: &SeatsMap, x: usize, y: usize, location: &Location) -> Location {
    if location == &Location::Floor {
        return *location;
    }
    let visible_occupied_count = count_occupied_visibles(&seats_map, x, y, Some(5));
    if location == &Location::Empty && visible_occupied_count == 0 {
        return Location::Occupied;
    } else if location == &Location::Occupied && visible_occupied_count >= 5 {
        return Location::Empty;
    }
    return *location;
}

fn count_occupied_visibles(seats_map: &SeatsMap, x: usize, y: usize, max: Option<u8>) -> u8 {
    let mut count: u8 = 0;
    for directions in DIRECTIONS_SET.iter() {
        count += find_occupied(seats_map, x, y, directions.0, directions.1);
        if max.is_some() && count == max.unwrap() {
            break;
        }
    }

    return count;
}

fn find_occupied(
    seats_map: &SeatsMap,
    start_x: usize,
    start_y: usize,
    delta_x: isize,
    delta_y: isize,
) -> u8 {
    let mut x: isize = start_x as isize;
    let mut y: isize = start_y as isize;
    loop {
        x += delta_x;
        y += delta_y;
        if x < 0 || y < 0 || y as usize >= seats_map.len() || x as usize >= seats_map[0].len() {
            return 0;
        }
        match get_location(seats_map, x, y) {
            Location::Occupied => return 1,
            Location::Empty => return 0,
            _ => {}
        }
    }
}

fn get_location(seats_map: &SeatsMap, x: isize, y: isize) -> Location {
    if x < 0 || y < 0 || y as usize >= seats_map.len() || x as usize >= seats_map[0].len() {
        return Location::Floor;
    }
    return seats_map[y as usize][x as usize];
}

fn is_occupied(seats_map: &SeatsMap, x: isize, y: isize) -> bool {
    return get_location(seats_map, x, y) == Location::Occupied;
}

fn count_occupied_adjacents(seats_map: &SeatsMap, x: usize, y: usize, max: Option<u8>) -> u8 {
    let mut count: u8 = 0;
    for directions in DIRECTIONS_SET.iter() {
        if is_occupied(
            seats_map,
            x as isize + directions.0,
            y as isize + directions.1,
        ) {
            count += 1;
        }
        if max.is_some() && count == max.unwrap() {
            break;
        }
    }
    return count;
}

// fn print_seats_map(seats_map: &SeatsMap) {
//     for seats_row in seats_map {
//         println!(
//             "{}",
//             seats_row.iter().map(location_char).collect::<String>()
//         );
//     }
// }

// fn location_char(location: &Location) -> char {
//     return match location {
//         Location::Empty => 'L',
//         Location::Occupied => '#',
//         Location::Floor => '.',
//     };
// }

fn load_seats_map() -> SeatsMap {
    return read_lines("./inputs/11")
        .unwrap()
        .filter_map(Result::ok)
        .map(parse_line)
        .collect();
}

fn parse_line(line: String) -> Vec<Location> {
    let mut locations: Vec<Location> = vec![];
    for c in line.chars() {
        let location = match c {
            'L' => Location::Empty,
            '#' => Location::Occupied,
            _ => Location::Floor,
        };
        locations.push(location);
    }
    return locations;
}
