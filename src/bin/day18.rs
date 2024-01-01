//! Day 18: Lavaduct Lagoon
//!
//! <https://adventofcode.com/2023/day/18>

use advent_of_code_2023::impl_main;
use std::cmp;
use winnow::ascii::{digit1, newline, space1};
use winnow::combinator::{delimited, fail, opt, separated, success};
use winnow::dispatch;
use winnow::prelude::*;
use winnow::token::{any, take_while};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    fn di_dj(self) -> (i64, i64) {
        match self {
            Self::Up => (-1, 0),
            Self::Down => (1, 0),
            Self::Left => (0, -1),
            Self::Right => (0, 1),
        }
    }
}

#[derive(Debug, Clone)]
struct InputLine {
    direction: Direction,
    distance: i64,
    hex_direction: Direction,
    hex_distance: i64,
}

impl InputLine {
    fn direction_and_distance(&self, direction_type: DirectionType) -> (Direction, i64) {
        match direction_type {
            DirectionType::Normal => (self.direction, self.distance),
            DirectionType::Hex => (self.hex_direction, self.hex_distance),
        }
    }
}

fn parse_direction(input: &mut &str) -> PResult<Direction> {
    dispatch! { any;
        'U' => success(Direction::Up),
        'L' => success(Direction::Left),
        'R' => success(Direction::Right),
        'D' => success(Direction::Down),
        _ => fail,
    }
    .parse_next(input)
}

fn parse_i64(input: &mut &str) -> PResult<i64> {
    digit1.parse_to().parse_next(input)
}

fn parse_hex_inner(input: &mut &str) -> PResult<(i64, Direction)> {
    '#'.parse_next(input)?;

    let distance = take_while(5, |c: char| c.is_ascii_hexdigit())
        .try_map(|input| i64::from_str_radix(input, 16))
        .parse_next(input)?;

    let direction = dispatch! { any;
        '0' => success(Direction::Right),
        '1' => success(Direction::Down),
        '2' => success(Direction::Left),
        '3' => success(Direction::Up),
        _ => fail,
    }
    .parse_next(input)?;

    Ok((distance, direction))
}

fn parse_hex(input: &mut &str) -> PResult<(i64, Direction)> {
    delimited('(', parse_hex_inner, ')').parse_next(input)
}

fn parse_line(input: &mut &str) -> PResult<InputLine> {
    let (direction, _, distance, _, (hex_distance, hex_direction)) =
        (parse_direction, space1, parse_i64, space1, parse_hex).parse_next(input)?;
    Ok(InputLine { direction, distance, hex_distance, hex_direction })
}

fn parse_input(input: &mut &str) -> PResult<Vec<InputLine>> {
    let lines = separated(1.., parse_line, newline).parse_next(input)?;
    opt(newline).parse_next(input)?;
    Ok(lines)
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct VerticalLine {
    min_i: i64,
    max_i: i64,
    j: i64,
    direction: Direction,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DirectionType {
    Normal,
    Hex,
}

fn solve(input: &str, direction_type: DirectionType) -> i64 {
    let input = parse_input.parse(input).expect("Invalid input");

    let mut lines = convert_to_vertical_lines(&input, direction_type);
    lines.sort_by(|a, b| a.j.cmp(&b.j).then(a.min_i.cmp(&b.min_i)));

    let mut min_i = i64::MAX;
    let mut max_i = i64::MIN;
    let mut min_j = i64::MAX;
    for line in &lines {
        min_i = cmp::min(min_i, line.min_i);
        max_i = cmp::max(max_i, line.max_i);
        min_j = cmp::min(min_j, line.j);
    }

    process_range(&lines, min_i, max_i, min_j, false)
}

fn process_range(lines: &[VerticalLine], min_i: i64, max_i: i64, j: i64, inside: bool) -> i64 {
    let Some((line_idx, line)) = find_next_line(lines, min_i, max_i, j) else {
        assert!(!inside, "No line found after min_i={min_i} max_i={max_i} j={j}");
        return 0;
    };

    let mut count = 0;

    // Above line
    if min_i < line.min_i {
        if inside {
            count += (line.j - j) * (line.min_i - min_i);
        }

        count += process_range(&lines[line_idx + 1..], min_i, line.min_i - 1, line.j, inside);
    }

    // Topmost point of line
    if min_i <= line.min_i {
        if inside {
            count += line.j - j;
        }

        count += process_horizontal_line(&lines[line_idx..], line.min_i, line.j, inside);
    }

    // Crossing line
    let cross_min_i = cmp::max(min_i, line.min_i + 1);
    let cross_max_i = cmp::min(max_i, line.max_i - 1);
    if cross_min_i > line.min_i
        && cross_min_i >= min_i
        && cross_max_i < line.max_i
        && cross_max_i <= max_i
        && cross_max_i >= cross_min_i
    {
        count += cross_max_i - cross_min_i + 1;

        if inside {
            count += (line.j - j) * (cross_max_i - cross_min_i + 1);
        }

        count +=
            process_range(&lines[line_idx + 1..], cross_min_i, cross_max_i, line.j + 1, !inside);
    }

    // Bottommost point of line
    if max_i >= line.max_i {
        if inside {
            count += line.j - j;
        }

        count += process_horizontal_line(&lines[line_idx..], line.max_i, line.j, inside);
    }

    // Below line
    if max_i > line.max_i {
        if inside {
            count += (line.j - j) * (max_i - line.max_i);
        }

        count += process_range(&lines[line_idx + 1..], line.max_i + 1, max_i, line.j, inside);
    }

    count
}

fn process_horizontal_line(lines: &[VerticalLine], i: i64, j: i64, inside: bool) -> i64 {
    let current_line = &lines[0];
    let Some((next_line_idx, next_line)) = find_next_line(&lines[1..], i, i, j + 1) else {
        assert!(!inside, "Invalid input; no line found after i={i} j={j}");
        return 0;
    };

    let new_inside = if current_line.direction == next_line.direction { !inside } else { inside };

    let mut line_count = next_line.j - j + 1;
    line_count += process_range(&lines[next_line_idx + 1..], i, i, next_line.j + 1, new_inside);

    line_count
}

fn find_next_line(
    lines: &[VerticalLine],
    min_i: i64,
    max_i: i64,
    j: i64,
) -> Option<(usize, &VerticalLine)> {
    lines.iter().enumerate().find(|(_, line)| {
        line.j >= j
            && ((line.min_i >= min_i && line.max_i <= max_i)
                || (line.min_i <= max_i && line.max_i >= max_i)
                || (line.min_i <= min_i && line.max_i >= min_i))
    })
}

fn convert_to_vertical_lines(
    input: &[InputLine],
    direction_type: DirectionType,
) -> Vec<VerticalLine> {
    let mut i = 0;
    let mut j = 0;
    let mut lines = Vec::new();
    for input_line in input {
        let (direction, distance) = input_line.direction_and_distance(direction_type);

        let (di, dj) = direction.di_dj();
        let new_i = i + di * distance;
        let new_j = j + dj * distance;
        if j == new_j {
            let min_i = cmp::min(i, new_i);
            let max_i = cmp::max(i, new_i);
            lines.push(VerticalLine { min_i, max_i, j, direction });
        }

        i = new_i;
        j = new_j;
    }

    lines
}

fn solve_part_1(input: &str) -> i64 {
    solve(input, DirectionType::Normal)
}

fn solve_part_2(input: &str) -> i64 {
    solve(input, DirectionType::Hex)
}

impl_main!(p1: solve_part_1, p2: solve_part_2);

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("../../sample_input/day18.txt");

    #[test]
    fn sample_input_part_1() {
        assert_eq!(solve_part_1(SAMPLE_INPUT), 62);
    }

    #[test]
    fn sample_input_part_2() {
        assert_eq!(solve_part_2(SAMPLE_INPUT), 952408144115);
    }
}
