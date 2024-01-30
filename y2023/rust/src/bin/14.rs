use itertools::Itertools;
use std::collections::HashMap;
use std::fmt;
advent_of_code::solution!(14);

enum Direction {
    North,
    West,
    South,
    East,
}

const DIRECTIONS: [Direction; 4] = [Direction::North, Direction::West, Direction::South, Direction::East];
type Grid = Vec<char>;
#[derive(Clone)]
struct Platform {
    grid: Grid,
    size: usize,
}

impl fmt::Debug for Platform {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "[")?;

        for row in 0..self.size {
            write!(f, "  [")?;

            let row_data = self.get_row_slice(row);
            for &value in row_data.iter() {
                write!(f, "{} ", value)?;
            }
            writeln!(f, "]")?;
        }
        writeln!(f, "]")
    }
}

impl Platform {

    fn to_hashable(&self) -> Vec<u128> {
        // Azarias version
        let mut result = vec![];
        for row in 0..self.size {
            let row_slice = self.get_row_slice(row);
            result.push(row_slice.iter().enumerate().fold(0u128, |acc, (x, &c)| {
                if c == 'O' {
                    acc | (1 << x)
                } else {
                    acc
                }
            }));
        }
        result
    }

    // More expensive representation. it does not matter in terms of speed.
    #[allow(dead_code)]
    fn to_hashable_2(&self) -> Vec<usize> {
        return self
            .grid
            .iter()
            .positions(|&item| item == 'O')
            .collect_vec();
    }

    fn get_row_slice(&self, row: usize) -> &[char] {
        let start = row * self.size;
        &self.grid[start..start + self.size]
    }

    fn get(&self, row: usize, col: usize) -> char {
        self.grid[row * self.size + col]
    }

    fn set(&mut self, row: usize, col: usize, value: char) {
        self.grid[row * self.size + col] = value;
    }

    fn count_weight(&self) -> usize {
        let weight = (0..self.size).fold(0, |sum, r| {
            sum + (self.size - r)
                * self
                    .get_row_slice(r)
                    .iter()
                    .filter(|&item| item == &'O')
                    .count()
        });
        weight
    }

    fn tilt_rocks(&mut self, direction: &Direction) {
        match direction {
            Direction::East => {
                for row_n in 0..self.size {
                    let mut first_available = self.size - 1;
                    for col_n in (0..self.size).rev() {
                        match self.get(row_n, col_n) {
                            '#' => first_available = col_n.saturating_sub(1),
                            'O' => {
                                self.set(row_n, col_n, '.');
                                self.set(row_n, first_available, 'O');
                                first_available = first_available.saturating_sub(1);
                            }
                            _ => {}
                        }
                    }
                }
            }
            Direction::West => {
                for row_n in 0..self.size {
                    let mut first_available = 0;
                    for col_n in 0..self.size {
                        match self.get(row_n, col_n) {
                            '#' => first_available = col_n + 1,
                            'O' => {
                                self.set(row_n, col_n, '.');
                                self.set(row_n, first_available, 'O');
                                first_available = first_available + 1;
                            }
                            _ => {}
                        }
                    }
                }
            }
            Direction::South => {
                for col_n in 0..self.size {
                    let mut first_available = self.size - 1;
                    for row_n in (0..self.size).rev() {
                        match self.get(row_n, col_n) {
                            '#' => first_available = row_n.saturating_sub(1),
                            'O' => {
                                self.set(row_n, col_n, '.');
                                self.set(first_available, col_n, 'O');
                                first_available = first_available.saturating_sub(1);
                            }
                            _ => {}
                        }
                    }
                }

            }
            Direction::North => {
                for col_n in 0..self.size {
                    let mut first_available = 0;
                    for row_n in 0..self.size {
                        match self.get(row_n, col_n) {
                            '#' => first_available = row_n + 1,
                            'O' => {
                                self.set(row_n, col_n, '.');
                                self.set(first_available, col_n, 'O');
                                first_available = first_available + 1;
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
    }

    fn do_cycle(&mut self) {
        for dir in DIRECTIONS.iter() {
            self.tilt_rocks(dir);
        }
    }
}

fn parse_lines(input: &str) -> (Grid, usize) {
    let mut res = vec![];
    let mut size = 0;
    for line in input.lines() {
        res.extend(line.chars().collect_vec());
        size += 1;
    }
    (res, size)
}

pub fn part_one(input: &str) -> Option<usize> {
    let (grid, size) = parse_lines(input);
    let mut platform = Platform { grid, size };
    platform.tilt_rocks(&Direction::North);
    Some(platform.count_weight())
}

pub fn part_two(input: &str) -> Option<usize> {
    let (grid, size) = parse_lines(input);
    let mut platform = Platform { grid, size };
    let mut history_map = HashMap::new();
    let end = 1_000_000_000 - 1;
    for i in 0..end {
        platform.do_cycle();
        let platform_hash = platform.to_hashable();
        if let Some(&position) = history_map.get(&platform_hash) {
            let cycle_size = i - position;
            let rest = end - i;
            let final_steps = rest % cycle_size;
            for _ in 0..final_steps {
                platform.do_cycle()
            }
            let result = platform.count_weight();
            return Some(result);
        } else {
            history_map.insert(platform_hash, i);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(136));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(64));
    }
}
