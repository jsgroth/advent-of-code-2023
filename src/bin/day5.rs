//! Day 5: If You Give A Seed A Fertilizer
//!
//! <https://adventofcode.com/2023/day/5>
//!
//! Assumptions made:
//! - The graph of maps is a straight line from "seed" to "location", and the maps are in order in the input
//!
//! Part 1: For each seed value in the input, follow the maps to determine the location value of that seed, and take
//! the min location value across all seeds.
//!
//! Part 2: Process the seeds as ranges to avoid the solution taking an extremely long time.
//!
//! First, sort each map by starting value so that it's possible to iterate over the map ranges in order.
//!
//! Then, for a given seed range and map, do the following for each map range, while tracking the minimum location value
//! and while adjusting seed_start after each iteration:
//! - If part of the seed range is before the next map range, values in [seed_start, map_start) go to the next
//!   fertilizer type without transformation
//! - If part of the seed range overlaps the next map range, values in [max(seed_start, map_start), min(seed_end, map_end))
//!   are transformed according to the map rule
//! At the end, if part of the seed range is after the last map range, values in [seed_start, seed_end) go to the next
//! fertilizer type without transformation

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
        .map(|seed| find_seed_location(&input, 0, seed))
        .min()
        .expect("No seeds in input")
}

fn find_seed_location(input: &Input, i: usize, value: i64) -> i64 {
    if i == input.maps.len() {
        return value;
    }

    input.maps[i]
        .iter()
        .find_map(|range| {
            (range.source_start..range.source_start + range.length).contains(&value).then(|| {
                find_seed_location(input, i + 1, value + range.dest_start - range.source_start)
            })
        })
        .unwrap_or_else(|| find_seed_location(input, i + 1, value))
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
            find_min_location(&input, 0, start, length)
        })
        .min()
        .expect("No seeds in input")
}

fn find_min_location(input: &Input, i: usize, start: i64, length: i64) -> i64 {
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
            min = cmp::min(min, find_min_location(input, i + 1, start, before_len));

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
            min = cmp::min(min, find_min_location(input, i + 1, overlap_start, overlap_len));

            length -= overlap_len;
            start += overlap_len;
        }

        if length == 0 {
            break;
        }
    }

    if length != 0 {
        // Part of this range is after the last range; pass values through directly
        min = cmp::min(min, find_min_location(input, i + 1, start, length));
    }

    min
}

impl_main!(p1: solve_part_1, p2: solve_part_2);

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("../../sample_input/day5.txt");

    #[test]
    fn sample_input_part_1() {
        assert_eq!(solve_part_1(SAMPLE_INPUT), 35);
    }

    #[test]
    fn sample_input_part_2() {
        assert_eq!(solve_part_2(SAMPLE_INPUT), 46);
    }
}
