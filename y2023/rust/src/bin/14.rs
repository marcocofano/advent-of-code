use std::fmt;

use itertools::Itertools;

advent_of_code::solution!(14);

type Grid = Vec<char>;
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
    
    fn get_row_slice(&self, row: usize) -> &[char] {
        let start = row * self.size;
        &self.grid[start..start + self.size]
    }

    fn rotate_clockwise(&mut self) {
        for layer in 0..self.size / 2 {
            let first = layer;
            let last = self.size - 1 - layer;

            for i in first..last {
                let offset = i - first;

                let top = self.get(first, i);
                self.set(first, i, self.get(last - offset, first));
                self.set(last - offset, first, self.get(last, last - offset));
                self.set(last, last - offset, self.get(i, last));
                self.set(i, last, top);
            }
        }
    }

    fn get(&self, row: usize, col: usize) -> char {
        self.grid[row * self.size + col]
    }

    fn set(&mut self, row: usize, col: usize, value: char) {
        self.grid[row * self.size + col] = value;
    }

    fn count_weight(&self) -> usize {
        let mut weight = 0;
        for row_num in 0..self.size {
            let row = self.get_row_slice(row_num);
            weight += row
                .iter()
                .enumerate()
                .fold(0, |sum, (i, e)| if *e == 'O' { sum + i + 1 } else { sum });
        }
        weight
    }

    fn tilt_rocks(&mut self) {
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
    let mut platform = Platform {
        grid,
        size,
    };
    platform.rotate_clockwise();
    platform.tilt_rocks();
    Some(platform.count_weight())
}

pub fn part_two(_input: &str) -> Option<u32> {
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
        assert_eq!(result, None);
    }
}
