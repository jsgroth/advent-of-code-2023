//! Day 11: Cosmic Expansion
//!
//! <https://adventofcode.com/2023/day/11>

use advent_of_code_2023::impl_main;
use rustc_hash::FxHashSet;
use std::cmp;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Galaxy {
    i: i64,
    j: i64,
}

impl Galaxy {
    fn new(i: i64, j: i64) -> Self {
        Self { i, j }
    }

    fn distance_to(self, other: Self) -> i64 {
        (self.i - other.i).abs() + (self.j - other.j).abs()
    }
}

fn parse_input(input: &str) -> impl Iterator<Item = Galaxy> + '_ {
    input.lines().enumerate().flat_map(|(i, line)| {
        line.chars()
            .enumerate()
            .filter_map(move |(j, c)| (c == '#').then_some(Galaxy::new(i as i64, j as i64)))
    })
}

fn solve(input: &str, expansion_size: i64) -> i64 {
    let galaxies: FxHashSet<_> = parse_input(input).collect();

    let mut rows_with_galaxies = FxHashSet::default();
    let mut cols_with_galaxies = FxHashSet::default();

    let mut min_row = i64::MAX;
    let mut max_row = i64::MIN;

    let mut min_col = i64::MAX;
    let mut max_col = i64::MIN;

    for &galaxy in &galaxies {
        rows_with_galaxies.insert(galaxy.i);
        cols_with_galaxies.insert(galaxy.j);

        min_row = cmp::min(min_row, galaxy.i);
        max_row = cmp::max(max_row, galaxy.i);

        min_col = cmp::min(min_col, galaxy.j);
        max_col = cmp::max(max_col, galaxy.j);
    }

    let mut expanded_galaxies = Vec::new();

    let mut expanded_row = 0_i64;
    for i in min_row..=max_row {
        let mut expanded_col = 0_i64;
        for j in min_col..=max_col {
            if galaxies.contains(&Galaxy::new(i, j)) {
                expanded_galaxies.push(Galaxy::new(expanded_row, expanded_col));
            }

            expanded_col += 1;
            if !cols_with_galaxies.contains(&j) {
                expanded_col += expansion_size - 1;
            }
        }

        expanded_row += 1;
        if !rows_with_galaxies.contains(&i) {
            expanded_row += expansion_size - 1;
        }
    }

    let mut sum = 0;
    for (i, &galaxy_a) in expanded_galaxies.iter().enumerate() {
        for &galaxy_b in &expanded_galaxies[i + 1..] {
            sum += galaxy_a.distance_to(galaxy_b);
        }
    }

    sum
}

const PART_1_EXPANSION_SIZE: i64 = 2;
const PART_2_EXPANSION_SIZE: i64 = 1_000_000;

fn solve_part_1(input: &str) -> i64 {
    solve(input, PART_1_EXPANSION_SIZE)
}

fn solve_part_2(input: &str) -> i64 {
    solve(input, PART_2_EXPANSION_SIZE)
}

impl_main!(p1: solve_part_1, p2: solve_part_2);

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("../../sample_input/day11.txt");

    #[test]
    fn sample_input_part_1() {
        assert_eq!(solve(SAMPLE_INPUT, 2), 374);
    }

    #[test]
    fn sample_input_part_2() {
        assert_eq!(solve(SAMPLE_INPUT, 10), 1030);
        assert_eq!(solve(SAMPLE_INPUT, 100), 8410);
    }
}
