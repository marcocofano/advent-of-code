use advent_of_code::parsers::lines;

advent_of_code::solution!(9);

// Found a potential closed formula for the solution, starting from the number to be found and working
// backwards. Clearly the Tartaglia's triangle and the binomial was
// involved. 
// Then proved the guessed formula for a general sequence of numbers by induction.

fn parse_line(line: &str) -> Vec<i64> {
    line.split_ascii_whitespace()
        .map(|v| v.parse().unwrap())
        .collect()
}
fn binomial_coefficient(n: i64, k: i64) -> i64 {
    if k > n {
        return 0;
    }
    let mut result = 1;
    for i in 0..k {
        result *= n - i;
        result /= i + 1;
    }
    result
}

fn compute_next_value(numbers: Vec<i64>) -> i64 {
    let n = numbers.len();
    let mut sum = 0;

    for j in 0..n {
        let coefficient = binomial_coefficient(n as i64, j as i64);
        // println!("n: {}. j: {}, binomial {}", n, j, coefficient);
        if (n-j+1) % 2 == 0 {
            sum += (numbers[j]) * coefficient;
            // println!(" number: {}, sum {}",numbers[j], sum);
        }
        else {
            sum -= (numbers[j]) * coefficient;
            // println!(" number: {}, sum {}",numbers[j], sum);
        }
    }
    // println!("{}", sum);
    sum
}

fn compute_previous_value(numbers: Vec<i64>) -> i64 {
    let n = numbers.len();
    let mut sum = 0;

    for j in 0..n {
        let coefficient = binomial_coefficient(n as i64, (j + 1) as i64);
        if (j) % 2 == 0 {
            sum += (numbers[j]) * coefficient;
        }
        else {
            sum -= (numbers[j]) * coefficient;
        }
    }
    sum
}
// The formula gives any X_j (once you include the added new value) in terms of the others so just rearranging the X's works as well.
// For part one we use the indices:
// X_0   X_1       ...         X_n-1   X_n
//    A_0   A_1    ...   A_n-1     A_n
//      B_0    B_1 ... B_m-2  B_n-1  
//                 ...
// For part two we reassign them with X_(-1) as the added one.
// We compute them respectively X_N,
//  as a function of the other X_j's as:
//  X_n = \Sum_{j=0}^{n-1} x_j \choose(n, j)(-1)^{n-j+1}.
//
// So , for part two we just reshuffle the formula and the indices. 
// X_(-1) = \Sum_{j=0}^{n-1} x_j \choose(n, j + 1)(-1)^{j}. (the indices are chosen to make the
// code clearer, given that vectors start from 0)

pub fn part_one(input: &str) -> Option<i64> {
    Some(lines(input).map(parse_line).map(|history| compute_next_value(history)).sum())
}

pub fn part_two(input: &str) -> Option<i64> {
    Some(lines(input).map(parse_line).map(|history| compute_previous_value(history)).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
