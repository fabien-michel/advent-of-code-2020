use crate::utils::print_day_banner;
use crate::utils::read_lines;
use itertools::iproduct;

pub fn day09_01() {
    print_day_banner(9, 1);
    let numbers = load_numbers();
    let number = find_invalid_number(&numbers);
    println!("Stopped on {:?}", number);
}

pub fn day09_02() {
    print_day_banner(9, 2);
    let numbers = load_numbers();
    let number = find_invalid_number(&numbers);

    let mut cursor_min: usize = 0;
    let mut cursor_max: usize = 1;
    let mut range;
    loop {
        range = &numbers[cursor_min..cursor_max];
        let sum: u64 = range.iter().sum();
        if sum == number {
            break;
        } else if sum > number {
            cursor_min += 1;
            cursor_max = cursor_min + 1
        } else {
            cursor_max += 1;
        }
    }
    let min = range.iter().min().unwrap();
    let max = range.iter().max().unwrap();
    let sum = min + max;

    println!("{:?}", range);
    println!("Min: {} | Max: {} | Sum: {}", min, max, sum);
}

fn find_invalid_number(numbers: &Vec<u64>) -> u64 {
    let mut cursor: usize = 25;

    loop {
        let number = numbers[cursor];
        let previous = &numbers[cursor - 25..cursor];
        let sum_exists = iproduct!(previous.iter(), previous.iter())
            .any(|(prev1, prev2)| prev1 != prev2 && prev1 + prev2 == number);
        if !sum_exists {
            return number;
        }
        cursor += 1;
    }
}

fn load_numbers() -> Vec<u64> {
    return read_lines("./inputs/09")
        .unwrap()
        .filter_map(Result::ok)
        .map(|line| line.parse::<u64>().unwrap())
        .collect();
}
