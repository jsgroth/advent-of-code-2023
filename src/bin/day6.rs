//! Day 6: Wait For It
//!
//! <https://adventofcode.com/2023/day/6>

use std::error::Error;

fn parse_line_part_1(line: &str) -> Vec<u64> {
    line.split_whitespace().skip(1).map(|s| s.parse::<u64>().expect("Invalid line")).collect()
}

fn solve_part_1(input: &str) -> u64 {
    let mut lines = input.lines();
    let times = parse_line_part_1(lines.next().expect("No times line"));
    let distances = parse_line_part_1(lines.next().expect("No distances line"));

    times
        .iter()
        .zip(&distances)
        .map(|(&time, &target_distance)| find_distance_diff(time, target_distance))
        .product()
}

fn find_distance_diff(time: u64, target_distance: u64) -> u64 {
    // Quadratic formula: x = (-b +/- sqrt(b^2 - 4ac)) / 2a
    // Solve (t - x) * x = d, or -x^2 + tx - d = 0
    let a = -1.0;
    let b = time as f64;
    let c = -(target_distance as f64);

    // Min is + and max is - because a is always negative
    let min = (-b + (b * b - 4.0 * a * c).sqrt()) / 2.0 * a;
    let max = (-b - (b * b - 4.0 * a * c).sqrt()) / 2.0 * a;

    (max.ceil() as u64 - 1) - (min.floor() as u64 + 1) + 1
}

fn parse_line_part_2(line: &str) -> u64 {
    let s: String = line.chars().filter(char::is_ascii_digit).collect();
    s.parse().expect("Invalid input line")
}

fn solve_part_2(input: &str) -> u64 {
    let mut lines = input.lines();
    let time = parse_line_part_2(lines.next().expect("No time line"));
    let target_distance = parse_line_part_2(lines.next().expect("No distance line"));

    find_distance_diff(time, target_distance)
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

    const SAMPLE_INPUT: &str = include_str!("../sample/day6.txt");

    #[test]
    fn sample_input_part_1() {
        assert_eq!(solve_part_1(SAMPLE_INPUT), 288);
    }

    #[test]
    fn sample_input_part_2() {
        assert_eq!(solve_part_2(SAMPLE_INPUT), 71503);
    }
}