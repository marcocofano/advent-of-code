use advent_of_code::parsers::lines;
use itertools::Itertools;
advent_of_code::solution!(4);

pub fn parse_scratchcard(line: &str) -> Option<u32> {
    let game = line.split_once(": ").unwrap_or_else(|| ("", ""));
    let scratchcard = game.1.split_once("| ").unwrap_or_else(|| ("", ""));
    let winning: Vec<&str> = scratchcard.0.split(" ").filter(|&s| s != "").collect();
    let numbers: Vec<&str> = scratchcard.1.split(" ").filter(|&s| s != "").collect();
    return Some(numbers.iter().filter(|&s| winning.contains(s)).cloned().collect_vec().len() as u32)
}

pub fn score_rule(num_wins: u32) -> Option<u32> {
    if num_wins > 0 {
        return Some(2u32.pow((num_wins - 1) as u32))
    } else {
        return Some(0);
    }
}



pub fn part_one(input: &str) -> Option<u32> {
    return Some(lines(input).map(|line| score_rule(parse_scratchcard(line).unwrap_or_default()).unwrap_or_default()).sum());
}


pub fn part_two(input: &str) -> Option<u32> {
    let lns = lines(input).collect_vec();
    let mut scratchcards = vec![0; lns.len()];
    for (r, &l) in lns.iter().enumerate() {
        scratchcards[r] += 1;
        let score = parse_scratchcard(l).unwrap_or_default();
        for i in 0..score as usize {
            if r + 1 + i < lns.len() {
                scratchcards[r + 1 + i] += scratchcards[r];
            }
        }
    }
    return Some(scratchcards.iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
 
