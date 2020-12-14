use crate::utils::print_day_banner;
use crate::utils::read_lines;

use std::collections::HashMap;

pub fn day10_01() {
    print_day_banner(10, 1);
    let mut joltages = load_joltages();
    let goal_output = joltages.iter().max().unwrap() + 3;
    joltages.push(goal_output);
    let mut current_output = 0;
    let mut diff_1_count = 0;
    let mut diff_3_count = 0;
    joltages.sort();

    for joltage in joltages.iter() {
        match joltage - current_output {
            1 => diff_1_count += 1,
            3 => diff_3_count += 1,
            _ => {}
        }
        current_output = *joltage;
    }

    let prod = diff_1_count * diff_3_count;

    println!(
        "1-diff: {:?} * 3-diff: {}  = {}",
        diff_1_count, diff_3_count, prod
    );
}

pub fn day10_02() {
    print_day_banner(10, 2);
    let mut joltages = load_joltages();
    let goal_output = joltages.iter().max().unwrap() + 3;
    joltages.push(0);
    joltages.push(goal_output);
    joltages.sort();
    let mut memo: HashMap<u16, u64> = HashMap::new();
    let edges = build_graph(&joltages);
    // println!("{:?}", edges);
    let count = count_to_goal(&edges, &0, goal_output, &mut memo);
    println!("All possible paths: {:?}", count);
}

fn count_to_goal(
    edges: &HashMap<u16, Vec<u16>>,
    from: &u16,
    goal: u16,
    memo: &mut HashMap<u16, u64>,
) -> u64 {
    if *from == goal {
        return 1;
    }
    if memo.contains_key(from) {
        return *memo.get(from).unwrap();
    }
    // println!("{:?}", from);
    let mut count: u64 = 0;
    let next_edges = edges.get(from).unwrap();
    for edge in next_edges {
        count += count_to_goal(&edges, edge, goal, memo);
    }
    memo.insert(*from, count);
    return count;
}

fn build_graph(joltages: &Vec<u16>) -> HashMap<u16, Vec<u16>> {
    let mut edges: HashMap<u16, Vec<u16>> = HashMap::new();
    let mut hot_joltages: Vec<u16> = vec![];
    for joltage in joltages {
        let mut to_clean: Vec<u16> = vec![];
        for hj in hot_joltages.iter() {
            let diff = joltage - hj;
            let edge = edges.entry(*hj).or_insert(vec![]);
            if diff <= 3 {
                edge.push(*joltage);
            } else {
                to_clean.push(*hj);
            }
        }
        hot_joltages = hot_joltages
            .iter()
            .filter(|hj| !to_clean.contains(hj))
            .map(|hj| *hj)
            .collect();
        hot_joltages.push(*joltage);
    }

    return edges;
}

fn load_joltages() -> Vec<u16> {
    return read_lines("./inputs/10")
        .unwrap()
        .filter_map(Result::ok)
        .map(|line| line.parse::<u16>().unwrap())
        .collect();
}
