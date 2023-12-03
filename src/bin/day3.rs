//! Day 3: Gear Ratios
//!
//! <https://adventofcode.com/2023/day/3>

use arrayvec::ArrayVec;
use std::collections::HashSet;
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
struct IndexedNumber {
    index: u32,
    number: u32,
}

fn solve_part_1(input: &str) -> u32 {
    let grid = parse_grid(input);
    let numbers = generate_number_grid(&grid);
    let mut added_indices = HashSet::new();

    let mut sum = 0;
    for (i, row) in grid.iter().enumerate() {
        for (j, space) in row.iter().copied().enumerate() {
            if !matches!(space, Space::Symbol(_)) {
                continue;
            }

            let min_row = i.saturating_sub(1);
            let max_row = i + 1;
            let min_col = j.saturating_sub(1);
            let max_col = j + 1;

            for row in numbers.iter().take(max_row + 1).skip(min_row) {
                for IndexedNumber { index, number } in
                    row.iter().copied().take(max_col + 1).skip(min_col)
                {
                    if number != 0 && added_indices.insert(index) {
                        sum += number;
                    }
                }
            }
        }
    }

    sum
}

fn generate_number_grid(grid: &[Vec<Space>]) -> Vec<Vec<IndexedNumber>> {
    let mut numbers = vec![vec![IndexedNumber::default(); grid[0].len()]; grid.len()];
    let mut index = 1;

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

            numbers[i][j..k].fill(IndexedNumber { index, number });
            index += 1;
        }
    }

    numbers
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

fn compute_gear_ratio(numbers: &[Vec<IndexedNumber>], i: usize, j: usize) -> u32 {
    let mut count = 0;
    let mut product = 1;
    let mut added_indices = ArrayVec::<_, 6>::new();

    let min_row = i.saturating_sub(1);
    let max_row = i + 1;
    let min_col = j.saturating_sub(1);
    let max_col = j + 1;

    for row in numbers.iter().take(max_row + 1).skip(min_row) {
        for IndexedNumber { index, number } in row.iter().copied().take(max_col + 1).skip(min_col) {
            if number != 0 && !added_indices.contains(&index) {
                count += 1;
                product *= number;
                added_indices.push(index);
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
