//! Day 4: Scratchcards
//!
//! <https://adventofcode.com/2023/day/4>

use advent_of_code_2023::impl_main;
use std::cmp;
use std::collections::HashSet;
use winnow::ascii::{digit1, space1};
use winnow::combinator::{separated, separated_pair};
use winnow::prelude::*;

fn parse_u32(input: &mut &str) -> PResult<u32> {
    digit1.parse_to().parse_next(input)
}

fn parse_numbers(input: &mut &str) -> PResult<Vec<u32>> {
    separated(1.., parse_u32, space1).parse_next(input)
}

fn parse_line(input: &mut &str) -> PResult<(Vec<u32>, Vec<u32>)> {
    ("Card", space1, digit1, ':', space1).parse_next(input)?;

    separated_pair(parse_numbers, (space1, '|', space1), parse_numbers).parse_next(input)
}

fn solve_part_1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let win_count = count_winning_numbers(line);
            if win_count != 0 { 2_u32.pow(win_count - 1) } else { 0 }
        })
        .sum()
}

fn count_winning_numbers(line: &str) -> u32 {
    let (winning_numbers, your_numbers) = parse_line.parse(line).expect("Invalid line");

    let winning_numbers: HashSet<_> = winning_numbers.into_iter().collect();
    your_numbers.into_iter().filter(|number| winning_numbers.contains(number)).count() as u32
}

fn solve_part_2(input: &str) -> u32 {
    let num_cards = input.lines().count();

    let mut card_counts = vec![1; num_cards];

    for (i, line) in input.lines().enumerate() {
        let win_count = count_winning_numbers(line);

        let end = cmp::min(i + win_count as usize + 1, card_counts.len());
        for j in i + 1..end {
            card_counts[j] += card_counts[i];
        }
    }

    card_counts.into_iter().sum()
}

impl_main!(p1: solve_part_1, p2: solve_part_2);

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("../sample/day4.txt");

    #[test]
    fn sample_input_part_1() {
        assert_eq!(solve_part_1(SAMPLE_INPUT), 13);
    }

    #[test]
    fn sample_input_part_2() {
        assert_eq!(solve_part_2(SAMPLE_INPUT), 30);
    }
}
