//! Day 16: The Floor Will Be Lava
//!
//! <https://adventofcode.com/2023/day/16>

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
    fn di(self) -> i32 {
        match self {
            Self::Up => -1,
            Self::Down => 1,
            Self::Left | Self::Right => 0,
        }
    }

    fn dj(self) -> i32 {
        match self {
            Self::Left => -1,
            Self::Right => 1,
            Self::Up | Self::Down => 0,
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

fn count_energized(
    grid: &[Vec<Space>],
    start_i: usize,
    start_j: usize,
    start_direction: Direction,
) -> u32 {
    let mut visited: Vec<Vec<Vec<Direction>>> = vec![vec![vec![]; grid[0].len()]; grid.len()];

    let initial_directions = determine_new_directions(start_direction, grid[start_i][start_j]);

    let mut current_beams = Vec::new();
    for initial_direction in initial_directions {
        current_beams.push((start_i, start_j, initial_direction));
        visited[start_i][start_j].push(initial_direction);
    }

    while let Some((i, j, direction)) = current_beams.pop() {
        let i = i as i32;
        let j = j as i32;
        let di = direction.di();
        let dj = direction.dj();

        if !(0..grid.len() as i32).contains(&(i + di))
            || !(0..grid[0].len() as i32).contains(&(j + dj))
        {
            continue;
        }

        let new_i = (i + di) as usize;
        let new_j = (j + dj) as usize;
        let new_directions = determine_new_directions(direction, grid[new_i][new_j]);

        for new_direction in new_directions {
            if !visited[new_i][new_j].contains(&new_direction) {
                current_beams.push((new_i, new_j, new_direction));
                visited[new_i][new_j].push(new_direction);
            }
        }
    }

    visited
        .into_iter()
        .map(|row| row.into_iter().filter(|directions| !directions.is_empty()).count())
        .sum::<usize>() as u32
}

fn determine_new_directions(direction: Direction, space: Space) -> Vec<Direction> {
    match (direction, space) {
        (_, Space::Empty)
        | (Direction::Up | Direction::Down, Space::VerticalSplitter)
        | (Direction::Left | Direction::Right, Space::HorizontalSplitter) => {
            vec![direction]
        }
        (Direction::Left | Direction::Right, Space::VerticalSplitter) => {
            vec![Direction::Up, Direction::Down]
        }
        (Direction::Up | Direction::Down, Space::HorizontalSplitter) => {
            vec![Direction::Left, Direction::Right]
        }
        (Direction::Left, Space::ForwardMirror) | (Direction::Right, Space::BackwardMirror) => {
            vec![Direction::Down]
        }
        (Direction::Right, Space::ForwardMirror) | (Direction::Left, Space::BackwardMirror) => {
            vec![Direction::Up]
        }
        (Direction::Up, Space::ForwardMirror) | (Direction::Down, Space::BackwardMirror) => {
            vec![Direction::Right]
        }
        (Direction::Down, Space::ForwardMirror) | (Direction::Up, Space::BackwardMirror) => {
            vec![Direction::Left]
        }
    }
}

impl_main!(p1: solve_part_1, p2: solve_part_2);

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("../sample/day16.txt");

    #[test]
    fn sample_input_part_1() {
        assert_eq!(solve_part_1(SAMPLE_INPUT), 46);
    }

    #[test]
    fn sample_input_part_2() {
        assert_eq!(solve_part_2(SAMPLE_INPUT), 51);
    }
}
