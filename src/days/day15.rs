use crate::utils::print_day_banner;
use crate::utils::read_file;
pub fn day15_01() {
    print_day_banner(15, 1);

    println!("2020th spoken number: {:}", nth_spoken_number(2020));
}

pub fn day15_02() {
    print_day_banner(15, 2);

    println!(
        "30,000,000th spoken number: {:}",
        nth_spoken_number(30_000_000)
    );
}

fn nth_spoken_number(nth: usize) -> usize {
    let mut spoken_numbers = load_initial_numbers();

    let mut last_turn_spoken: Vec<isize> = vec![-1; nth];

    for (turn, number) in spoken_numbers.iter().enumerate() {
        if turn == spoken_numbers.len() - 1 {
            break;
        }
        last_turn_spoken[*number] = turn as isize;
    }

    for turn in (spoken_numbers.len() - 1)..(nth - 1) {
        let next_turn = turn + 1;

        let last_spoken_number = spoken_numbers[turn];
        let next_number = match last_turn_spoken[last_spoken_number] {
            -1 => 0,
            previous_turn => next_turn - previous_turn as usize - 1,
        };
        last_turn_spoken[last_spoken_number] = turn as isize;
        spoken_numbers.push(next_number);
    }
    return spoken_numbers[nth - 1];
}

fn load_initial_numbers() -> Vec<usize> {
    return read_file("./inputs/15")
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect();
}
