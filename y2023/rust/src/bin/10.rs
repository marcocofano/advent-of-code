use advent_of_code::parsers::lines;
use itertools::Itertools;

advent_of_code::solution!(10);

type Point = (usize, usize);

#[derive(Debug, PartialEq, PartialOrd, Copy, Clone)]
enum Dir {
    N,
    W,
    S,
    E,
    O, //Origin
    G, // Ground
}

impl Dir {
    fn as_coord(&self) -> (isize, isize) {
        match self {
            Dir::N => (-1, 0),
            Dir::E => (0, 1),
            Dir::S => (1, 0),
            Dir::W => (0, -1),
            _ => (0, 0),
        }
    }
}

// type Tile = (Direction, Direction);

fn parse_tile(c: char) -> (Dir, Dir) {
    return match c {
        '7' => (Dir::W, Dir::S),
        'J' => (Dir::N, Dir::W),
        'L' => (Dir::N, Dir::E),
        '-' => (Dir::W, Dir::E),
        '|' => (Dir::N, Dir::S),
        'F' => (Dir::S, Dir::E),
        'S' => (Dir::O, Dir::O),
        _ => (Dir::G, Dir::G),
    };
}
fn find_start(grid: &Vec<Vec<(Dir, Dir)>>) -> Option<Point> {
    // Finds the Starting tile and looks for the first available valid pipe tile
    for (row_index, row) in grid.iter().enumerate() {
        if let Some(col_index) = row.iter().position(|&val| val == (Dir::O, Dir::O)) {
            return Some((row_index, col_index));
        }
    }
    return None;
}

fn find_first_tile(grid: &Vec<Vec<(Dir, Dir)>>, start_pos: Point) -> Option<(Point, Dir)> {
    // Finds the Starting tile and looks for the first available valid pipe tile
    let checks = [
        [(Dir::S, Dir::E), (Dir::N, Dir::E), (Dir::W, Dir::S)],
        [(Dir::W, Dir::S), (Dir::S, Dir::E), (Dir::N, Dir::W)],
        [(Dir::W, Dir::E), (Dir::S, Dir::E), (Dir::N, Dir::E)],
        [(Dir::N, Dir::W), (Dir::N, Dir::E), (Dir::N, Dir::S)],
    ];
    let dir = [Dir::E, Dir::N, Dir::W, Dir::S];
    for (c, d) in checks.iter().zip(dir) {
        let row: usize = start_pos.0
            .saturating_add_signed(d.as_coord().0)
            .try_into()
            .unwrap();
        let col: usize = start_pos.1
            .saturating_add_signed(d.as_coord().1)
            .try_into()
            .unwrap();
        let current_symbol = grid[row][col];
        if c.iter().find(|&&s| s == current_symbol).is_some() {
            return Some(((row, col), d));
        }
    }
    None
}
fn find_out_direction(in_dir: Dir, tile: (Dir, Dir)) -> Dir {
    // in a pipe tile, given the entry direction finds the exit direction
    // TODO: write it as a method of a tile struct?
    if in_dir == tile.0 {
        return tile.1;
    } else {
        return tile.0;
    }
}

fn move_next_tile(
    grid: &Vec<Vec<(Dir, Dir)>>,
    in_direction: Dir,
    current_pos: Point,
) -> (Dir, Point) {
    let tile = grid[current_pos.0][current_pos.1];
    let out_dir = find_out_direction(in_direction, tile);
    let next_row = current_pos
        .0
        .saturating_add_signed(out_dir.as_coord().0)
        .try_into()
        .unwrap();
    let next_col = current_pos
        .1
        .saturating_add_signed(out_dir.as_coord().1)
        .try_into()
        .unwrap();
    return (out_dir, (next_row, next_col));
}

fn change_out_in_dir(out_dir: Dir) -> Dir {
    // The exit direction gets mapped to the opposite direction to serve as entry for the next tile
    return match out_dir {
        Dir::N => Dir::S,
        Dir::S => Dir::N,
        Dir::W => Dir::E,
        Dir::E => Dir::W,
        _ => Dir::G,
    };
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = lines(input)
        .map(|l| l.chars().map(|t| parse_tile(t)).collect_vec())
        .collect_vec();
    let (start_row, start_col) = find_start(&grid).expect("No starting tile found");
    let (first_tile_pos, first_tile_dir) =
        find_first_tile(&grid, (start_row, start_col)).expect("No first valid pipe tile found");

    let mut in_direction = change_out_in_dir(first_tile_dir);
    let mut current_pos = first_tile_pos;
    let mut current_tile = grid[first_tile_pos.0][first_tile_pos.1];
    let mut step = 1;
    while current_tile != (Dir::O, Dir::O) {
        let (next_direction, next_pos) = move_next_tile(&grid, in_direction, current_pos);
        in_direction = change_out_in_dir(next_direction);
        current_tile = grid[next_pos.0][next_pos.1]; // can avoid this line if the check is on the
                                                     // start position instead of tile
        current_pos = next_pos;
        step += 1;
    }
    Some(step / 2)
}

fn poligon_area_step(current_pos: Point, next_pos: Point) -> i32 {
    // compute the area inside the poligon with unit sides, applying Green's theorem. For the final
    // area sum these steps and divide by 2.
    return (next_pos.0 as i32 + current_pos.0 as i32) * (next_pos.1 as i32 - current_pos.1 as i32);
}
pub fn part_two(input: &str) -> Option<u32> {
    let grid = lines(input)
        .map(|l| l.chars().map(|t| parse_tile(t)).collect_vec())
        .collect_vec();
    let start_pos = find_start(&grid).expect("No starting tile found");
    let (first_tile_pos, first_tile_dir) =
        find_first_tile(&grid, start_pos).expect("This needs to be a valid start");

    let mut in_direction = change_out_in_dir(first_tile_dir);
    let mut current_pos = first_tile_pos;
    let mut current_tile = grid[first_tile_pos.0][first_tile_pos.1];
    let mut area: i32 = poligon_area_step(start_pos, first_tile_pos);
    let mut step: i32 = 1;
    while current_tile != (Dir::O, Dir::O) {
        let (next_direction, next_pos) = move_next_tile(&grid, in_direction, current_pos);
        in_direction = change_out_in_dir(next_direction);
        current_tile = grid[next_pos.0][next_pos.1];

        area += poligon_area_step(current_pos, next_pos);
        current_pos = next_pos;
        step += 1;
    }
    area = area / 2;
    Some((area - (step / 2) + 1i32).try_into().unwrap()) // pick's formula
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(23));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
