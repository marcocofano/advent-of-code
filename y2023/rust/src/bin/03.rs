use std::{cmp::min, collections::HashMap};
use regex::Regex;
use advent_of_code::parsers::lines;

advent_of_code::solution!(3);

pub fn check_symbols(start_pos: usize, end_pos: usize, row: &str, symbols:&Vec<char>) -> bool {
    let sub = &row[start_pos..=end_pos];
    if sub.chars().any(|c| symbols.contains(&c)) == true {
        true
    } else {
        false // Handle cases where start or end are out of bounds
    }
}
pub fn part_one(input: &str) -> Option<u32> {
    let lns: Vec<&str> = lines(input).collect();
    let regex = Regex::new(r"\d+").unwrap();
    let tot_cols = lns[0].len();

    let mut symbols: Vec<char> = (0..=127 as u8) // ASCII range
            .filter_map(|c| if c.is_ascii_punctuation() { Some(c as char) } else { None })
            .collect();
    let char_to_remove = '.';
    symbols.retain(|&c| c != char_to_remove);
    let tot_rows = lns.len() as i32;
    let mut result: u32 = 0;
    for (r, &row) in lns.iter().enumerate() {
        let r = r as i32;
        regex.find_iter(row).for_each(|number|  {
            let col_start = number.start().saturating_sub(1);
            let col_end = min(number.end(), tot_cols - 1);
            for offset in [-1i32, 0, 1].iter() {
                let check_row = r + offset;
                if check_row < tot_rows && check_row > 0 {
                    let current_line = lns.get(check_row as usize).unwrap();
                    if check_symbols(col_start, col_end, &current_line, &symbols) == true {
                        let add = number.as_str().parse::<u32>().unwrap_or_else(|_| 0);
                        result += add;
                        break
                    }
                }
            }
        })
    }
    Some(result)
}

pub fn check_star(start_pos: usize, end_pos: usize, row: &str) -> Vec<usize> {
    let mut stars_indexes = vec![];
    for (i, item) in row[start_pos..=end_pos].chars().enumerate() {
         if item == '*' {
            stars_indexes.push(start_pos + i);
         };
    }
    return stars_indexes
}

pub fn part_two(input: &str) -> Option<u32> {
    let lns: Vec<&str> = lines(input).collect();
    let mut gears: HashMap<String, Vec<u32>> = HashMap::new();
    let regex = Regex::new(r"\d+").unwrap();
    let tot_cols = lns[0].len();

    let mut symbols: Vec<char> = (0..=127 as u8) // ASCII range
            .filter_map(|c| if c.is_ascii_punctuation() { Some(c as char) } else { None })
            .collect();
    let char_to_remove = '.';
    symbols.retain(|&c| c != char_to_remove);
    let tot_rows = lns.len() as i32;
    for (r, &row) in lns.iter().enumerate() {
        let r = r as i32;
        regex.find_iter(row).for_each(|number|  {
            let col_start = number.start().saturating_sub(1);
            let col_end = min(number.end(), tot_cols - 1);
            for offset in [-1i32, 0, 1].iter() {
                let check_row = r + offset;
                if check_row < tot_rows && check_row > 0 {
                    let current_line = lns.get(check_row as usize).unwrap();
                    let star_cols = check_star(col_start, col_end, &current_line);
                    if !star_cols.is_empty() {
                        for star_col in star_cols.iter() {
                            let add = number.as_str().parse::<u32>().unwrap_or_else(|_| 0);
                            let key =format!("{}-{}",check_row, &star_col);
                            gears.entry(key.to_string()).or_insert(Vec::new()) 
                                .push(add);
                        }
                    }
                }
            }
        })
    }
    let mut result = 0;
    for value in gears.values() {
        if value.len() == 2 {
            let prod = value.iter().fold(1,|a, &b| a * b); 
            result += prod;
        }
        
    }
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
