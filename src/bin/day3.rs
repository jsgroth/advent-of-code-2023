//! Day 3: Gear Ratios
//!
//! <https://adventofcode.com/2023/day/3>

use std::cmp;
use std::error::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Space {
    Empty,
    Symbol(u8),
    Digit(u32),
}

fn parse_grid(input: &str) -> Vec<Vec<Space>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => Space::Empty,
                    '0'..='9' => Space::Digit(c.to_digit(10).unwrap()),
                    _ => Space::Symbol(c as u8),
                })
                .collect()
        })
        .collect()
}

fn solve_part_1(input: &str) -> u32 {
    let grid = parse_grid(input);
    let numbers = generate_number_grid(&grid);

    let mut sum = 0;
    for (i, row) in grid.iter().enumerate() {
        let mut last_was_digit = false;
        for (j, space) in row.iter().copied().enumerate() {
            let Space::Digit(_) = space else {
                last_was_digit = false;
                continue;
            };

            if last_was_digit {
                continue;
            }
            last_was_digit = true;

            let mut k = j + 1;
            while k < grid[i].len() && matches!(grid[i][k], Space::Digit(..)) {
                k += 1;
            }

            if is_symbol_nearby(&grid, i, j, k) {
                sum += numbers[i][j];
            }
        }
    }

    sum
}

fn generate_number_grid(grid: &[Vec<Space>]) -> Vec<Vec<u32>> {
    let mut numbers = vec![vec![0; grid[0].len()]; grid.len()];

    for (i, row) in grid.iter().enumerate() {
        let mut last_was_digit = false;

        for (j, space) in row.iter().copied().enumerate() {
            let Space::Digit(digit) = space else {
                last_was_digit = false;
                continue;
            };

            if last_was_digit {
                continue;
            }
            last_was_digit = true;

            let mut number = digit;
            let mut k = j + 1;
            while k < row.len() {
                let Space::Digit(next_digit) = row[k] else { break };
                number = 10 * number + next_digit;
                k += 1;
            }

            numbers[i][j..k].fill(number);
        }
    }

    numbers
}

fn is_symbol_nearby(grid: &[Vec<Space>], i: usize, j: usize, k: usize) -> bool {
    let col_range = j.saturating_sub(1)..cmp::min(k + 1, grid[0].len());

    if i > 0 {
        for col in col_range.clone() {
            if matches!(grid[i - 1][col], Space::Symbol(..)) {
                return true;
            }
        }
    }

    if i < grid.len() - 1 {
        for col in col_range {
            if matches!(grid[i + 1][col], Space::Symbol(..)) {
                return true;
            }
        }
    }

    if j > 0 && matches!(grid[i][j - 1], Space::Symbol(..)) {
        return true;
    }

    if k < grid.len() && matches!(grid[i][k], Space::Symbol(..)) {
        return true;
    }

    false
}

fn solve_part_2(input: &str) -> u32 {
    let grid = parse_grid(input);
    let numbers = generate_number_grid(&grid);

    let mut sum = 0;
    for (i, row) in grid.iter().enumerate() {
        for (j, space) in row.iter().copied().enumerate() {
            if space != Space::Symbol(b'*') {
                continue;
            }

            sum += compute_gear_ratio(&numbers, i, j);
        }
    }

    sum
}

fn compute_gear_ratio(numbers: &[Vec<u32>], i: usize, j: usize) -> u32 {
    let mut count = 0;
    let mut product = 1;

    if i > 0 {
        if numbers[i - 1][j] != 0 {
            count += 1;
            product *= numbers[i - 1][j];
        } else {
            if j > 0 && numbers[i - 1][j - 1] != 0 {
                count += 1;
                product *= numbers[i - 1][j - 1];
            }

            if j < numbers[i - 1].len() - 1 && numbers[i - 1][j + 1] != 0 {
                count += 1;
                product *= numbers[i - 1][j + 1];
            }
        }
    }

    if j > 0 && numbers[i][j - 1] != 0 {
        count += 1;
        product *= numbers[i][j - 1];
    }

    if j < numbers[i].len() - 1 && numbers[i][j + 1] != 0 {
        count += 1;
        product *= numbers[i][j + 1];
    }

    if i < numbers.len() - 1 {
        if numbers[i + 1][j] != 0 {
            count += 1;
            product *= numbers[i + 1][j];
        } else {
            if j > 0 && numbers[i + 1][j - 1] != 0 {
                count += 1;
                product *= numbers[i + 1][j - 1];
            }

            if j < numbers[i + 1].len() - 1 && numbers[i + 1][j + 1] != 0 {
                count += 1;
                product *= numbers[i + 1][j + 1];
            }
        }
    }

    if count == 2 { product } else { 0 }
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = advent_of_code_2023::read_input()?;

    let solution1 = solve_part_1(&input);
    println!("{solution1}");

    let solution2 = solve_part_2(&input);
    println!("{solution2}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("../sample/day3.txt");

    #[test]
    fn sample_input_part_1() {
        assert_eq!(solve_part_1(SAMPLE_INPUT), 4361);
    }

    #[test]
    fn sample_input_part_2() {
        assert_eq!(solve_part_2(SAMPLE_INPUT), 467835);
    }
}
