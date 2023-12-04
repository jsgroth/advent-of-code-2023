//! Day 4: Scratchcards
//!
//! <https://adventofcode.com/2023/day/4>

use std::cmp;
use std::error::Error;

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
    let (_, numbers_str) = line.split_once(": ").expect("Invalid line");
    let (winning_numbers, your_numbers) = numbers_str.split_once(" | ").expect("Invalid line");

    let winning_numbers = parse_numbers(winning_numbers);
    let your_numbers = parse_numbers(your_numbers);

    your_numbers.into_iter().filter(|number| winning_numbers.contains(number)).count() as u32
}

fn parse_numbers(numbers: &str) -> Vec<u32> {
    numbers.split(' ').filter_map(|s| s.parse::<u32>().ok()).collect()
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
