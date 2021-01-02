use crate::utils::print_day_banner;
use crate::utils::read_lines;
use std::cmp;
use std::collections::HashMap;

lazy_static! {
    static ref INSTRUCTION_DELTA: HashMap<Instruction, (isize, isize)> = {
        let mut m = HashMap::new();
        m.insert(Instruction::East, (1, 0));
        m.insert(Instruction::SouthEast, (0, 1));
        m.insert(Instruction::SouthWest, (-1, 1));
        m.insert(Instruction::West, (-1, 0));
        m.insert(Instruction::NorthWest, (0, -1));
        m.insert(Instruction::NorthEast, (1, -1));
        m
    };
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum Instruction {
    East,
    SouthEast,
    SouthWest,
    West,
    NorthWest,
    NorthEast,
}

type TileInstructions = Vec<Instruction>;

type Tiles = HashMap<(isize, isize), bool>;

pub fn day24_01() {
    print_day_banner(24, 1);
    let tiles_instructions = load_tiles_instructions();

    let tiles = build_initial_tiles(&tiles_instructions);

    let count = tiles.values().filter(|v| **v).count();
    println!("Black tiles: {:?}", count);
}

pub fn day24_02() {
    print_day_banner(24, 2);
    let tiles_instructions = load_tiles_instructions();
    let mut tiles = build_initial_tiles(&tiles_instructions);
    for _d in 1..=100 {
        tiles = next_day(&tiles);
    }
    let count = tiles.values().filter(|v| **v).count();
    println!("Black tiles: {:?}", count);
}

fn next_day(tiles: &Tiles) -> Tiles {
    let mut next_tiles: Tiles = HashMap::new();
    let ((min_q, max_q), (min_r, max_r)) = get_bounds(tiles);

    for q in (min_q - 1)..=(max_q + 1) {
        for r in (min_r - 1)..=(max_r + 1) {
            let black_neighbors = count_black_neighbors(tiles, q, r);
            let mut tile = *tiles.get(&(q, r)).unwrap_or(&false);
            if tile && (black_neighbors == 0 || black_neighbors > 2) {
                tile = false;
            } else if !tile && black_neighbors == 2 {
                tile = true;
            }
            next_tiles.insert((q, r), tile);
        }
    }
    return next_tiles;
}

fn count_black_neighbors(tiles: &Tiles, q: isize, r: isize) -> usize {
    let mut black_count = 0;
    for (_dir, (dq, dr)) in INSTRUCTION_DELTA.iter() {
        let tile = *tiles.get(&(q + dq, r + dr)).unwrap_or(&false);
        if tile {
            black_count += 1;
        }
    }
    return black_count;
}

fn get_bounds(tiles: &Tiles) -> ((isize, isize), (isize, isize)) {
    let mut min_q = 0;
    let mut min_r = 0;
    let mut max_q = 0;
    let mut max_r = 0;
    for (q, r) in tiles
        .iter()
        .filter_map(|(k, v)| if *v { Some(k) } else { None })
    {
        min_q = cmp::min(min_q, *q);
        min_r = cmp::min(min_r, *r);
        max_q = cmp::max(max_q, *q);
        max_r = cmp::max(max_r, *r);
    }
    return ((min_q, max_q), (min_r, max_r));
}

fn build_initial_tiles(tiles_instructions: &Vec<TileInstructions>) -> Tiles {
    let mut tiles: Tiles = HashMap::new();

    for tile_instructions in tiles_instructions.iter() {
        let coordinates = instructions_coordinates(tile_instructions);
        tiles.insert(coordinates, !tiles.contains_key(&coordinates));
    }
    return tiles;
}

fn instructions_coordinates(instructions: &Vec<Instruction>) -> (isize, isize) {
    let mut q = 0;
    let mut r = 0;

    for instruction in instructions.iter() {
        let (dq, dr) = instruction_delta(instruction.clone());
        q += dq;
        r += dr;
    }

    return (q, r);
}

fn instruction_delta(instruction: Instruction) -> (isize, isize) {
    return *INSTRUCTION_DELTA.get(&instruction).unwrap();
}

// fn print_tiles(tiles: &Tiles) {
//     let ((min_q, max_q), (min_r, max_r)) = get_bounds(tiles);
//     let mut matrix: Vec<Vec<char>> =
//         vec![vec!['░'; max_q as usize * 2 + 2]; max_r as usize * 2 + 2];
    
//     for (q, r) in tiles
//         .iter()
//         .filter_map(|(k, v)| if *v { Some(k) } else { None })
//     {
//         let mq = q + min_q.abs();
//         let mr = r + min_r.abs();
//         matrix[mr as usize][mq as usize] = '▓';
//     }

//     for row in matrix.iter() {
//         for c in row.iter() {
//             print!("{}", c);
//         }
//         print!("\n");
//     }
// }

fn load_tiles_instructions() -> Vec<TileInstructions> {
    return read_lines("./inputs/24")
        .unwrap()
        .filter_map(Result::ok)
        .map(parse_line)
        .collect();
}

fn parse_line(line: String) -> TileInstructions {
    let mut instructions: TileInstructions = vec![];
    let mut south_north: Option<char> = None;
    for c in line.chars() {
        match (south_north, c) {
            (None, 's') => south_north = Some('s'),
            (None, 'n') => south_north = Some('n'),
            (None, 'e') => {
                instructions.push(Instruction::East);
                south_north = None;
            }
            (None, 'w') => {
                instructions.push(Instruction::West);
                south_north = None;
            }
            (Some('n'), 'e') => {
                instructions.push(Instruction::NorthEast);
                south_north = None;
            }
            (Some('s'), 'e') => {
                instructions.push(Instruction::SouthEast);
                south_north = None;
            }
            (Some('n'), 'w') => {
                instructions.push(Instruction::NorthWest);
                south_north = None;
            }
            (Some('s'), 'w') => {
                instructions.push(Instruction::SouthWest);
                south_north = None;
            }
            _ => panic!(""),
        }
    }
    return instructions;
}
