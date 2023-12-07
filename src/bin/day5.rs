//! Day 5: If You Give A Seed A Fertilizer
//!
//! <https://adventofcode.com/2023/day/5>

use nom::bytes::complete::tag;
use nom::character::complete::{char, digit1, newline, not_line_ending, space1};
use nom::combinator::map_res;
use nom::multi::separated_list1;
use nom::sequence::{pair, preceded, separated_pair, terminated};
use nom::IResult;
use std::cmp;
use std::error::Error;

#[derive(Debug, Clone)]
struct MapRange {
    dest_start: i64,
    source_start: i64,
    length: i64,
}

#[derive(Debug, Clone)]
struct Input {
    seeds: Vec<i64>,
    maps: Vec<Vec<MapRange>>,
}

fn parse_i64(input: &str) -> IResult<&str, i64> {
    map_res(digit1, str::parse)(input)
}

fn parse_seeds(input: &str) -> IResult<&str, Vec<i64>> {
    preceded(tag("seeds: "), separated_list1(char(' '), parse_i64))(input)
}

fn parse_map_range(input: &str) -> IResult<&str, MapRange> {
    let (input, (dest_start, (source_start, length))) =
        separated_pair(parse_i64, space1, separated_pair(parse_i64, space1, parse_i64))(input)?;

    Ok((input, MapRange { dest_start, source_start, length }))
}

fn parse_map(input: &str) -> IResult<&str, Vec<MapRange>> {
    // Skip header line
    let (input, _) = pair(not_line_ending, newline)(input)?;

    separated_list1(newline, parse_map_range)(input)
}

fn parse_input(input: &str) -> IResult<&str, Input> {
    let (input, seeds) = terminated(parse_seeds, pair(newline, newline))(input)?;

    let (input, maps) = separated_list1(pair(newline, newline), parse_map)(input)?;

    Ok((input, Input { seeds, maps }))
}

fn solve_part_1(input: &str) -> i64 {
    let (_, input) = parse_input(input).expect("Invalid input");

    input
        .seeds
        .iter()
        .copied()
        .map(|seed| find_min_location_part_1(&input, 0, seed))
        .min()
        .expect("No seeds in input")
}

fn find_min_location_part_1(input: &Input, i: usize, value: i64) -> i64 {
    if i == input.maps.len() {
        return value;
    }

    input.maps[i]
        .iter()
        .find_map(|range| {
            (range.source_start..range.source_start + range.length).contains(&value).then(|| {
                find_min_location_part_1(
                    input,
                    i + 1,
                    value + range.dest_start - range.source_start,
                )
            })
        })
        .unwrap_or_else(|| find_min_location_part_1(input, i + 1, value))
}

fn solve_part_2(input: &str) -> i64 {
    let (_, mut input) = parse_input(input).expect("Invalid input");

    for map in &mut input.maps {
        map.sort_by_key(|range| range.source_start);
    }

    input
        .seeds
        .chunks_exact(2)
        .map(|chunk| {
            let &[start, length] = chunk else { unreachable!("chunks_exact(2)") };
            find_min_location_part_2(&input, 0, start, length)
        })
        .min()
        .expect("No seeds in input")
}

fn find_min_location_part_2(input: &Input, i: usize, start: i64, length: i64) -> i64 {
    if i == input.maps.len() {
        return start;
    }

    let mut min = i64::MAX;

    let mut start = start;
    let mut length = length;
    for range in &input.maps[i] {
        if start < range.source_start {
            // Part of this range is before the next range in the map; pass values through directly
            let before_len = cmp::min(range.source_start - start, length);
            min = cmp::min(min, find_min_location_part_2(input, i + 1, start, before_len));

            length -= before_len;
            start += before_len;
        }

        if length == 0 {
            break;
        }

        let range_end = range.source_start + range.length;
        if start < range_end {
            // Part of this range overlaps the next range; map values appropriately
            let end = start + length;
            let overlap_len = cmp::min(end, range_end) - start;
            let overlap_start = range.dest_start + (start - range.source_start);
            min = cmp::min(min, find_min_location_part_2(input, i + 1, overlap_start, overlap_len));

            length -= overlap_len;
            start += overlap_len;
        }

        if length == 0 {
            break;
        }
    }

    if length != 0 {
        // Part of this range is after the last range; pass values through directly
        min = cmp::min(min, find_min_location_part_2(input, i + 1, start, length));
    }

    min
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

    const SAMPLE_INPUT: &str = include_str!("../sample/day5.txt");

    #[test]
    fn sample_input_part_1() {
        assert_eq!(solve_part_1(SAMPLE_INPUT), 35);
    }

    #[test]
    fn sample_input_part_2() {
        assert_eq!(solve_part_2(SAMPLE_INPUT), 46);
    }
}
