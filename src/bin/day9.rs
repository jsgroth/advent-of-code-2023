//! Day 9: Mirage Maintenance
//!
//! <https://adventofcode.com/2023/day/9>

use advent_of_code_2023::impl_standard_main;

fn parse_line(line: &str) -> Vec<i64> {
    line.split(' ').map(|s| s.parse::<i64>().expect("Invalid line")).collect()
}

fn fold_differences<F>(numbers: &[i64], f: F) -> i64
where
    F: Copy + Fn(&[i64], i64) -> i64,
{
    if numbers.iter().all(|&n| n == 0) {
        return 0;
    }

    let differences: Vec<_> = numbers.windows(2).map(|window| window[1] - window[0]).collect();
    let next = fold_differences(&differences, f);
    f(numbers, next)
}

fn solve_part_1(input: &str) -> i64 {
    input
        .lines()
        .map(|line| {
            fold_differences(&parse_line(line), |numbers, diff| *numbers.last().unwrap() + diff)
        })
        .sum()
}

fn solve_part_2(input: &str) -> i64 {
    input
        .lines()
        .map(|line| fold_differences(&parse_line(line), |numbers, diff| numbers[0] - diff))
        .sum()
}

impl_standard_main!(p1: solve_part_1, p2: solve_part_2);

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("../sample/day9.txt");

    #[test]
    fn sample_input_part_1() {
        assert_eq!(solve_part_1(SAMPLE_INPUT), 114);
    }

    #[test]
    fn sample_input_part_2() {
        assert_eq!(solve_part_2(SAMPLE_INPUT), 2);
    }
}
