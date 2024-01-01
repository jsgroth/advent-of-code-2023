//! Day 15: Lens Library
//!
//! <https://adventofcode.com/2023/day/15>
//!
//! Part 1: Simply apply the given hash algorithm to each string in the comma-separated input and sum the results.
//!
//! Part 2: As the problem description not-so-subtly implies, this is essentially just implementing a hash map with a
//! fixed array size (256) and using chaining for hash collisions. Each xx=N command is insert(xx, N), and each
//! xx=- command is remove(xx).

use advent_of_code_2023::impl_main;
use winnow::ascii::{alpha1, digit1, newline};
use winnow::combinator::{alt, opt, preceded, separated};

use winnow::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Command {
    Remove,
    Insert(u32),
}

fn parse_remove(input: &mut &str) -> PResult<Command> {
    '-'.parse_next(input)?;
    Ok(Command::Remove)
}

fn parse_u32(input: &mut &str) -> PResult<u32> {
    digit1.parse_to().parse_next(input)
}

fn parse_insert(input: &mut &str) -> PResult<Command> {
    let length = preceded('=', parse_u32).parse_next(input)?;
    Ok(Command::Insert(length))
}

fn parse_command(input: &mut &str) -> PResult<Command> {
    alt((parse_remove, parse_insert)).parse_next(input)
}

fn parse_input<'a>(input: &mut &'a str) -> PResult<Vec<(&'a str, Command)>> {
    let commands = separated(1.., (alpha1, parse_command), ',').parse_next(input)?;

    opt(newline).parse_next(input)?;

    Ok(commands)
}

fn solve_part_1(input: &str) -> u32 {
    input.lines().next().expect("No lines in input").split(',').map(hash).sum()
}

fn solve_part_2(input: &str) -> u32 {
    let commands = parse_input.parse(input).expect("Invalid input");

    let mut buckets: Vec<Vec<(&str, u32)>> = vec![vec![]; 256];

    for (cmd_label, command) in commands {
        let bucket_idx = hash(cmd_label);
        let bucket = &mut buckets[bucket_idx as usize];

        match command {
            Command::Remove => {
                bucket.retain(|(label, _)| *label != cmd_label);
            }
            Command::Insert(length) => {
                let mut found = false;
                for (label, value) in bucket.iter_mut() {
                    if *label == cmd_label {
                        found = true;
                        *value = length;
                        break;
                    }
                }

                if !found {
                    bucket.push((cmd_label, length));
                }
            }
        }
    }

    focusing_power(&buckets)
}

fn focusing_power(buckets: &[Vec<(&str, u32)>]) -> u32 {
    buckets
        .iter()
        .enumerate()
        .map(|(bucket_idx, bucket)| {
            bucket
                .iter()
                .enumerate()
                .map(|(slot, &(_, length))| (bucket_idx as u32 + 1) * (slot as u32 + 1) * length)
                .sum::<u32>()
        })
        .sum()
}

fn hash(s: &str) -> u32 {
    s.chars().fold(0, |hash, c| ((hash + c as u32) * 17) % 256)
}

impl_main!(p1: solve_part_1, p2: solve_part_2);

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("../../sample_input/day15.txt");

    #[test]
    fn sample_input_part_1() {
        assert_eq!(solve_part_1(SAMPLE_INPUT), 1320);
    }

    #[test]
    fn sample_input_part_2() {
        assert_eq!(solve_part_2(SAMPLE_INPUT), 145);
    }
}
