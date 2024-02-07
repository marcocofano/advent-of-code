use core::{fmt, panic};
use itertools::Itertools;
use std::{
    collections::{BinaryHeap, HashMap},
    fmt::Debug,
    io::Seek,
    usize,
};

advent_of_code::solution!(17);

type Point = (isize, isize);

type PriorityQueue = BinaryHeap<State>;
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct HeapNode {
    position: Point,
    direction: Point,
    direction_count: usize,
}

impl HeapNode {
    fn new(position: Point, direction: Point, direction_count: usize) -> Self {
        Self {
            position,
            direction,
            direction_count,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct State {
    weight: usize,
    node: HeapNode,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .weight
            .cmp(&self.weight)
            .then_with(|| self.node.cmp(&other.node))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(other.weight.cmp(&self.weight))
    }
}

struct GridDebug {
    grid: Vec<Vec<char>>,
    size: usize,
}

impl fmt::Debug for GridDebug {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "[")?;

        for row in 0..self.size {
            write!(f, "  [")?;

            for col in 0..self.size {
                write!(f, "{} ", self.grid[row][col])?;
            }
            writeln!(f, "]")?;
        }
        writeln!(f, "]")
    }
}

struct Grid {
    grid: Vec<Vec<usize>>,
    size: usize,
}

impl fmt::Debug for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "[")?;

        for row in 0..self.size {
            write!(f, "  [")?;

            for col in 0..self.size {
                write!(f, "{} ", self.grid[row][col])?;
            }
            writeln!(f, "]")?;
        }
        writeln!(f, "]")
    }
}
impl Grid {
    fn is_valid(&self, point: Point) -> bool {
        return point.0 >= 0
            && point.0 < self.size as isize
            && point.1 >= 0
            && point.1 < self.size as isize;
    }
    fn get(&self, coordinates: &Point) -> Option<usize> {
        if self.is_valid(*coordinates) {
            Some(self.grid[coordinates.0 as usize][coordinates.1 as usize])
        } else {
            None
        }
    }
    fn get_adj(&self, node: &HeapNode) -> Vec<HeapNode> {
        let mut adjs = Vec::new();
        for new_direction in [(0, -1), (-1, 0), (0, 1), (1, 0)] {
            let new_position = (
                node.position.0 + new_direction.0,
                node.position.1 + new_direction.1,
            );
            if self.is_valid(new_position) {
                if (
                    node.direction.0 + new_direction.0,
                    node.direction.1 + new_direction.1,
                ) == (0, 0)
                {
                    continue;
                } else if new_direction != node.direction {
                    adjs.push(HeapNode::new(new_position, new_direction, 1));
                } else if node.direction_count < 3 {
                    adjs.push(HeapNode::new(
                        new_position,
                        new_direction,
                        node.direction_count + 1,
                    ));
                }
            }
        }
        adjs
    }
}
fn parse_grid(input: &str) -> Grid {
    let grid = input
        .lines()
        .map(|l| {
            l.chars()
                .filter_map(|c| c.to_digit(10))
                .map(|d| d as usize)
                .collect_vec()
        })
        .collect_vec();
    let size = grid[0].len();
    Grid { grid, size }
}

fn parse_grid_debug(input: &str) -> GridDebug {
    let grid = input.lines().map(|l| l.chars().collect_vec()).collect_vec();
    GridDebug {
        grid: grid.clone(),
        size: grid[0].len(),
    }
}

fn dijikistra_find_shortest(grid: &Grid, start_point: Point) -> HashMap<HeapNode, usize> {
    let mut shortest = HashMap::new();
    let mut prio = PriorityQueue::new();
    shortest.insert(HeapNode::new(start_point, (1, 0), 0), 0);
    shortest.insert(HeapNode::new(start_point, (0, 1), 0), 0);
    prio.push(State {
        weight: 0,
        node: HeapNode::new(start_point, (1, 0), 0),
    });
    prio.push(State {
        weight: 0,
        node: HeapNode::new(start_point, (0, 1), 0),
    });
    while let Some(State { weight, node }) = prio.pop() {
        if node.position == (grid.size as isize - 1, grid.size as isize - 1) {
            break;
        }
        for adj_node in grid.get_adj(&node) {
            let new_weight = weight + grid.get(&adj_node.position).unwrap();
            if let Some(&best_weight) = shortest.get(&adj_node) {
                if new_weight >= best_weight {
                    continue;
                }
            }
            shortest.insert(adj_node.clone(), new_weight);
            prio.push(State {
                weight: new_weight,
                node: adj_node,
            })
        }
    }
    return shortest;
}

fn print_result(grid: &mut GridDebug, shortest: &HashMap<HeapNode, usize>) {
    let start_point = (0, 0);
    let mut point = (grid.size as isize - 1, grid.size as isize - 1);
    let mut current_dir = (0, 0);
    let mut count = 1;
    let mut old_dir = current_dir.clone();
    while point != start_point {
        // let current_node_list = shortest
        //     .iter()
        //     .filter(|(key, _)| key.position == point)
        //     .collect_vec();
        // dbg!(&grid);
        // dbg!(current_node_list);
        let current_node = shortest
            .iter()
            .filter(|(key, _)| key.position == point && !(key.direction == current_dir && count == 3))
            .min_by_key(|(_, &value)| value);
        // dbg!(current_node);
        current_dir = match current_node {
            Some((node, _)) => node.direction,
            _ => panic!("No direction found, cannot build a debug grid"),
        };
        if old_dir == current_dir {
            count += 1;
        } else {
            count = 1;
        }
        old_dir = current_dir.clone();
        let symbol = match current_dir {
            (-1, 0) => '^',
            (1, 0) => 'v',
            (0, -1) => '<',
            (0, 1) => '>',
            _ => panic!("No direction"),
        };
        grid.grid[point.0 as usize][point.1 as usize] = symbol;
        point = (point.0 - current_dir.0, point.1 - current_dir.1);
    }
    dbg!(grid);
}
pub fn part_one(input: &str) -> Option<usize> {
    let city = parse_grid(input);
    let mut city_debug = parse_grid_debug(input);
    dbg!(&city);
    let start_point = (0, 0);
    let end_point = (city.size as isize - 1, city.size as isize - 1);
    let distances = dijikistra_find_shortest(&city, start_point);
    print_result(&mut city_debug, &distances);
    let shortest = distances
        .iter()
        .filter(|(key, _)| key.position == end_point)
        .min_by_key(|(_, &value)| value);
    let result = match shortest {
        Some((_, value)) => Some(*value),
        None => None,
    };
    result
}

pub fn part_two(_input: &str) -> Option<u32> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(102));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
