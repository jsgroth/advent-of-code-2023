//! Day 16: The Floor Will Be Lava
//!
//! <https://adventofcode.com/2023/day/16>
//!
//! Part 1: Path tracing, with the twist that certain spaces cause the path to split in two and move in both directions
//! simultaneously. In order to avoid possible infinite loops, the algorithm short circuits if a given tile has already
//! been visited while facing the given direction.
//!
//! Part 2: This is just a brute force search finding the max number of spaces touched across every possible starting
//! position and direction.

use advent_of_code_2023::impl_main;
use std::cmp;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Left,
    Right,
    Down,
}

impl Direction {
    fn di_dj(self) -> (i32, i32) {
        match self {
            Self::Up => (-1, 0),
            Self::Down => (1, 0),
            Self::Left => (0, -1),
            Self::Right => (0, 1),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Space {
    Empty,
    // '-'
    HorizontalSplitter,
    // '|'
    VerticalSplitter,
    // '/'
    ForwardMirror,
    // '\'
    BackwardMirror,
}

impl From<char> for Space {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Empty,
            '-' => Self::HorizontalSplitter,
            '|' => Self::VerticalSplitter,
            '/' => Self::ForwardMirror,
            '\\' => Self::BackwardMirror,
            _ => panic!("Invalid space char: {value}"),
        }
    }
}

fn parse_input(input: &str) -> Vec<Vec<Space>> {
    input.lines().map(|line| line.chars().map(Space::from).collect()).collect()
}

fn solve_part_1(input: &str) -> u32 {
    let grid = parse_input(input);

    count_energized(&grid, 0, 0, Direction::Right)
}

fn solve_part_2(input: &str) -> u32 {
    let grid = parse_input(input);

    let mut max = u32::MIN;
    for i in 0..grid.len() {
        max = cmp::max(max, count_energized(&grid, i, 0, Direction::Right));
        max = cmp::max(max, count_energized(&grid, i, grid[0].len() - 1, Direction::Left));
    }

    for j in 0..grid[0].len() {
        max = cmp::max(max, count_energized(&grid, 0, j, Direction::Down));
        max = cmp::max(max, count_energized(&grid, grid.len() - 1, j, Direction::Up));
    }

    max
}

type VisitedGrid = Vec<Vec<DirectionBits>>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct DirectionBits(u8);

impl DirectionBits {
    fn new() -> Self {
        Self(0)
    }

    fn contains(self, direction: Direction) -> bool {
        self.0 & direction_bit_for(direction) != 0
    }

    fn set(&mut self, direction: Direction) {
        self.0 |= direction_bit_for(direction);
    }

    fn is_empty(self) -> bool {
        self.0 == 0
    }
}

fn direction_bit_for(direction: Direction) -> u8 {
    match direction {
        Direction::Left => 1 << 0,
        Direction::Right => 1 << 1,
        Direction::Up => 1 << 2,
        Direction::Down => 1 << 3,
    }
}

fn count_energized(
    grid: &[Vec<Space>],
    start_i: usize,
    start_j: usize,
    start_direction: Direction,
) -> u32 {
    let mut visited: VisitedGrid = vec![vec![DirectionBits::new(); grid[0].len()]; grid.len()];

    visit(grid, start_i as i32, start_j as i32, start_direction, &mut visited);

    visited
        .into_iter()
        .map(|row| row.into_iter().filter(|directions| !directions.is_empty()).count())
        .sum::<usize>() as u32
}

fn visit(grid: &[Vec<Space>], i: i32, j: i32, direction: Direction, visited: &mut VisitedGrid) {
    if !(0..grid.len() as i32).contains(&i)
        || !(0..grid[i as usize].len() as i32).contains(&j)
        || visited[i as usize][j as usize].contains(direction)
    {
        return;
    }
    visited[i as usize][j as usize].set(direction);

    let space = grid[i as usize][j as usize];
    if space == Space::HorizontalSplitter && matches!(direction, Direction::Up | Direction::Down) {
        visit(grid, i, j - 1, Direction::Left, visited);
        visit(grid, i, j + 1, Direction::Right, visited);
    } else if space == Space::VerticalSplitter
        && matches!(direction, Direction::Left | Direction::Right)
    {
        visit(grid, i - 1, j, Direction::Up, visited);
        visit(grid, i + 1, j, Direction::Down, visited);
    } else {
        let new_direction =
            match (space, direction) {
                (Space::Empty | Space::HorizontalSplitter | Space::VerticalSplitter, _) => {
                    direction
                }
                (Space::ForwardMirror, Direction::Right)
                | (Space::BackwardMirror, Direction::Left) => Direction::Up,
                (Space::ForwardMirror, Direction::Left)
                | (Space::BackwardMirror, Direction::Right) => Direction::Down,
                (Space::ForwardMirror, Direction::Up)
                | (Space::BackwardMirror, Direction::Down) => Direction::Right,
                (Space::ForwardMirror, Direction::Down)
                | (Space::BackwardMirror, Direction::Up) => Direction::Left,
            };

        let (di, dj) = new_direction.di_dj();

        visit(grid, i + di, j + dj, new_direction, visited);
    }
}

impl_main!(p1: solve_part_1, p2: solve_part_2);

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("../../sample_input/day16.txt");

    #[test]
    fn sample_input_part_1() {
        assert_eq!(solve_part_1(SAMPLE_INPUT), 46);
    }

    #[test]
    fn sample_input_part_2() {
        assert_eq!(solve_part_2(SAMPLE_INPUT), 51);
    }
}
