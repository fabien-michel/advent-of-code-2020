use crate::utils::print_day_banner;
use crate::utils::read_file;
use std::collections::HashMap;
use std::io::{self, Write};
use num_format::{Locale, ToFormattedString};

#[derive(Debug)]
struct Cup {
    next: usize,
    prev: usize,
}

type Cups = HashMap<usize, Cup>;

pub fn day23_01() {
    print_day_banner(23, 1);
    let (mut cups, values) = load_initial_cups();
    // println!("{:?}", cups);
    play_game(&mut cups, values[0], 100);
    println!("{:?}", cups_from_1(&cups));
}

pub fn day23_02() {
    print_day_banner(23, 2);
    let (mut cups, values) = load_initial_cups();
    create_extra_cups(&mut cups, &values, 1_000_000);
    play_game(&mut cups, values[0], 10_000_000);
    let cups_after_1 = cups[&1].next;
    let cups_after_after_1 = cups[&cups_after_1].next;

    println!(
        "{} * {} = {}",
        cups_after_1,
        cups_after_after_1,
        cups_after_1 * cups_after_after_1
    );
}

fn create_extra_cups(cups: &mut Cups, values: &Vec<usize>, quantity: usize) {
    let max = get_max(cups);
    let mut previous_value = values[values.len() - 1];
    cups.get_mut(&previous_value).unwrap().next = max + 1;
    for cup_number in (max + 1)..=quantity {
        cups.insert(
            cup_number,
            Cup {
                next: if cup_number < quantity {
                    cup_number + 1
                } else {
                    values[0]
                },
                prev: previous_value,
            },
        );
        previous_value = cup_number;
    }
    cups.get_mut(&values[0]).unwrap().prev = quantity;
}

fn play_game(cups: &mut Cups, current_value: usize, moves: usize) {
    println!("Start game");
    let mut current_value = current_value;
    let max_value = get_max(cups);
    for _move_number in 1..=moves {
        if _move_number % 100_000 == 0 {
            print!("\r--- Move {:?} ---", _move_number.to_formatted_string(&Locale::en));
            io::stdout().flush().unwrap();
        }
        current_value = do_move(cups, current_value, max_value);
    }
    println!("");
}

fn do_move(cups: &mut Cups, current_value: usize, max_value: usize) -> usize {
    // print_cups(cups, current_value);
    // println!("Cups: {:?}", cups);
    // println!("Current index: {:?}", current_index);
    let picked_values = pick_up(cups, current_value);
    // println!("Picked values: {:?}", picked_values);
    let dest_value = destination(&picked_values, current_value, max_value);
    // println!("Destination: {}", dest_value);

    move_cups(cups, &picked_values, dest_value);
    // print_cups(cups, current_index);
    return cups[&current_value].next;
}

fn pick_up(cups: &Cups, current_value: usize) -> Vec<usize> {
    let mut current_value = current_value;
    let mut picked_cups: Vec<usize> = vec![];
    for _ in 0..3 {
        current_value = cups[&current_value].next;
        picked_cups.push(current_value);
    }
    return picked_cups;
}

fn destination(
    picked_values: &Vec<usize>,
    current_value: usize,
    max_value: usize,
) -> usize {
    let mut seek = current_value - 1;
    if seek == 0 {
        seek = max_value;
    }
    while picked_values.contains(&seek) {
        seek -= 1;
        if seek == 0 {
            seek = max_value;
        }
    }

    return seek;
}

fn move_cups(cups: &mut Cups, picked_values: &Vec<usize>, dest: usize) {
    let first_picked = picked_values[0];
    let last_picked = picked_values[2];
    let before_picked_values = cups[&first_picked].prev;
    let after_picked_values = cups[&last_picked].next;
    let before_insertion = dest;
    let after_insertion = cups[&dest].next;
    cups.get_mut(&before_insertion).unwrap().next = first_picked;
    cups.get_mut(&first_picked).unwrap().prev = before_insertion;
    cups.get_mut(&last_picked).unwrap().next = after_insertion;
    cups.get_mut(&after_insertion).unwrap().prev = last_picked;
    cups.get_mut(&before_picked_values).unwrap().next = after_picked_values;
    cups.get_mut(&after_picked_values).unwrap().prev = before_picked_values;
}

fn get_max(cups: &Cups) -> usize {
    return *cups.keys().max().unwrap();
}

fn cups_from_1(cups: &Cups) -> String {
    let mut result: String = String::from("");
    let mut cursor_value = 1;
    for _ in 0..cups.len() - 1 {
        cursor_value = cups[&cursor_value].next;
        result = result.to_owned() + cursor_value.to_string().as_str();
    }
    return result;
}

// fn print_cups(cups: &Cups, current_value: usize) {
//     let mut cursor_value = 1;
//     for _ in 0..cups.len() {
//         if cursor_value == current_value {
//             print!("({}) ", cursor_value);
//         } else {
//             print!("{} ", cursor_value);
//         }
//         cursor_value = cups[&cursor_value].next;
//     }
//     print!("\n");
// }

fn load_initial_cups() -> (Cups, Vec<usize>) {
    let mut cups: Cups = HashMap::new();
    let values: Vec<usize> = read_file("./inputs/23")
        .chars()
        .map(|c| c.to_string().parse().unwrap())
        .collect();
    for (index, value) in values.iter().enumerate() {
        cups.insert(
            *value,
            Cup {
                next: if index < values.len() - 1 {
                    values[index + 1]
                } else {
                    values[0]
                },
                prev: if index > 0 {
                    values[index - 1]
                } else {
                    values[values.len() - 1]
                },
            },
        );
    }

    return (cups, values);
}
