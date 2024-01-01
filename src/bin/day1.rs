//! Day 1: Trebuchet?!
//!
//! <https://adventofcode.com/2023/day/1>

use advent_of_code_2023::impl_main;

fn solve_part_1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let first = first_digit_part_1(line.chars());
            let last = first_digit_part_1(line.chars().rev());
            10 * first + last
        })
        .sum()
}

fn first_digit_part_1(mut iter: impl Iterator<Item = char>) -> u32 {
    iter.find_map(|c| c.to_digit(10)).expect("No digits in line")
}

const WORDS: [(&[u8], u32); 9] = [
    ("one".as_bytes(), 1),
    ("two".as_bytes(), 2),
    ("three".as_bytes(), 3),
    ("four".as_bytes(), 4),
    ("five".as_bytes(), 5),
    ("six".as_bytes(), 6),
    ("seven".as_bytes(), 7),
    ("eight".as_bytes(), 8),
    ("nine".as_bytes(), 9),
];

fn solve_part_2(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let first = first_digit_part_2(line, 0..line.len());
            let last = first_digit_part_2(line, (0..line.len()).rev());
            10 * first + last
        })
        .sum()
}

fn first_digit_part_2(line: &str, indices: impl Iterator<Item = usize>) -> u32 {
    let bytes = line.as_bytes();

    for i in indices {
        if bytes[i].is_ascii_digit() {
            return (bytes[i] - b'0').into();
        }

        if let Some(digit) = check_word(bytes, i) {
            return digit;
        }
    }

    panic!("No digits found in line: {line}");
}

fn check_word(bytes: &[u8], i: usize) -> Option<u32> {
    for (word, digit) in WORDS {
        if i + word.len() <= bytes.len() && &bytes[i..i + word.len()] == word {
            return Some(digit);
        }
    }

    None
}

impl_main!(p1: solve_part_1, p2: solve_part_2);

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("../../sample_input/day1.txt");
    const SAMPLE_INPUT_2: &str = include_str!("../../sample_input/day1-2.txt");

    #[test]
    fn sample_input_part_1() {
        assert_eq!(solve_part_1(SAMPLE_INPUT), 142);
    }

    #[test]
    fn sample_input_part_2() {
        assert_eq!(solve_part_2(SAMPLE_INPUT_2), 281);
    }
}
