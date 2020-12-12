use crate::utils::print_day_banner;
use crate::utils::read_lines;
use std::collections::HashMap;
use std::collections::HashSet;

pub fn day06_01() {
    print_day_banner(6, 1);

    let groups_answers = load_groups_answers();
    let groups_answers_count = groups_answers.iter().map(group_answers_count_anyone);
    let total: u16 = groups_answers_count.sum();

    println!("Sum of counts: {}", total);
}

pub fn day06_02() {
    print_day_banner(6, 2);

    let groups_answers = load_groups_answers();
    let groups_answers_count = groups_answers.iter().map(group_answers_count_everyone);
    let total: u16 = groups_answers_count.sum();

    println!("Sum of counts: {}", total);
}

pub fn group_answers_count_anyone(group: &Vec<String>) -> u16 {
    let mut unique_answers = HashSet::new();
    for answer in group {
        for letter in answer.chars() {
            unique_answers.insert(letter);
        }
    }
    return unique_answers.len() as u16;
}

pub fn group_answers_count_everyone(group: &Vec<String>) -> u16 {
    let mut answers_count = HashMap::new();
    for answer in group {
        for letter in answer.chars() {
            let answer_count = answers_count.entry(letter).or_insert(0);
            *answer_count += 1;
        }
    }
    let people_count = group.len();
    return answers_count
        .values()
        .filter(|val| **val == people_count as u16)
        .collect::<Vec<&u16>>()
        .len() as u16;
}

fn load_groups_answers() -> Vec<Vec<String>> {
    return read_lines("./inputs/06")
        .unwrap()
        .filter_map(Result::ok)
        .collect::<Vec<String>>()
        .split(|line| line == "")
        .map(|group| group.to_vec())
        .collect();
}
