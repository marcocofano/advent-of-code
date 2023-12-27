use advent_of_code::parsers::lines;
use std::cmp::max;
advent_of_code::solution!(2);


fn validate(num: u32, color: &str) -> bool {
    match color {
        "red" => num <= 12,
        "green" => num <= 13,
        "blue" => num <= 14,
        _ => false, 
    }
}

pub fn parse_game(line: &str) -> Option<u32> {
    let mut split_game  = line.split(": ");
    let game_number = split_game.next().expect("No Game found");
    let sets_str = split_game.next()
        .expect("No Sets found").replace(";", ",");
    let sets = sets_str.split(", ");
    for draw in sets {
        let (number, color): (&str, &str) = draw
            .trim()
            .split_once(' ')
            .unwrap_or_else(|| panic!("Failed to parse game set"));
        let count = number.parse::<u32>().expect("Error in parsing number");
        if validate(count, color) == true {
            continue
        }
        else {
            return None
        }
    };
    return Some(game_number.split_once(" ").unwrap_or_else(|| ("", "0")).1
                .parse::<u32>().unwrap_or_else(|_| 0));
}

#[derive(Debug, Default)]
struct MinimumSet {
    red: u32,
    green: u32,
    blue: u32,
}

pub fn parse_game_2(line: &str) -> u32 {
    let mut split_game  = line.split(": ");
    let _game_number = split_game.next().expect("No Game found");
    let sets_str = split_game.next()
        .expect("No Sets found").replace(";", ",");
    let sets = sets_str.split(", ");
    let mut minimum_set = MinimumSet::default();
    for draw in sets {
        let (number, color): (&str, &str) = draw
            .trim()
            .split_once(' ')
            .unwrap_or_else(|| panic!("Failed to parse game set"));
        let count = number.parse::<u32>().expect("Error in parsing number");
        match color {
            "red" => minimum_set.red = max(minimum_set.red, count),
            "green" => minimum_set.green = max(minimum_set.green, count),
            "blue" => minimum_set.blue = max(minimum_set.blue, count),
            _ => continue
        }
    } 
    return minimum_set.red * minimum_set.green * minimum_set.blue 
}

pub fn part_one(input: &str) -> Option<u32> {
    return Some(lines(input).map(|line| parse_game(line).unwrap_or_default()).sum());
}
pub fn part_two(input: &str) -> Option<u32> {
    return Some(lines(input).map(|line| parse_game_2(line)).sum());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
