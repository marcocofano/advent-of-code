use advent_of_code::parsers::lines;
use itertools::Itertools;

advent_of_code::solution!(11);

type Point = (usize, usize);

fn find_galaxies(ix: usize, line: &str) -> Option<Vec<Point>> {
    let columns = line.match_indices("#").map(|(i, _)| i).collect_vec();
    let mut result = vec![];
    for col in columns {
        result.push((ix, col));
    }
    Some(result)
}

fn manhattan_distance(point1: Point, point2: Point) -> usize {
    point1.0.abs_diff(point2.0) + point1.1.abs_diff(point2.1)
}

fn solve_expansion(grid: Vec<&str>, expansion: usize) -> Option<usize> {
    // Can be refactored. Some steps can be extracted to their own functions
    let mut points = vec![];
    for (r, row) in grid.iter().enumerate() {
        points.extend(find_galaxies(r, row)?);
    }
    // Find the maximum row and column values from the points
    let max_row = points.iter().map(|&(row, _)| row).max().unwrap_or(0);
    let max_col = points.iter().map(|&(_, col)| col).max().unwrap_or(0);

    // Specify the maximum number of rows and columns
    let max_rows = max_row + 1; // Add 1 to include the maximum row
    let max_columns = max_col + 1; // Add 1 to include the maximum column

    // Find unique row and column values from the points
    let unique_rows: Vec<usize> = points.iter().map(|&(row, _)| row).collect();
    let unique_columns: Vec<usize> = points.iter().map(|&(_, col)| col).collect();

    // Find missing rows and columns, already add the expansion
    // Expanding the missing ros/cols is probably not needed if we compare the missing row and columns with the fixed
    // pre expansion point, instead of comparing with the expanded points, see next step. TODO: REFACTOR
    let missing_rows: Vec<usize> = (0..max_rows)
        .filter(|&row| !unique_rows.contains(&row))
        .enumerate()
        .map(|(index, value)| value + index*expansion)
        .collect();
    let missing_columns: Vec<usize> = (0..max_columns)
        .filter(|&col| !unique_columns.contains(&col))
        .enumerate()
        .map(|(index, value)| value + index*expansion)
        .collect();
    // Expand the universe, Hubble
    // TODO: REFACTOR. if we keep reference to the initial point coordinates we can avoid expanding the missing
    // row, and cols
    for &mr in &missing_rows {
        for (x, y) in &mut points {
            if *x > mr {
                (*x, *y) = (*x + expansion, *y);
            }
        }
    }
    for &mc in &missing_columns {
        for (x, y) in &mut points {
            if *y > mc {
                (*x, *y) = (*x, *y + expansion);
            }
        }
    }
    // println!("{:?}", points);

    let mut total_distance = 0;
    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            total_distance += manhattan_distance(points[i], points[j]);
        }
    }
    Some(total_distance)
}

pub fn part_one(input: &str) -> Option<usize> {
    let grid = lines(input).collect_vec();
    return solve_expansion(grid, 1)
}

pub fn part_two(input: &str) -> Option<usize> {
    let grid = lines(input).collect_vec();
    return solve_expansion(grid, 999999)
    }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(374));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8410));
    }
}
