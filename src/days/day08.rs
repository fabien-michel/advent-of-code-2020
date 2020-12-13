use crate::utils::print_day_banner;
use crate::utils::read_lines;
use regex::Regex;
use std::collections::HashSet;

#[derive(Debug, Copy, Clone)]
enum Operation {
    Nop,
    Acc,
    Jmp,
}

#[derive(Debug, Copy, Clone)]
struct Instruction {
    id: u16,
    operation: Operation,
    argument: i16,
}

pub fn day08_01() {
    print_day_banner(8, 1);

    let instructions = load_instructions();

    let (accumulator, _) = compute_instructions(&instructions, None);

    println!("Accumulator: {:?}", accumulator);
}

pub fn day08_02() {
    print_day_banner(8, 2);

    let instructions = load_instructions();

    // Get all JMp and Nop instructions
    let jmp_nop_instructions: Vec<&Instruction> = instructions
        .iter()
        .filter(|instruction| matches!(instruction.operation, Operation::Nop | Operation::Jmp))
        .collect();

    let mut jmp_nop_switch_occurence = 0;
    let mut tries = 0;
    let mut last_accumulator;
    
    loop {
        tries += 1;
        // Try with the next Jmp/Nop instruction by changing it operation
        let jmp_nop_instruction = &jmp_nop_instructions[jmp_nop_switch_occurence];
        let change_instruction = Instruction {
            id: jmp_nop_instruction.id,
            operation: if matches!(jmp_nop_instruction.operation, Operation::Jmp) {
                Operation::Nop
            } else {
                Operation::Jmp
            },
            argument: jmp_nop_instruction.argument,
        };
        let (accumulator, end_reached) = compute_instructions(
            &instructions,
            Some((jmp_nop_instruction.id, Some(&change_instruction))),
        );
        last_accumulator = accumulator;
        if end_reached {
            break;
        }
        jmp_nop_switch_occurence += 1;
    }

    println!("Accumulator: {:?} ({} tries)", last_accumulator, tries);
}

fn compute_instructions(
    instructions: &Vec<Instruction>,
    change_operation: Option<(u16, Option<&Instruction>)>,
) -> (i16, bool) {
    let mut accumulator: i16 = 0;
    let mut visited_instructions: HashSet<u16> = HashSet::new();
    let mut cursor: i16 = 0;

    let (change_id, change_instruction) = change_operation.unwrap_or((0, None));

    loop {
        let instruction;

        if change_instruction.is_some() && change_id == instructions[cursor as usize].id {
            instruction = change_instruction.unwrap();
        } else {
            instruction = &instructions[cursor as usize];
        }

        if visited_instructions.contains(&instruction.id) {
            return (accumulator, false);
        }
        visited_instructions.insert(instruction.id);

        match instruction.operation {
            Operation::Acc => accumulator += instruction.argument,
            Operation::Jmp => cursor += instruction.argument - 1,
            _ => {}
        }

        cursor += 1;

        if cursor as usize > instructions.len() - 1 {
            return (accumulator, true);
        }
    }
}

fn load_instructions() -> Vec<Instruction> {
    return read_lines("./inputs/08")
        .unwrap()
        .filter_map(Result::ok)
        .enumerate()
        .map(parse_line)
        .collect();
}

fn parse_line((index, line): (usize, String)) -> Instruction {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^(?P<operation>...) (?P<argument>[-+]\d+)$").unwrap();
    }
    // println!("{}", line);
    let caps = RE.captures(line.as_str()).unwrap();

    return Instruction {
        id: index as u16,
        operation: match caps.name("operation").unwrap().as_str() {
            "jmp" => Operation::Jmp,
            "acc" => Operation::Acc,
            _ => Operation::Nop,
        },
        argument: caps
            .name("argument")
            .unwrap()
            .as_str()
            .parse::<i16>()
            .unwrap(),
    };
}
