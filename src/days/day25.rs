use crate::utils::print_day_banner;
use crate::utils::read_lines;

const DIVIDER: usize = 20201227;

pub fn day25_01() {
    print_day_banner(25, 1);
    let (card_pk, door_pk) = load_public_keys();
    println!("Primary keys: {:?}", (card_pk, door_pk));
    let (card_loop_size, door_loop_size) = guess_loop_sizes((card_pk, door_pk));
    println!("Loop sizes: {:?}", (card_loop_size, door_loop_size));
    println!("{:?}", transform_number(7, card_loop_size));
    let encryption_key = transform_number(card_pk, door_loop_size);
    println!("Encryption key: {:?}", encryption_key);
}

pub fn day25_02() {
    print_day_banner(25, 2);
}

fn guess_loop_sizes(pks: (usize, usize)) -> (usize, usize) {
    let mut result = 1;
    let mut loop_sizes: (usize, usize) = (0, 0);
    for n in 1.. {
        result = (result * 7) % DIVIDER;
        if result == pks.0 {
            loop_sizes.0 = n;
        } else if result == pks.1 {
            loop_sizes.1 = n;
        }
        if loop_sizes.0 != 0 && loop_sizes.1 != 0 {
            break;
        }
    }
    return loop_sizes;
}

fn transform_number(number: usize, loop_size: usize) -> usize {
    let mut result = 1;
    for _ in 0..loop_size {
        result = result * number;
        result = result % DIVIDER;
    }
    return result;
}

fn load_public_keys() -> (usize, usize) {
    let pks: Vec<_> = read_lines("./inputs/25")
        .unwrap()
        .filter_map(Result::ok)
        .map(|line| line.parse().unwrap())
        .collect();
    return (pks[0], pks[1]);
}
