use itertools::Itertools;

advent_of_code::solution!(13);

// Utilities
trait IteratorAllButOneExt: Iterator {
    fn all_but_one<F>(&mut self, mut condition: F) -> bool
    where
        Self: Sized,
        F: FnMut(&Self::Item) -> bool,
    {
        let mut count_fails = 0;

        for element in self {
            if !condition(&element) {
                count_fails += 1;
            }
            if count_fails > 2 {
                return false;
            }
        }
        if count_fails == 1 {
            return true;
        } else {
            return false;
        }
    }
}

impl<I: Iterator> IteratorAllButOneExt for I {}

fn transpose(input: &Vec<&str>) -> Vec<String> {
    let mut transposed: Vec<String> = Vec::new();

    for i in 0..input[0].len() {
        let column: String = input.iter().map(|s| s.chars().nth(i).unwrap()).collect();
        transposed.push(column);
    }

    transposed
}

fn is_palindrome(s: &str) -> bool {
    if s.len() == 0 {
        return false;
    }
    s.chars().collect::<String>() == s.chars().rev().collect::<String>()
}

fn parse_blocks(input: &str) -> Vec<Vec<&str>> {
    let mut blocks = vec![];
    let mut block = vec![];
    for line in input.lines() {
        if line.is_empty() {
            blocks.push(block);
            block = vec![];
        } else {
            block.push(line);
        }
    }
    if !block.is_empty() {
        // push the last block
        blocks.push(block);
    }
    blocks
}

fn find_reflection(block: &[&str]) -> Option<usize> {
    let length = block[0].len();
    for i in (1..length).step_by(2) {
        if block.iter().all(|l| is_palindrome(&l[0..length-i])) {
            return Some((length - i) / 2);
        }
        if block.iter().all(|l| is_palindrome(&l[i..])) {
            return Some(i + (length - i) / 2);
        }
    }
    None
}


fn find_reflection_but_one(block: &[&str]) -> Option<usize> {
    let length = block[0].len();
    for i in (1..length).step_by(2) {
        if block
            .iter()
            .all_but_one(|l| is_palindrome(&l[0..length - i]))
        {
            return Some((length - i) / 2);
        }
        if block.iter().all_but_one(|l| is_palindrome(&l[i..])) {
            return Some(i + (length - i) / 2);
        }
    }
    None
}


fn find_final_reflection(block: &Vec<&str>, find_function: fn(&[&str]) -> Option<usize>) -> usize {
    if let Some(res) = find_function(block) {
        return res;
    }
    let blk = transpose(block);
    if let Some(res) = find_function(&blk.iter().map(|s| s.as_str()).collect_vec()) {
        return res * 100;
    }
    panic!("No reflection found")
}

pub fn part_one(input: &str) -> Option<u32> {
    let blocks = parse_blocks(input);
    Some(
        blocks
            .iter()
            .map(|v| find_final_reflection(v, find_reflection) as u32)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let blocks = parse_blocks(input);
    Some(
        blocks
            .iter()
            .map(|v| find_final_reflection(v, find_reflection_but_one) as u32)
            .sum(),
    )
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(405));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(400));
    }
}
