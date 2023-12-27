use advent_of_code::parsers::lines;
advent_of_code::solution!(1);

pub fn find_integer(line: &str) -> Option<char> {
    line.chars().find(|c| c.is_ascii_digit())
}

pub fn parse_line(line: &str) -> u32 {
    let first = find_integer(&line);
    let rev_line: String = line.chars().rev().collect();
    let last = find_integer(&rev_line);
    return format!(
        "{}{}",
        first.unwrap_or_else(|| '0'),
        last.unwrap_or_else(|| '0')
    )
    .parse::<u32>()
    .unwrap()
}

fn apply_subtitutions(line: &str) -> String {
    let mut result = String::from(line);
    let subs = [
        ("one", "o1e"),
        ("two", "t2o"),
        ("three", "t3e"),
        ("four", "f4r"),
        ("five", "f5e"),
        ("six", "s6x"),
        ("seven", "s7n"),
        ("eight", "e8t"),
        ("nine", "n9e"),
    ];
    for (find, replace) in subs {
        result = result.replace(find, replace);
    }

    result
}

pub fn part_one(input: &str) -> Option<u32> {
    return Some(lines(input).map(|line| parse_line(line)).sum());
}

pub fn part_two(input: &str) -> Option<u32> {
    return Some(lines(input).map(|line| parse_line(&apply_subtitutions(line))).sum());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(281));
    }
}
