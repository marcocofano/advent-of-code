use std::{collections::BTreeMap, usize};

use itertools::Itertools;

advent_of_code::solution!(15);
fn parse_steps(input: &str) -> Vec<&str> {
    let lines = input.lines().collect_vec();
    let mut steps = vec![];
    for line in lines {
        steps.extend(parse_collect_line(line));
    }
    steps
}

fn parse_collect_line(line: &str) -> Vec<&str> {
    line.split(',').collect_vec()
}

pub fn part_one(input: &str) -> Option<usize> {
    let steps = parse_steps(input);
    let result = steps.iter().map(|item| hash_function(&item)).sum();
    // dbg!(result);
    Some(result)
}

fn hash_function(step: &str) -> usize {
    step.chars()
        .fold(0, |acc, c| ((acc + (c as usize)) * 17) % 256)
}

pub fn part_two(input: &str) -> Option<usize> {
    let steps = parse_steps(input);
    let mut boxes: BTreeMap<usize, Vec<(&str, usize)>> = BTreeMap::new();
    for step in steps {
        if let Some((label, focal)) = step.split_once('=') {
            let focal_parsed = focal.parse().unwrap();
            if let Some(b) = boxes.get_mut(&hash_function(&label)) {
                if let Some((_, focal_to_update)) = b.iter_mut().find(|(l, _)| *l == label) {
                    // Update the value in the existing tuple
                    *focal_to_update = focal_parsed;
                } else {
                    // If not found, append a new tuple to the vector
                    b.push((&label, focal_parsed));
                }
            } else {
                boxes.insert(hash_function(label), vec![(label, focal_parsed)]);
            }
        }
        if let Some((label, _)) = step.split_once('-') {
            if let Some(b) = boxes.get_mut(&hash_function(&label)) {
                b.retain(| (l, _) | *l != label)
            }
        }

    }
    // dbg!(&boxes);
    Some(
        boxes
            .iter()
            .map(|(key, b)| {
                (key + 1) * b.iter().enumerate().fold(0, |acc, (i, (_, f))| acc + (i + 1) * f)
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash() {
        assert_eq!(hash_function("rn"), 0);
        assert_eq!(hash_function("qp"), 1);
        assert_eq!(hash_function("pc"), 3);
    }
    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1320));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file(
            "examples", DAY,
        ));
        assert_eq!(result, Some(145));
    }
}
