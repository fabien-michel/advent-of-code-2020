use crate::utils::print_day_banner;
use crate::utils::read_lines;
use regex::Regex;

const TILE_SIZE: usize = 10;
const IMAGE_SIZE: usize = 12;

#[derive(Debug, Clone)]
struct Edges {
    top: u16,
    left: u16,
    right: u16,
    bottom: u16,
}

type TileData = Vec<Vec<char>>;

#[derive(Debug, Clone)]
struct TileVersion {
    edges: Edges,
    data: TileData,
}

type TileVersions = Vec<TileVersion>;

#[derive(Debug, Clone)]
pub struct Tile {
    id: usize,
    data: TileData,
    versions: TileVersions,
}

type Tiles = Vec<Tile>;

#[derive(Debug, Clone)]
pub struct VersionedTile<'a> {
    tile: &'a Tile,
    version_number: usize,
}

impl<'a> VersionedTile<'a> {
    fn edges(&self) -> &'a Edges {
        return &self.tile.versions[self.version_number].edges;
    }
    pub fn data(&self) -> &'a TileData {
        return &self.tile.versions[self.version_number].data;
    }
}

pub type TileMatrix<'a> = Vec<Vec<VersionedTile<'a>>>;

pub fn day20_01() {
    print_day_banner(20, 1);

    let tiles = load_tiles();
    let tile_matrix = build_tile_matrix(&tiles);
    // print_tile_matrix(&tile_matrix);

    let corners_prod: usize = vec![
        tile_matrix[0][0].tile.id,
        tile_matrix[0][IMAGE_SIZE - 1].tile.id,
        tile_matrix[IMAGE_SIZE - 1][0].tile.id,
        tile_matrix[IMAGE_SIZE - 1][IMAGE_SIZE - 1].tile.id,
    ]
    .iter()
    .product();

    println!("{:?}", corners_prod);
}

pub fn build_tile_matrix<'a>(all_tiles: &'a Tiles) -> TileMatrix {
    let mut tile_matrix: TileMatrix<'a> = vec![];
    let used_tile_ids: Vec<usize> = vec![];
    let res = find_tile(&mut tile_matrix, all_tiles, &used_tile_ids, 0, 0);
    if !res {
        panic!("Unable to find tile matrix");
    }
    return tile_matrix;
}

fn find_tile<'a>(
    tile_matrix: &mut TileMatrix<'a>,
    all_tiles: &'a Tiles,
    used_tile_ids: &Vec<usize>,
    x: usize,
    y: usize,
) -> bool {
    let surrounding_tiles: [Option<&VersionedTile>; 2] = match (x, y) {
        (0, 0) => [None, None],
        (x, 0) => [None, Some(&tile_matrix[y][x - 1])],
        (0, y) => [Some(&tile_matrix[y - 1][x]), None],
        (x, y) => [Some(&tile_matrix[y - 1][x]), Some(&tile_matrix[y][x - 1])],
    };
    let remaining_tiles: Vec<&Tile> = all_tiles
        .iter()
        .filter(|tile| !used_tile_ids.contains(&tile.id))
        .collect();
    let matching_tile_versions: Vec<VersionedTile> = remaining_tiles
        .iter()
        .map(|tile| matching_tile_versions(&tile, &surrounding_tiles))
        .flatten()
        .collect();

    let (next_x, next_y) = if (0..(IMAGE_SIZE - 1)).contains(&x) {
        (x + 1, y)
    } else {
        (0, y + 1)
    };

    for versioned_tile in matching_tile_versions {
        // print_progress(x,y);
        // println!("{:?}", (x,y,&versioned_tile.edges()));
        let mut new_used_tile_ids: Vec<usize> = used_tile_ids.iter().cloned().collect();
        new_used_tile_ids.push(versioned_tile.tile.id);
        if tile_matrix.len() <= y {
            tile_matrix.push(vec![]);
        }
        if tile_matrix[y].len() <= x {
            tile_matrix[y].push(versioned_tile);
        } else {
            tile_matrix[y][x] = versioned_tile;
        }
        if next_y == IMAGE_SIZE {
            return true;
        }

        let res = find_tile(tile_matrix, all_tiles, &new_used_tile_ids, next_x, next_y);
        if res {
            return true;
        }
    }
    return false;
}

fn matching_tile_versions<'a>(
    tile: &'a Tile,
    surrounding_tiles: &[Option<&VersionedTile>; 2],
) -> Vec<VersionedTile<'a>> {
    return tile
        .versions
        .iter()
        .enumerate()
        .filter(|(_, version)| match surrounding_tiles {
            [Some(top_tile), Some(left_tile)] => {
                version.edges.top == top_tile.edges().bottom
                    && version.edges.left == left_tile.edges().right
            }
            [None, Some(left_tile)] => version.edges.left == left_tile.edges().right,
            [Some(top_tile), None] => version.edges.top == top_tile.edges().bottom,
            [None, None] => true,
        })
        .map(|(index, _)| VersionedTile {
            tile: tile,
            version_number: index,
        })
        .collect();
}

// fn print_progress(x: usize, y: usize) {
//     let progress = y * IMAGE_SIZE + x;
//     let remaing = IMAGE_SIZE * IMAGE_SIZE - progress;
//     print!(
//         "{}\r",
//         [
//             String::from("▯").repeat(progress),
//             String::from(" ").repeat(remaing),
//             String::from("|")
//         ]
//         .join("")
//     );
// }

// fn print_tile_matrix_ids(tile_matrix: &TileMatrix) {
//     for tile_row in tile_matrix {
//         for vtile in tile_row {
//             print!("{:04} #{} ", vtile.tile.id, vtile.version_number);
//         }
//         print!("\n");
//     }
//     print!("\n");
// }

// pub fn print_tile_matrix(tile_matrix: &TileMatrix) {
//     let mut str_matrix =
//         vec![vec![' '; IMAGE_SIZE * (TILE_SIZE + 1)]; IMAGE_SIZE * (TILE_SIZE + 1)];
//     for (tile_row, row) in tile_matrix.iter().enumerate() {
//         for (tile_col, vtile) in row.iter().enumerate() {
//             for (pix_row, crow) in vtile.data().iter().enumerate() {
//                 for (pix_col, c) in crow.iter().enumerate() {
//                     let c_row = (tile_row * (TILE_SIZE + 1)) + pix_row;
//                     let c_col = (tile_col * (TILE_SIZE + 1)) + pix_col;
//                     str_matrix[c_row][c_col] = if c == &'0' { '░' } else { '█' };
//                 }
//             }
//             let tile_id_row = tile_row * (TILE_SIZE + 1) + 10;
//             let tile_id_col = tile_col * (TILE_SIZE + 1);
//             let tile_id: Vec<char> = format!("{} /{}", vtile.tile.id, vtile.version_number)
//                 .chars()
//                 .collect();
//             for (i, c) in tile_id.iter().enumerate() {
//                 str_matrix[tile_id_row][tile_id_col + i] = *c;
//             }
//         }
//     }
//     for row in str_matrix {
//         for c in row {
//             print!("{}", c);
//         }
//         print!("\n");
//     }
// }

pub fn load_tiles() -> Tiles {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^Tile (\d+):").unwrap();
    }

    let lines: Vec<String> = read_lines("./inputs/20")
        .unwrap()
        .filter_map(Result::ok)
        .collect();

    let mut tiles: Tiles = vec![];
    let mut tile_id: usize = 0;
    let mut tile_data_str: Vec<String> = vec![];
    // let mut tile_edges: Edges;
    for line in lines {
        if line == "" {
            tiles.push(create_tile(tile_id, &tile_data_str));
            tile_data_str = vec![];
            continue;
        }
        let caps = RE.captures(line.as_str());
        if caps.is_some() {
            tile_id = caps.unwrap().get(1).unwrap().as_str().parse().unwrap();
            continue;
        }
        tile_data_str.push(line.replace(".", "0").replace("#", "1"));
    }

    return tiles;
}

fn create_tile(id: usize, tile_data_str: &Vec<String>) -> Tile {
    let tile_data = get_tile_data(tile_data_str);
    return Tile {
        id: id,
        versions: get_versions(&tile_data),
        // versions: old_get_versions(tile_data_str),
        data: tile_data,
    };
}

fn get_versions(tile_data: &TileData) -> TileVersions {
    // (rotate 90° times, 1 = flip h | 2 = flip v)
    return [
        (0, 0),
        (0, 1),
        (0, 2),
        (1, 0),
        (1, 1),
        (1, 2),
        (2, 0),
        (3, 0),
    ]
    .iter()
    .map(|(rotate, flip)| {
        let mut data = match rotate {
            0 => tile_data.clone(),
            _ => rotate_tile_data_90(tile_data, *rotate),
        };
        data = match flip {
            1 => flip_h(&data),
            2 => flip_v(&data),
            _ => data,
        };
        TileVersion {
            edges: get_edges(&data),
            data: data,
        }
    })
    .collect();
}

fn get_tile_data(tile_data_str: &Vec<String>) -> TileData {
    let mut data = vec![vec!['0'; TILE_SIZE]; TILE_SIZE];
    for (y, row) in tile_data_str.iter().enumerate() {
        for (x, c) in row.chars().enumerate() {
            data[y][x] = c;
        }
    }
    return data;
}

fn get_edges(tile_data: &TileData) -> Edges {
    let top_row_str: String = tile_data[0].iter().clone().collect();
    let bottom_row_str: String = tile_data[TILE_SIZE - 1].iter().clone().collect();
    let left_col_str: String = tile_data.iter().map(|row| row[0]).collect();
    let right_col_str: String = tile_data.iter().map(|row| row[TILE_SIZE - 1]).collect();

    let top_row = u16::from_str_radix(top_row_str.as_str(), 2).unwrap();
    let bottom_row = u16::from_str_radix(bottom_row_str.as_str(), 2).unwrap();
    let left_col = u16::from_str_radix(left_col_str.as_str(), 2).unwrap();
    let right_col = u16::from_str_radix(right_col_str.as_str(), 2).unwrap();

    return Edges {
        top: top_row,
        right: right_col,
        bottom: bottom_row,
        left: left_col,
    };
}

fn rotate_tile_data_90(tile_data: &TileData, count: usize) -> TileData {
    let mut data: TileData = vec![vec!['0'; TILE_SIZE]; TILE_SIZE];
    for (y, row) in tile_data.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            data[x][TILE_SIZE - 1 - y] = *c;
        }
    }
    if count > 1 {
        data = rotate_tile_data_90(&data, count - 1);
    }
    return data;
}

fn flip_h(tile_data: &TileData) -> TileData {
    let mut data: TileData = vec![vec!['0'; TILE_SIZE]; TILE_SIZE];
    for (y, row) in tile_data.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            data[y][TILE_SIZE - 1 - x] = *c;
        }
    }
    return data;
}
fn flip_v(tile_data: &TileData) -> TileData {
    let mut data: TileData = vec![vec!['0'; TILE_SIZE]; TILE_SIZE];
    for (y, row) in tile_data.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            data[TILE_SIZE - 1 - y][x] = *c;
        }
    }
    return data;
}
