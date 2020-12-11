use crate::utils::print_day_banner;
use crate::utils::read_lines;
use std::collections::HashSet;
use std::iter::FromIterator;

pub fn day05_01() {
    print_day_banner(5, 1);

    let seats = load_seats();
    let max_seat_id = get_seat_ids(&seats).max().unwrap();
    println!("Max seat ID: {}", max_seat_id)
}

pub fn day05_02() {
    print_day_banner(5, 2);
    let seats = load_seats();
    let seat_ids_set: HashSet<u16> = HashSet::from_iter(get_seat_ids(&seats));
    let min_seat_id = seat_ids_set.iter().min().unwrap();
    let max_seat_id = seat_ids_set.iter().max().unwrap();
    let all_seat_ids: HashSet<u16> = HashSet::from_iter(*min_seat_id..=*max_seat_id);
    let missing_seat_ids = all_seat_ids.difference(&seat_ids_set);
    println!("Missing seat id: {}", missing_seat_ids.last().unwrap());
}

pub fn get_seat_ids(seats: &Vec<String>) -> impl Iterator<Item = u16> + '_ {
    return seats.iter().map(get_seat_id);
}

pub fn get_seat_id(seat: &String) -> u16 {
    let mut row_min = 0;
    let mut row_max = 127;
    let mut col_min = 0;
    let mut col_max = 7;

    for seat_char in seat.chars() {
        let row_mid = row_min + (row_max - row_min) / 2;
        let col_mid = col_min + (col_max - col_min) / 2;
        match seat_char {
            'F' => row_max = row_mid,
            'B' => row_min = row_mid + 1,
            'L' => col_max = col_mid,
            'R' => col_min = col_mid + 1,
            _ => {}
        }
    }

    return row_min * 8 + col_min;
}

fn load_seats() -> Vec<String> {
    return read_lines("./inputs/05")
        .unwrap()
        .filter_map(Result::ok)
        .collect();
}
