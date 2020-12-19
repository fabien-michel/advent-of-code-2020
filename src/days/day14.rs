use crate::utils::print_day_banner;
use crate::utils::read_lines;
use regex::Regex;
use std::collections::HashMap;

type BitsValue = [char; 36];

#[derive(Debug, Copy, Clone)]
enum Instruction {
    Mask(BitsValue),
    Mem {
        address: usize,
        baddress: BitsValue,
        value: usize,
        bvalue: BitsValue,
    },
}

pub fn day14_01() {
    print_day_banner(14, 1);

    let mut memory = HashMap::new();

    let instructions = load_instructions();
    let mut mask = string_to_bit_value(String::from(""));
    for instruction in instructions.iter() {
        match instruction {
            Instruction::Mask(bvalue) => {
                mask = *bvalue;
            }
            Instruction::Mem {
                bvalue,
                value: _,
                address,
                baddress: _,
            } => {
                // println!("-----");
                // println!("value:  {:?}", bvalue);
                // println!("mask :  {:?}", mask);
                memory.insert(address, apply_mask(*bvalue, mask, 'X'));
                // println!("result: {:?}", memory.get(address).unwrap());
            }
        }
    }

    let total = memory
        .values()
        .fold(0, |acc, bvalue| acc + bit_value_to_int(*bvalue));

    println!("{:?}", total);
}

pub fn day14_02() {
    print_day_banner(14, 2);

    let mut memory = HashMap::new();

    let instructions = load_instructions();
    let mut mask = string_to_bit_value(String::from(""));
    for instruction in instructions.iter() {
        match instruction {
            Instruction::Mask(bvalue) => {
                mask = *bvalue;
            }
            Instruction::Mem {
                bvalue: _,
                value,
                address: _,
                baddress,
            } => {
                // println!("-----");
                // println!("value:  {:?}", value);
                // println!("mask :  {:?}", mask);
                // println!("baddress :  {:?}", baddress);
                for address in all_possible_addresses(*baddress, mask) {
                    // println!("  -> {:?}\t{:?}", address, int_to_bit_value(address));
                    memory.insert(address, value);
                }
            }
        }
    }

    let total = memory.values().fold(0, |acc, value| acc + *value);

    println!("{:?}", total);
}

fn apply_mask(bvalue: BitsValue, mask: BitsValue, neutral_char: char) -> BitsValue {
    return mask
        .iter()
        .enumerate()
        .filter(|(_i, &c)| c != neutral_char)
        .fold(bvalue, |mut bv, (index, c)| {
            bv[index] = *c;
            bv
        });
}

fn all_possible_addresses(baddress: BitsValue, mask: BitsValue) -> Vec<usize> {
    // apply mask on BitValue address
    let baddress_with_mask = apply_mask(baddress, mask, '0');
    // println!("baddress_with_mask :  {:?}", baddress_with_mask);

    // get X chars indexes in baddress_with_mask
    let floating_bit_indexes: Vec<usize> = baddress_with_mask
        .iter()
        .enumerate()
        .filter(|(_i, &c)| c == 'X')
        .map(|(i, _c)| i)
        .collect();
        
    let x_count = floating_bit_indexes.len() as u32;
    
    let mut addresses: Vec<usize> = vec![];

    // counter from 1 to 2^(numbers of x)
    for cpt in 0..2usize.pow(x_count) {
        // get binary reperensation of the counter (split by chars)
        let mut value_str: Vec<char> = format!("{:b}", cpt).chars().collect();
        // insert missing leading 0 (ugly)
        for _ in 0..(floating_bit_indexes.len() - value_str.len()) {
            value_str.insert(0, '0');
        }
        // copy baddress_with_mask and replace X chars for all possible values
        let mut new_badresse: BitsValue = baddress_with_mask.clone();
        for (v_index, a_index) in floating_bit_indexes.iter().enumerate() {
            new_badresse[*a_index] = value_str[v_index];
        }
        addresses.push(bit_value_to_int(new_badresse))
    }
    return addresses;
}

fn string_to_bit_value(string: String) -> BitsValue {
    let mut bvalue: BitsValue = ['X'; 36];
    for (index, c) in string.chars().enumerate() {
        if index >= 36 {
            break;
        }
        bvalue[index] = c;
    }
    return bvalue;
}

fn int_to_bit_value(value: usize) -> BitsValue {
    let mut bvalue: BitsValue = ['0'; 36];
    let value_str = format!("{:b}", value);
    let offset = 36 - value_str.len();
    for (index, c) in value_str.chars().enumerate() {
        bvalue[index + offset] = c;
    }
    return bvalue;
}

fn bit_value_to_int(bvalue: BitsValue) -> usize {
    let mut value = 0;
    for (index, c) in bvalue.iter().enumerate() {
        if *c == '1' {
            value += 2usize.pow(35u32 - index as u32);
        }
    }
    return value;
}

fn load_instructions() -> Vec<Instruction> {
    return read_lines("./inputs/14")
        .unwrap()
        .filter_map(Result::ok)
        .map(parse_line)
        .collect();
}

fn parse_line(line: String) -> Instruction {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"^(?P<kind>mask|mem)(\[(?P<address>\d+)\])? = (?P<value>.*)$").unwrap();
    }
    let caps = RE.captures(line.as_str()).unwrap();
    let value = caps.name("value").unwrap().as_str().to_string();
    return match caps.name("kind").unwrap().as_str() {
        "mask" => Instruction::Mask(string_to_bit_value(value)),
        "mem" => {
            let value: usize = value.parse().unwrap();
            let address: usize = caps.name("address").unwrap().as_str().parse().unwrap();
            Instruction::Mem {
                address: address,
                baddress: int_to_bit_value(address),
                value: value,
                bvalue: int_to_bit_value(value),
            }
        }
        _ => panic!(),
    };
}
