use crate::utils::print_day_banner;
use crate::utils::read_lines;
use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;

#[derive(Debug, Clone)]
enum Rule {
    Char(char),
    Subs(Vec<Vec<usize>>),
}

type Rules = HashMap<usize, Rule>;
type RulesRegexs = HashMap<usize, String>;

pub fn day19_01() {
    print_day_banner(19, 1);
    let (rules, messages) = load_data();
    let rule_regexs = build_rules_regex(&rules);

    let regex_0 = Regex::new(format!("^{}$", rule_regexs.get(&0).unwrap()).as_str()).unwrap();

    // println!("Regex 0: {:?}", regex_0);

    let count = messages
        .iter()
        .filter(|message| regex_0.is_match(message.as_str()))
        .count();

    println!("Matching messages: {:?}", count);
}

pub fn day19_02() {
    print_day_banner(19, 2);
    let (mut rules, messages) = load_data();
    
    // override rule 8 and 11
    let rule8 = rules.entry(8).or_insert(Rule::Char('?'));
    *rule8 = Rule::Subs(vec![vec![42], vec![42, 8]]);
    let rule11 = rules.entry(11).or_insert(Rule::Char('?'));
    *rule11 = Rule::Subs(vec![vec![42, 31], vec![42, 11, 31]]);

    let rule_regexs = build_rules_regex(&rules);

    let regex_0 = Regex::new(format!("^{}$", rule_regexs.get(&0).unwrap()).as_str()).unwrap();

    let count = messages
        .iter()
        .filter(|message| regex_0.is_match(message.as_str()))
        .count();

    println!("Matching messages: {:?}", count);
}

fn build_rules_regex(rules: &Rules) -> RulesRegexs {
    let mut rule_regexs: RulesRegexs = HashMap::new();

    for (rule_index, _rule) in rules {
        get_rule_regex(&mut rule_regexs, rule_index, &rules, 0);
    }
    return rule_regexs;
}

fn get_rule_regex(
    rule_regexs: &mut RulesRegexs,
    rule_index: &usize,
    rules: &Rules,
    lvl: usize,
) -> String {
    // Do not recurse more than 8 level (loop of day 2)
    if lvl >= 10 {
        return "".to_string();
    }
    // Do not recompute already known regex
    let has_regex = rule_regexs.get(rule_index);
    if has_regex.is_some() {
        return has_regex.unwrap().clone();
    }
    let regex = match &rules[rule_index] {
        Rule::Char(c) => c.to_string(),
        Rule::Subs(subs) => subs_regex(rule_regexs, rules, &subs, lvl),
    };
    
    rule_regexs.insert(*rule_index, regex.clone());
    return regex;
}

fn subs_regex(
    rule_regexs: &mut RulesRegexs,
    rules: &Rules,
    subs: &Vec<Vec<usize>>,
    lvl: usize,
) -> String {
    let mut subs_str = subs
        .iter()
        .map(|sub_idxs| {
            sub_idxs
                .iter()
                .map(|i| get_rule_regex(rule_regexs, i, rules, lvl + 1))
                .join("")
        })
        .join("|");
    if subs_str.contains("|") {
        subs_str = format!("({})", subs_str);
    }

    return subs_str.to_string();
}

fn load_data() -> (Rules, Vec<String>) {
    let lines: Vec<String> = read_lines("./inputs/19")
        .unwrap()
        .filter_map(Result::ok)
        .collect();
    // let rules:Vec<Rule> = Vec::with_capacity(capacity: usize);
    let mut rules: Rules = HashMap::new();
    let mut messages: Vec<String> = vec![];

    let mut mode = 0;
    for line in lines {
        match (line.as_str(), mode) {
            ("", _) => mode += 1,
            (_, 0) => {
                let (rule_index, rule) = parse_rule(line);
                rules.insert(rule_index, rule);
            }
            (_, 1) => messages.push(line),
            _ => panic!(),
        }
    }

    return (rules, messages);
}

fn parse_rule(line: String) -> (usize, Rule) {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^(?P<index>\d+): (?P<rule_str>.+)$").unwrap();
    }
    let caps = RE.captures(line.as_str()).unwrap();
    let rule_index: usize = caps.name("index").unwrap().as_str().parse().unwrap();
    let rule_str = caps.name("rule_str").unwrap().as_str();
    let rule: Rule;
    if rule_str.contains('"') {
        rule = Rule::Char(rule_str.chars().nth(1).unwrap());
    } else {
        let sub_rules: Vec<Vec<usize>> = rule_str
            .split(" | ")
            .map(|sub_str| sub_str.split(' ').map(|s| s.parse().unwrap()).collect())
            .collect();
        rule = Rule::Subs(sub_rules);
    }

    return (rule_index, rule);
}
