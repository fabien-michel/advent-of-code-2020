use crate::days::day20_01::{build_tile_matrix, load_tiles, TileMatrix};
use crate::utils::print_day_banner;

const TILE_SIZE: usize = 10;
const IMAGE_SIZE: usize = 96;

type Image = Vec<Vec<char>>;

pub fn day20_02() {
    print_day_banner(20, 2);

    let tiles = load_tiles();
    let tile_matrix = build_tile_matrix(&tiles);
    let mut image = merge_tiles(&tile_matrix);
    let mut monsters = 0;
    let mut rotate_cpt = 1;
    let mut flip_h_cpt = 0;
    let mut flip_v_cpt = 0;
    loop {
        for x in 0..IMAGE_SIZE {
            for y in 0..IMAGE_SIZE {
                if find_monster(&mut image, x, y) {
                    monsters += 1;
                }
            }
        }
        if monsters > 0 {
            break;
        }
        image = if rotate_cpt != flip_h_cpt {
            flip_h_cpt += 1;
            flip_h(&image)
        } else if rotate_cpt != flip_v_cpt {
            flip_v_cpt += 1;
            flip_v(&image)
        } else {
            rotate_cpt += 1;
            rotate_image_90(&image)
        }
    }

    println!("Monsters found: {:?}", monsters);
    let remaining_hash: usize = image
        .iter()
        .map(|row| row.iter().filter(|&c| c == &'#').count())
        .sum();
    println!("Remaining #: {:?}", remaining_hash);
}

const MONSTER: [(usize, usize); 15] = [
    (0, 18),
    (1, 0),
    (1, 5),
    (1, 6),
    (1, 11),
    (1, 12),
    (1, 17),
    (1, 18),
    (1, 19),
    (2, 1),
    (2, 4),
    (2, 7),
    (2, 10),
    (2, 13),
    (2, 16),
];

fn find_monster(image: &mut Image, x: usize, y: usize) -> bool {
    let mut checks_ok = 0;
    for (dy, dx) in MONSTER.iter() {
        let test_x = x + dx;
        let test_y = y + dy;
        if test_x >= IMAGE_SIZE || test_y >= IMAGE_SIZE {
            return false;
        }
        if image[test_y][test_x] == '#' {
            checks_ok += 1;
        }
    }
    if checks_ok != MONSTER.len() {
        return false;
    }
    for (dy, dx) in MONSTER.iter() {
        image[dy + y][dx + x] = 'O';
    }

    return true;
}

fn merge_tiles(tile_matrix: &TileMatrix) -> Image {
    let mut image: Image = vec![vec!['.'; IMAGE_SIZE]; IMAGE_SIZE];

    for (tile_row, row) in tile_matrix.iter().enumerate() {
        for (tile_col, vtile) in row.iter().enumerate() {
            for (pix_row, crow) in vtile.data().iter().enumerate() {
                if pix_row == 0 || pix_row == TILE_SIZE - 1 {
                    continue;
                }
                for (pix_col, c) in crow.iter().enumerate() {
                    if pix_col == 0 || pix_col == TILE_SIZE - 1 {
                        continue;
                    }
                    let ip_row = tile_row * (TILE_SIZE - 2) + pix_row - 1;
                    let ip_col = tile_col * (TILE_SIZE - 2) + pix_col - 1;
                    image[ip_row][ip_col] = match c {
                        '1' => '#',
                        _ => '.',
                    };
                }
            }
        }
    }

    return image;
}

// fn print_image(image: &Image) {
//     for row in image {
//         for c in row {
//             let c = match c {
//                 '#' => '▓',
//                 '.' => '░',
//                 _ => ' ',
//             };
//             print!("{}", c);
//         }
//         print!("\n");
//     }
// }

fn rotate_image_90(image: &Image) -> Image {
    let mut new_image: Image = vec![vec!['.'; IMAGE_SIZE]; IMAGE_SIZE];

    for (row, row_data) in image.iter().enumerate() {
        for (col, c) in row_data.iter().enumerate() {
            // data[x][TILE_SIZE - 1 - y] = *c;
            new_image[col][IMAGE_SIZE - 1 - row] = *c;
        }
    }

    return new_image;
}

fn flip_h(image: &Image) -> Image {
    let mut new_image: Image = vec![vec!['.'; IMAGE_SIZE]; IMAGE_SIZE];
    for (y, row) in image.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            new_image[y][IMAGE_SIZE - 1 - x] = *c;
        }
    }
    return new_image;
}
fn flip_v(image: &Image) -> Image {
    let mut new_image: Image = vec![vec!['.'; IMAGE_SIZE]; IMAGE_SIZE];
    for (y, row) in image.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            new_image[IMAGE_SIZE - 1 - y][x] = *c;
        }
    }
    return new_image;
}
