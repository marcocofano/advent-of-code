#![allow(dead_code)]

use std::collections::HashSet;

use advent_of_code::parsers::lines;
use itertools::Itertools;
advent_of_code::solution!(16);

type Point = (i32, i32);

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
enum Direction {
    Right,
    Left,
    Up,
    Down,
}

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
struct State {
    position: Point,
    direction: Direction,
}

impl State {
    fn hor_direction(&self) -> bool {
        return self.direction == Direction::Right || self.direction == Direction::Left;
    }
    fn go_straight(&self) -> State {
        let new_position = match self.direction {
            Direction::Right => (self.position.0 + 0, self.position.1 + 1),
            Direction::Left => (self.position.0 + 0, self.position.1 - 1),
            Direction::Up => (self.position.0 - 1, self.position.1 + 0),
            Direction::Down => (self.position.0 + 1, self.position.1 + 0),
        };
        State {
            position: new_position,
            direction: self.direction,
        }
    }
    fn turn(&self, direction: Direction) -> State {
        State {
            position: self.position,
            direction,
        }
    }
    fn reflect_slash(&self) -> State {
        let new_direction = match self.direction {
            Direction::Right => Direction::Up,
            Direction::Left => Direction::Down,
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
        };
        State {
            position: self.position,
            direction: new_direction,
        }
    }
    fn reflect_backslash(&self) -> State {
        let new_direction = match self.direction {
            Direction::Right => Direction::Down,
            Direction::Left => Direction::Up,
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
        };
        State {
            position: self.position,
            direction: new_direction,
        }
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
enum Tiles {
    HorizontalSplit,
    VerticalSplit,
    SlashMirror,
    BackSlashMirror,
    Dot,
}

#[derive(Debug)]
struct Grid {
    tiles: Vec<Vec<Tiles>>,
    width: usize,
    height: usize,
}

impl Grid {
    fn new(input: &str) -> Grid {
        let tiles: Vec<Vec<Tiles>> = lines(input)
            .map(|l| {
                l.chars()
                    .map(|c| match c {
                        '-' => Tiles::HorizontalSplit,
                        '|' => Tiles::VerticalSplit,
                        '/' => Tiles::SlashMirror,
                        '\\' => Tiles::BackSlashMirror,
                        '.' => Tiles::Dot,
                        _ => panic!("Tile not valid!"),
                    })
                    .collect()
            })
            .collect();

        Grid {
            height: tiles.len(),
            width: tiles[0].len(),
            tiles,
        }
    }
    fn get_tile(&self, position: &Point) -> Option<Tiles> {
        if (0..self.width).contains(&(position.0 as usize))
            && (0..self.height).contains(&(position.1 as usize))
        {
            Some(self.tiles[position.0 as usize][position.1 as usize])
        } else {
            None
        }
    }

    fn energize(&self, state: State, memo: &mut HashSet<State>) {
        if memo.contains(&state) {
            return;
        }

        if let Some(tile) = self.get_tile(&state.position) {
            memo.insert(state);

            match tile {
                Tiles::Dot => self.energize(state.go_straight(), memo),
                Tiles::VerticalSplit => {
                    if state.hor_direction() {
                        let up = state.turn(Direction::Up);
                        let down = state.turn(Direction::Down);
                        self.energize(up.go_straight(), memo);
                        self.energize(down.go_straight(), memo);
                    } else {
                        self.energize(state.go_straight(), memo);
                    }
                }
                Tiles::HorizontalSplit => {
                    if state.hor_direction() {
                        self.energize(state.go_straight(), memo);
                    } else {
                        let right = state.turn(Direction::Right);
                        let left = state.turn(Direction::Left);
                        self.energize(right.go_straight(), memo);
                        self.energize(left.go_straight(), memo);
                    }
                }
                Tiles::BackSlashMirror => {
                    self.energize(state.reflect_backslash().go_straight(), memo);
                }
                Tiles::SlashMirror => {
                    self.energize(state.reflect_slash().go_straight(), memo);
                }
            }
        }
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let grid = Grid::new(input);
    let mut memo: HashSet<State> = HashSet::new();
    let starting_state = State {
        position: (0, 0),
        direction: Direction::Right,
    };
    grid.energize(starting_state, &mut memo);
    Some(memo.iter().map(|s| s.position).unique().count())
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut results: Vec<usize> = vec![];
    let mut starting_states: Vec<State> = vec![];
    let grid = Grid::new(input);
    for i in 0..grid.width {
        starting_states.extend([
            State {
                position: (0, i as i32),
                direction: Direction::Down,
            },
            State {
                position: (i as i32, (grid.width - 1) as i32),
                direction: Direction::Left,
            },
            State {
                position: ((grid.height - 1) as i32, i as i32),
                direction: Direction::Up,
            },
            State {
                position: (i as i32, 0),
                direction: Direction::Right,
            },
        ])
    }
    for ss in starting_states {
        let mut memo: HashSet<State> = HashSet::new();
        grid.energize(ss, &mut memo);
        results.push(memo.iter().map(|s| s.position).unique().count());
    }
    results.iter().max().copied()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(51));
    }
}
