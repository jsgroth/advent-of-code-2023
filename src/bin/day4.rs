//! Day 4: Scratchcards
//!
//! <https://adventofcode.com/2023/day/4>

use nom::bytes::complete::tag;
use nom::character::complete::{char, digit1, space1};
use nom::combinator::map_res;
use nom::multi::separated_list1;
use nom::sequence::{delimited, pair, separated_pair};
use nom::IResult;
use std::cmp;
use std::collections::HashSet;
use std::error::Error;

fn parse_u32(input: &str) -> IResult<&str, u32> {
    map_res(digit1, str::parse)(input)
}

fn parse_numbers(input: &str) -> IResult<&str, Vec<u32>> {
    separated_list1(space1, parse_u32)(input)
}

fn parse_line(input: &str) -> IResult<&str, (Vec<u32>, Vec<u32>)> {
    let (input, _) = delimited(pair(tag("Card"), space1), digit1, pair(char(':'), space1))(input)?;

    let (input, (winning_numbers, your_numbers)) =
        separated_pair(parse_numbers, delimited(space1, char('|'), space1), parse_numbers)(input)?;
    Ok((input, (winning_numbers, your_numbers)))
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
    let (_, (winning_numbers, your_numbers)) = parse_line(line).expect("Invalid line");

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
