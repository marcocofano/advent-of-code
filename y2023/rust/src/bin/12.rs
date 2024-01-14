
use std::{char, collections::HashMap};

use advent_of_code::parsers::lines;
use itertools::Itertools;

advent_of_code::solution!(12);

fn parse_spring(line: &str) -> (Vec<char>, Vec<usize>) {
    let (springs, damaged) = line.split_once(' ').unwrap();
    let springs = springs.chars().collect();
    let damaged = damaged
        .split(',')
        .map(|m| m.parse::<usize>().unwrap())
        .collect_vec();
    (springs, damaged)
}

fn unfold_springs(springs: Vec<char>, n: usize) -> Vec<char> {
    let mut result = Vec::new();

    for i in 0..n {
        if i > 0 {
            result.push('?');
        }
        result.extend(springs.iter().cloned());
    }

    result
}

fn unfold_damaged(damaged: Vec<usize>, n: usize) -> Vec<usize> {
    let mut result = Vec::new();

    for _ in 0..n {
        result.extend(damaged.iter().cloned());
    }

    result
}

fn parse_spring_unfold(line: &str) -> (Vec<char>, Vec<usize>) {
    let (springs, damaged) = line.split_once(' ').unwrap();
    let springs = springs.chars()
        .collect();
    let damaged = damaged
        .split(',')
        .map(|m| m.parse::<usize>().unwrap())
        .collect_vec();
    (
        unfold_springs(springs, 5),
        unfold_damaged(damaged, 5)
    )
}

type Cache = HashMap<(Vec<char>, Vec<usize>), usize>;
fn solve_spring(springs: &[char], damaged: &[usize]) -> usize {
    let mut cache = HashMap::new();
    count_arrangements(springs, damaged, &mut cache)
}
fn count_arrangements(springs: &[char], damaged: &[usize], cache: &mut Cache) -> usize {
    if let Some(result) = cache.get(&(springs.to_vec(), damaged.to_vec())) {
        return *result;
    }
    if damaged.is_empty() {
        return (!springs.contains(&'#')) as usize;
    }

    let minimum_springs = damaged.iter().sum::<usize>() + damaged.len() - 1;

    if springs.len() < minimum_springs {
        return 0;
    }
    let result = match springs[0] {
        '.' => count_arrangements(&springs[1..], damaged,  cache),
        '#' => hash_branch(springs, damaged,  cache),
        '?' => count_arrangements(&springs[1..], damaged,  cache) + hash_branch(springs, damaged,  cache),
        _ => panic!("Invalid springs"),
    };
    cache.insert((springs.to_vec(), damaged.to_vec()), result);
    result
}

fn hash_branch(springs: &[char], damaged: &[usize], cache: &mut Cache) -> usize {
    if springs.len() < damaged[0] || springs[0..damaged[0]].contains(&'.') {
        return 0;
    }

    if springs.len() == damaged[0] {
        return (damaged.len() == 1) as usize;
    }

    if springs[damaged[0]] == '#' {
        return 0;
    }

    count_arrangements(&springs[damaged[0] + 1..], &damaged[1..], cache)
}

pub fn part_one(input: &str) -> Option<usize> {
    return Some(
        lines(input)
            .map(parse_spring)
            // .inspect(|(springs, damaged)| println!("{:?}", (springs, damaged)))
            .map(|(springs, damaged)| solve_spring(&springs, &damaged))
            .sum(),
    );
}

pub fn part_two(input: &str) -> Option<usize> {
    return Some(
        lines(input)
            .map(parse_spring_unfold)
            // .inspect(|(springs, damaged)| println!("{:?}", (springs, damaged)))
            .map(|(springs, damaged)| solve_spring(&springs, &damaged))
            .sum(),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(525152));
    }
}
