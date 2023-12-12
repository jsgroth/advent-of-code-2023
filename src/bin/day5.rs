//! Day 5: If You Give A Seed A Fertilizer
//!
//! <https://adventofcode.com/2023/day/5>

use advent_of_code_2023::impl_main;
use std::cmp;
use winnow::ascii::{digit1, newline, not_line_ending, space1};
use winnow::combinator::{opt, preceded, separated, terminated};
use winnow::prelude::*;

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

fn parse_i64(input: &mut &str) -> PResult<i64> {
    digit1.parse_to().parse_next(input)
}

fn parse_seeds(input: &mut &str) -> PResult<Vec<i64>> {
    preceded("seeds: ", separated(1.., parse_i64, ' ')).parse_next(input)
}

fn parse_map_range(input: &mut &str) -> PResult<MapRange> {
    let (dest_start, _, source_start, _, length) =
        (parse_i64, space1, parse_i64, space1, parse_i64).parse_next(input)?;
    Ok(MapRange { dest_start, source_start, length })
}

fn parse_map(input: &mut &str) -> PResult<Vec<MapRange>> {
    // Skip header line
    (not_line_ending, newline).parse_next(input)?;

    separated(1.., parse_map_range, newline).parse_next(input)
}

fn parse_input(input: &mut &str) -> PResult<Input> {
    let seeds = terminated(parse_seeds, (newline, newline)).parse_next(input)?;

    let maps = separated(1.., parse_map, (newline, newline)).parse_next(input)?;

    opt(newline).parse_next(input)?;

    Ok(Input { seeds, maps })
}

fn solve_part_1(input: &str) -> i64 {
    let input = parse_input
        .parse(input)
        .map_err(|err| {
            println!("{}", input.len());
            err
        })
        .expect("Invalid input");

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
    let mut input = parse_input.parse(input).expect("Invalid input");

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

impl_main!(p1: solve_part_1, p2: solve_part_2);

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
