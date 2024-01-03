use std::collections::VecDeque;

use itertools::Itertools;
advent_of_code::solution!(6);
fn parse_lines(line: &mut VecDeque<&str>) -> Vec<f64> {
    return line
        .pop_front()
        .unwrap()
        .split_once(":")
        .unwrap_or_default()
        .1
        .trim()
        .split_ascii_whitespace()
        .map(|t| t.parse::<f64>().unwrap_or_default())
        .collect_vec();
}

pub fn find_solution(time: f64, distance: f64) -> u32 {
    let discriminant = time * time - 4.0 * distance;

    let sqrt_discriminant = f64::sqrt(discriminant);
    let x1 = (-time + sqrt_discriminant) / 2.0;
    let x2 = (-time - sqrt_discriminant) / 2.0;

    let mut result = (x1.ceil() - x2.ceil()).abs() as u32;
    if x1.fract() == 0.0  && x2.fract() == 0.0 {
        result =  result.saturating_sub(1);
    }
    result
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut input_data: VecDeque<&str> = input.lines().collect();
    let times = parse_lines(&mut input_data);
    let distances = parse_lines(&mut input_data);
    return Some(
        times.iter()
            .zip(distances)
            .map(|(&t, d)| find_solution(t, d))
            .fold(1, |acc, x| acc * x),
    )
}

fn parse_lines_2(line: &mut VecDeque<&str>) -> f64 {
    return line
        .pop_front()
        .unwrap()
        .split_once(":")
        .unwrap_or_default()
        .1
        .replace(' ', "")
        .parse::<f64>().unwrap_or_default();
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut input_data: VecDeque<&str> = input.lines().collect();
    let time = parse_lines_2(&mut input_data);
    let distance = parse_lines_2(&mut input_data);
    Some(find_solution(time, distance))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}
