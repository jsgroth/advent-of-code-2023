//! Day 17: Clumsy Crucible
//!
//! <https://adventofcode.com/2023/day/17>
//!
//! Part 1: Use Dijkstra's algorithm to find the shortest path from the top-left corner to the bottom-right corner, while
//! keeping track of the number of spaces moved in the current direction in order to follow the restriction that the
//! crucible can't move more than three consecutive steps in a same direction. The map is treated as a graph where each
//! space is a node that has outgoing edges to each adjacent space that the crucible can legally move to, with the
//! weight of each edge set to the additional heat loss incurred by the move.
//!
//! Part 2: Essentially the same algorithm as part 1 but with additional restrictions on legal moves. These additional
//! restrictions also mean that simply reaching the bottom-right corner is not necessarily a solution - the crucible
//! must have moved at least 4 steps in the same direction when it reaches the destination or else it will not be able
//! to stop.

use advent_of_code_2023::impl_main;
use rustc_hash::FxHashMap;
use std::cmp::{Ordering, Reverse};
use std::collections::BinaryHeap;

fn parse_input(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).expect("Invalid digit in input")).collect())
        .collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

    fn rotate_left(self) -> Self {
        match self {
            Self::Up => Self::Left,
            Self::Left => Self::Down,
            Self::Down => Self::Right,
            Self::Right => Self::Up,
        }
    }

    fn rotate_right(self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct HeapEntry {
    i: u32,
    j: u32,
    direction: Direction,
    consecutive_moves: u32,
    heat_loss: u32,
}

impl PartialOrd for HeapEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HeapEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        self.heat_loss.cmp(&other.heat_loss)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct VisitedKey {
    i: u32,
    j: u32,
    direction: Direction,
    consecutive_moves: u32,
}

fn check_end_part_1(_consecutive_moves: u32) -> bool {
    true
}

fn check_end_part_2(consecutive_moves: u32) -> bool {
    // Can only stop on the end position if moved at least 4 spaces in the same direction
    consecutive_moves >= 4
}

fn check_direction_part_1(is_straight: bool, consecutive_moves: u32) -> bool {
    !(is_straight && consecutive_moves == 3)
}

fn check_direction_part_2(is_straight: bool, consecutive_moves: u32) -> bool {
    !((is_straight && consecutive_moves == 10) || (!is_straight && consecutive_moves < 4))
}

fn solve(
    input: &str,
    check_end: impl Fn(u32) -> bool,
    check_direction: impl Fn(bool, u32) -> bool,
) -> u32 {
    let map = parse_input(input);

    // Reverse because std BinaryHeap is a max heap
    let mut heap: BinaryHeap<Reverse<HeapEntry>> = BinaryHeap::new();

    heap.push(Reverse(HeapEntry {
        i: 1,
        j: 0,
        direction: Direction::Down,
        consecutive_moves: 1,
        heat_loss: map[1][0],
    }));
    heap.push(Reverse(HeapEntry {
        i: 0,
        j: 1,
        direction: Direction::Right,
        consecutive_moves: 1,
        heat_loss: map[0][1],
    }));

    let mut visited = FxHashMap::default();
    visited.insert(
        VisitedKey { i: 1, j: 0, direction: Direction::Down, consecutive_moves: 1 },
        map[1][0],
    );
    visited.insert(
        VisitedKey { i: 0, j: 1, direction: Direction::Right, consecutive_moves: 1 },
        map[0][1],
    );

    while let Some(Reverse(HeapEntry { i, j, direction, consecutive_moves, heat_loss })) =
        heap.pop()
    {
        if i == map.len() as u32 - 1 && j == map[0].len() as u32 - 1 && check_end(consecutive_moves)
        {
            return heat_loss;
        }

        for new_direction in [direction, direction.rotate_left(), direction.rotate_right()] {
            if !check_direction(new_direction == direction, consecutive_moves) {
                continue;
            }

            let (di, dj) = new_direction.di_dj();
            let new_i = i as i32 + di;
            let new_j = j as i32 + dj;
            if !(0..map.len() as i32).contains(&new_i) || !(0..map[0].len() as i32).contains(&new_j)
            {
                continue;
            }

            let new_heat_loss = heat_loss + map[new_i as usize][new_j as usize];
            let new_consecutive_moves =
                if new_direction == direction { consecutive_moves + 1 } else { 1 };

            let visited_key = VisitedKey {
                i: new_i as u32,
                j: new_j as u32,
                direction: new_direction,
                consecutive_moves: new_consecutive_moves,
            };
            if !visited
                .get(&visited_key)
                .is_some_and(|&existing_heat_loss| existing_heat_loss <= new_heat_loss)
            {
                heap.push(Reverse(HeapEntry {
                    i: new_i as u32,
                    j: new_j as u32,
                    direction: new_direction,
                    consecutive_moves: new_consecutive_moves,
                    heat_loss: new_heat_loss,
                }));
                visited.insert(visited_key, new_heat_loss);
            }
        }
    }

    panic!("Never reached destination")
}

fn solve_part_1(input: &str) -> u32 {
    solve(input, check_end_part_1, check_direction_part_1)
}

fn solve_part_2(input: &str) -> u32 {
    solve(input, check_end_part_2, check_direction_part_2)
}

impl_main!(p1: solve_part_1, p2: solve_part_2);

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("../../sample_input/day17.txt");
    const SAMPLE_INPUT_2: &str = include_str!("../../sample_input/day17-2.txt");

    #[test]
    fn sample_input_part_1() {
        assert_eq!(solve_part_1(SAMPLE_INPUT), 102);
    }

    #[test]
    fn sample_input_part_2() {
        assert_eq!(solve_part_2(SAMPLE_INPUT), 94);
        assert_eq!(solve_part_2(SAMPLE_INPUT_2), 71);
    }
}
