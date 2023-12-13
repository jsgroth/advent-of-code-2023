//! Day 13: Point of Incidence
//!
//! <https://adventofcode.com/2023/day/13>

use advent_of_code_2023::impl_main;

fn parse_input(input: &str) -> Vec<Vec<Vec<bool>>> {
    let mut lines = input.lines().peekable();

    let mut grids = Vec::new();
    while lines.peek().is_some() {
        let grid = lines
            .by_ref()
            .take_while(|line| !line.is_empty())
            .map(|line| line.chars().map(|c| c == '#').collect())
            .collect();
        grids.push(grid);
    }

    grids
}

fn solve(input: &str, target_differences: u64) -> u64 {
    let grids = parse_input(input);

    grids
        .iter()
        .map(|grid| {
            for row in 1..grid.len() {
                if count_mirror_diffs_row(grid, row) == target_differences {
                    return 100 * row as u64;
                }
            }

            for col in 1..grid[0].len() {
                if count_mirror_diffs_col(grid, col) == target_differences {
                    return col as u64;
                }
            }

            panic!("No reflection found in grid: {grid:?}")
        })
        .sum()
}

fn count_mirror_diffs_row(grid: &[Vec<bool>], row: usize) -> u64 {
    let mut diffs = 0;
    let mut i = row - 1;
    let mut j = row;
    loop {
        diffs += (0..grid[0].len()).filter(|&col| grid[i][col] != grid[j][col]).count() as u64;

        if i == 0 || j == grid.len() - 1 {
            break;
        }

        i -= 1;
        j += 1;
    }

    diffs
}

fn count_mirror_diffs_col(grid: &[Vec<bool>], col: usize) -> u64 {
    let mut diffs = 0;
    let mut i = col - 1;
    let mut j = col;
    loop {
        diffs += (0..grid.len()).filter(|&row| grid[row][i] != grid[row][j]).count() as u64;

        if i == 0 || j == grid[0].len() - 1 {
            break;
        }

        i -= 1;
        j += 1;
    }

    diffs
}

fn solve_part_1(input: &str) -> u64 {
    solve(input, 0)
}

fn solve_part_2(input: &str) -> u64 {
    solve(input, 1)
}

impl_main!(p1: solve_part_1, p2: solve_part_2);

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("../sample/day13.txt");

    #[test]
    fn sample_input_part_1() {
        assert_eq!(solve_part_1(SAMPLE_INPUT), 405);
    }

    #[test]
    fn sample_input_part_2() {
        assert_eq!(solve_part_2(SAMPLE_INPUT), 400);
    }
}
