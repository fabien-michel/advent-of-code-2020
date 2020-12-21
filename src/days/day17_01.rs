use crate::utils::print_day_banner;
use crate::utils::read_lines;

const PDIM_SIZE: usize = 20;

#[derive(Debug, Clone)]
struct Pos {
    x: usize,
    y: usize,
    z: usize,
}

#[derive(Debug, Clone)]
struct Cube {
    state: bool,
    neighbors: Vec<Pos>,
}

type Cubes = Vec<Vec<Vec<Cube>>>;

#[derive(Debug)]
struct PDim {
    cubes: Cubes,
}

impl PDim {
    fn cube_at(&self, pos: &Pos) -> &Cube {
        return &self.cubes[pos.z][pos.y][pos.x];
    }
    fn state_at(&self, pos: &Pos) -> bool {
        return self.cube_at(pos).state;
    }
}

pub fn day17_01() {
    print_day_banner(17, 1);

    let mut pdim = load_pocket_dimension();
    // print_pdim(&pdim);

    let mut iteration = 0;

    loop {
        iteration += 1;
        // println!("Iteration {}", iteration);

        let mut new_cubes = init_cubes();
        let mut changed = false;

        walk_pdim(&pdim, |cube, pos| {
            let (active_neighbors, _) = count_neighbors(&pdim, &pos);

            let new_state = match (cube.state, active_neighbors) {
                (true, 2) | (true, 3) => true,
                (false, 3) => true,
                _ => false,
            };

            if cube.state != new_state {
                changed = true;
            }
            new_cubes[pos.z][pos.y][pos.x] = Cube {
                state: new_state,
                neighbors: cube.neighbors.clone(),
            }
        });

        pdim.cubes = new_cubes;

        // print_pdim(&pdim);
        if !changed || iteration == 6 {
            break;
        }
    }

    println!("Active cubes: {:?}", count_active_cubes(&pdim));
}

fn count_active_cubes(pdim: &PDim) -> usize {
    let mut count = 0;
    walk_pdim(&pdim, |c, _pos| {
        if c.state {
            count += 1;
        }
    });
    return count;
}

fn walk_pdim<F>(pdim: &PDim, mut f: F)
where
    F: FnMut(&Cube, Pos),
{
    for z in 0..PDIM_SIZE {
        for y in 0..PDIM_SIZE {
            for x in 0..PDIM_SIZE {
                let pos = to_pos(x, y, z);
                f(&pdim.cube_at(&pos), pos);
            }
        }
    }
}

fn count_neighbors(pdim: &PDim, pos: &Pos) -> (usize, usize) {
    let counts = pdim
        .cube_at(pos)
        .neighbors
        .iter()
        .fold((0, 0), |acc, n_pos| match pdim.state_at(n_pos) {
            true => (acc.0 + 1, acc.1),
            false => (acc.0, acc.1 + 1),
        });
    return counts;
}

// fn print_pdim(pdim: &PDim) {
//     for z in 0..PDIM_SIZE {
//         println!("z={}", z);
//         for y in 0..PDIM_SIZE {
//             for x in 0..PDIM_SIZE {
//                 print!("{}", get_char(&pdim.cubes[z][y][x]));
//             }
//             print!("\n");
//         }
//     }
// }

// fn get_char(cube: &Cube) -> char {
//     match cube.state {
//         true => '#',
//         false => '.',
//     }
// }

fn to_pos(x: usize, y: usize, z: usize) -> Pos {
    return Pos { x: x, y: y, z: z };
}

fn init_cubes() -> Cubes {
    let cubes = vec![
        vec![
            vec![
                Cube {
                    state: false,
                    neighbors: vec![]
                };
                PDIM_SIZE
            ];
            PDIM_SIZE
        ];
        PDIM_SIZE
    ];

    return cubes;
}

fn load_pocket_dimension() -> PDim {
    let lines: Vec<String> = read_lines("./inputs/17")
        .unwrap()
        .filter_map(Result::ok)
        .collect();
    let mut cubes = init_cubes();
    init_neighbors(&mut cubes);

    let offset = PDIM_SIZE / 2 - lines.len() / 2;
    let z = PDIM_SIZE / 2;

    for (y, line) in lines.iter().enumerate() {
        let y = y + offset;
        for (x, c) in line.chars().enumerate() {
            let x = x + offset;
            cubes[z][y][x].state = c == '#';
        }
    }

    return PDim { cubes: cubes };
}

fn init_neighbors(cubes: &mut Cubes) {
    for z in 0..PDIM_SIZE {
        for y in 0..PDIM_SIZE {
            for x in 0..PDIM_SIZE {
                cubes[z][y][x].neighbors = get_neighbors(x as isize, y as isize, z as isize);
            }
        }
    }
}

fn get_neighbors(x: isize, y: isize, z: isize) -> Vec<Pos> {
    let mut neighbors = vec![];
    for nx in (x - 1)..=(x + 1) {
        for ny in (y - 1)..=(y + 1) {
            for nz in (z - 1)..=(z + 1) {
                if !(nx == x && ny == y && nz == z)
                    && (0..PDIM_SIZE).contains(&(nx as usize))
                    && (0..PDIM_SIZE).contains(&(ny as usize))
                    && (0..PDIM_SIZE).contains(&(nz as usize))
                {
                    neighbors.push(to_pos(nx as usize, ny as usize, nz as usize));
                }
            }
        }
    }
    return neighbors;
}
