use crate::utils::print_day_banner;
use crate::utils::read_lines;
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;

type BagContain = Vec<(String, u16)>;

#[derive(Debug)]
struct BagInfo {
    color: String,
    contains: BagContain,
    parents: Vec<String>,
}

pub fn day07_01() {
    print_day_banner(7, 1);

    let rules = load_bags_rules();

    let shiny_gold = rules.get("shiny gold").unwrap();

    let mut parents = shiny_gold.parents.to_vec();
    let mut found_parents = HashSet::new();
    while parents.len() > 0 {
        let parent_color = parents.pop().unwrap();
        let parent = rules.get(&parent_color).unwrap();
        if parent.parents.len() > 0 {
            parents.extend_from_slice(&parent.parents);
        }
        found_parents.insert(parent_color.clone());
    }

    println!("Parents count: {:?}", found_parents.len());
}

pub fn day07_02() {
    print_day_banner(7, 2);

    let rules = load_bags_rules();

    let shiny_gold = rules.get("shiny gold").unwrap();

    let count = count_all_childs(&shiny_gold, &rules);

    println!("Childs count: {:?}", count);
}

fn count_all_childs(bag_info: &BagInfo, rules: &HashMap<String, BagInfo>) -> u16 {
    let mut count: u16 = 0;
    for (child_color, quantity) in bag_info.contains.iter() {
        let child_bag_info = rules.get(child_color).unwrap();
        count += quantity;
        count += count_all_childs(child_bag_info, rules) * quantity;
    }
    return count;
}

fn load_bags_rules() -> HashMap<String, BagInfo> {
    let mut rules: HashMap<String, BagInfo> = HashMap::new();

    let lines: Vec<String> = read_lines("./inputs/07")
        .unwrap()
        .filter_map(Result::ok)
        .collect();
    for line in lines {
        let bag_info = extract_bag_info(&line);
        rules.insert(bag_info.color.clone(), bag_info);
    }

    populate_parents(&mut rules);

    return rules;
}

fn extract_bag_info(line: &String) -> BagInfo {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(?P<color>.*) bags contain (?P<contains>.*).").unwrap();
        static ref RE_CONTAINS: Regex =
            Regex::new(r"^(?P<quantity>\d+) (?P<color>.*) bags?$").unwrap();
    }
    let caps = RE.captures(line.as_str()).unwrap();
    let color = caps.name("color").unwrap().as_str();
    let contains_str = caps.name("contains").unwrap().as_str();

    let mut bag_info: BagInfo = BagInfo {
        color: color.to_string(),
        contains: vec![],
        parents: vec![],
    };

    if contains_str == "no other bags" {
        return bag_info;
    }

    for contain_str in contains_str.split(", ") {
        let contain_caps = RE_CONTAINS.captures(contain_str).unwrap();
        let contain_color = contain_caps.name("color").unwrap().as_str();
        let contain_quantity = contain_caps
            .name("quantity")
            .unwrap()
            .as_str()
            .parse::<u16>()
            .unwrap();
        bag_info
            .contains
            .push((contain_color.to_string(), contain_quantity));
    }

    return bag_info;
}

fn populate_parents(rules: &mut HashMap<String, BagInfo>) {
    // Determine for each bag, its direct possible parents (container)
    // Because I didn't success to mutate a HashMap during calking throught it,
    // I create an intermediate HashMap containing parents for each bag color and then populate the rules HashMap
    let mut parents_to_insert: HashMap<String, Vec<String>> = HashMap::new();
    for (color, bag_info) in rules.iter() {
        for (contain_color, _) in bag_info.contains.iter() {
            let bag_parents = parents_to_insert.entry(contain_color.clone()).or_default();
            bag_parents.push(color.clone());
        }
    }

    for (color, bag_parents) in parents_to_insert {
        rules
            .get_mut(&color)
            .unwrap()
            .parents
            .extend_from_slice(&bag_parents);
    }
}
