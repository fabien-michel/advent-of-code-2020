use crate::utils::print_day_banner;
use crate::utils::read_lines;

type TreesRow = [bool; 31];
type TreesPattern = Vec<TreesRow>;

pub fn day03_01() {
    print_day_banner(3, 1);
    let trees_pattern = load_trees_pattern();
    let trees_encountered = trees_encountered_hypothesis(&trees_pattern, 3, 1);

    println!("Trees encountered: {:?}", trees_encountered)
}

pub fn day03_02() {
    print_day_banner(3, 2);
    let trees_pattern = load_trees_pattern();

    let hypotheses: [(u16, u16); 5] = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    let trees_encountered: Vec<u16> = hypotheses
        .iter()
        .map(|(dx, dy)| trees_encountered_hypothesis(&trees_pattern, *dx, *dy))
        .collect();

    println!("Trees encountered: {:?}", trees_encountered);
    let prod: u32 = trees_encountered.iter().copied().map(|te| te as u32).product();

    println!("Product: {:?}", prod)
}

fn trees_encountered_hypothesis(trees_pattern: &TreesPattern, delta_x: u16, delta_y: u16) -> u16 {
    let mut x: u16 = 0;
    let mut y: u16 = 0;

    let mut trees_encountered: u16 = 0;
    loop {
        match is_tree_at(&trees_pattern, x, y) {
            Ok(bam) => trees_encountered += if bam { 1 } else { 0 },
            Err(_) => break,
        }
        x = x + delta_x;
        y = y + delta_y;
    }
    return trees_encountered;
}

fn is_tree_at(trees_pattern: &TreesPattern, x: u16, y: u16) -> Result<bool, &str> {
    let modulated_x = x % 31;
    if y > trees_pattern.len() as u16 - 1 {
        return Err("Fini !");
    }
    return Ok(trees_pattern[y as usize][modulated_x as usize]);
}

fn load_trees_pattern() -> TreesPattern {
    return read_lines("./inputs/03")
        .unwrap()
        .filter_map(Result::ok)
        .map(parse_line)
        .collect();
}

fn parse_line(line: String) -> TreesRow {
    let mut result: TreesRow = [false; 31];
    for (index, c) in line.chars().enumerate() {
        if c == '#' {
            result[index] = true;
        }
    }
    return result;
}
