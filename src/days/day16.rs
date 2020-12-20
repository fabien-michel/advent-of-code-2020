use crate::utils::print_day_banner;
use crate::utils::read_lines;
// use itertools::iproduct;
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::ops::RangeInclusive;

#[derive(Debug)]
struct Rule {
    field_name: String,
    ranges: [RangeInclusive<usize>; 2],
}

type Ticket = Vec<usize>;

// enum ParseMode{
//     Rules,
//     MyTicket,
//     NearbyTickets,
// }

pub fn day16_01() {
    print_day_banner(16, 1);

    let (rules, _, nearby_tickets) = load_data();

    let invalid_values: Vec<usize> = nearby_tickets
        .iter()
        .map(|ticket| ticket_invalid_values(&ticket, &rules))
        .flatten()
        .map(|(_, value)| value)
        .collect();

    println!("{} / {}", invalid_values.len(), nearby_tickets.len());
    println!(
        "Sum of invalid values: {:?}",
        invalid_values.iter().sum::<usize>()
    );
}

pub fn day16_02() {
    print_day_banner(16, 2);

    let (rules, my_ticket, nearby_tickets) = load_data();

    let mut fields_matching_names: HashMap<usize, HashSet<&String>> = HashMap::new();

    let valid_nearby_tickets = nearby_tickets
        .iter()
        .filter(|ticket| !is_ticket_invalid(ticket, &rules));

    for ticket in valid_nearby_tickets {
        let ticket_fields_matching_rules = get_ticket_fields_matching_rules(&ticket, &rules);
        let ticket_fields_matching_names: Vec<(usize, Vec<&String>)> = ticket_fields_matching_rules
            .iter()
            .map(|(i, rules)| (*i, rules.iter().map(|rule| &rule.field_name).collect()))
            .collect();
        for (field_index, matching_field_name) in ticket_fields_matching_names {
            let matching_field_names_set: HashSet<&String> =
                matching_field_name.iter().cloned().collect();

            let field_field_name = fields_matching_names
                .entry(field_index)
                .or_insert(matching_field_names_set.clone());
            *field_field_name = field_field_name
                .intersection(&matching_field_names_set)
                .cloned()
                .collect();
        }
        // println!(
        //     "{:?}",
        //     fields_matching_names
        //         .iter()
        //         .map(|(k, v)| (k, v.len()))
        //         .collect::<Vec<(&usize, usize)>>()
        // );
    }

    let mut fields_mapping: HashMap<usize, String> = HashMap::new();
    loop {
        for (field_index, field_names) in &fields_matching_names {
            if field_names.len() == 1 && !fields_mapping.contains_key(field_index) {
                let field_name: &String = field_names.iter().cloned().collect::<Vec<&String>>()[0];
                fields_mapping.insert(*field_index, field_name.clone());
            }
        }
        for (_field_index, field_name) in &mut fields_mapping {
            let other_fields_keys: Vec<usize> = fields_matching_names.keys().cloned().collect();
            for other_key in other_fields_keys {
                let field_field_name = fields_matching_names.entry(other_key).or_default();
                field_field_name.remove(field_name);
            }
        }
        if fields_mapping.len() == rules.len() {
            break;
        }
    }

    let departure_fields: Vec<(&usize, &String)> = fields_mapping
        .iter()
        .filter(|(_, name)| name.starts_with("departure"))
        .collect();
    let departure_field_indexes: Vec<usize> = departure_fields.iter().map(|(i, _)| **i).collect();

    let my_departure_field_values: Vec<usize> = departure_field_indexes
        .iter()
        .map(|index| my_ticket[*index])
        .collect();
    let result: usize = my_departure_field_values.iter().product();

    println!("Product of my ticket's departure fields: {:?}", result);
}

fn ticket_invalid_values(ticket: &Ticket, rules: &Vec<Rule>) -> Vec<(usize, usize)> {
    let mut invalid_values = vec![];
    for (field_index, value) in ticket.iter().enumerate() {
        let rules_valid = rules.iter().any(|rule| is_value_match_rule(value, &rule));
        if !rules_valid {
            invalid_values.push((field_index, *value));
            break;
        }
    }
    // println!("{:?} {:?}", ticket, invalid_values);
    return invalid_values;
}

fn is_ticket_invalid(ticket: &Ticket, rules: &Vec<Rule>) -> bool {
    return ticket_invalid_values(ticket, rules).len() > 0;
}

fn get_ticket_fields_matching_rules<'a>(
    ticket: &Ticket,
    rules: &'a Vec<Rule>,
) -> Vec<(usize, Vec<&'a Rule>)> {
    let mut ticket_possible_fields: Vec<(usize, Vec<&Rule>)> = vec![];
    for (field_index, value) in ticket.iter().enumerate() {
        let value_possible_fields: Vec<&Rule> = rules
            .iter()
            .filter(|rule| is_value_match_rule(&value, rule))
            .collect();
        ticket_possible_fields.push((field_index, value_possible_fields))
    }

    return ticket_possible_fields;
}

fn is_value_match_rule(value: &usize, rule: &Rule) -> bool {
    return rule.ranges.iter().any(|range| range.contains(&value));
}

fn load_data() -> (Vec<Rule>, Ticket, Vec<Ticket>) {
    let lines = read_lines("./inputs/16").unwrap().filter_map(Result::ok);
    let mut rules = vec![];
    let mut my_ticket = vec![];
    let mut nearby_tickets = vec![];
    let mut mode = 0;
    for line in lines {
        match line.as_str() {
            "" => {
                mode += 1;
                continue;
            }
            "your ticket:" | "nearby tickets:" => {
                continue;
            }
            _ => {}
        }

        match mode {
            0 => rules.push(parse_rule(line)),
            1 => my_ticket = parse_ticket(line),
            2 => nearby_tickets.push(parse_ticket(line)),
            _ => {}
        }
    }

    return (rules, my_ticket, nearby_tickets);
}

fn parse_rule(line: String) -> Rule {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^(.+): (\d+)-(\d+) or (\d+)-(\d+)$").unwrap();
    }
    let caps = RE.captures(line.as_str()).unwrap();
    return Rule {
        field_name: String::from(&caps[1]),
        ranges: [
            caps[2].parse().unwrap()..=caps[3].parse().unwrap(),
            caps[4].parse().unwrap()..=caps[5].parse().unwrap(),
        ],
    };
}

fn parse_ticket(line: String) -> Ticket {
    return line
        .split(',')
        .map(|value| value.parse().unwrap())
        .collect();
}
