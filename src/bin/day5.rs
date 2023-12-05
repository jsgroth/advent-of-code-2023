//! Day 5: If You Give A Seed A Fertilizer
//!
//! <https://adventofcode.com/2023/day/5>

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

fn parse_input(input: &str) -> Input {
    let mut lines = input.lines();
    let seeds_str = lines.next().expect("Invalid input: no lines");
    let (_, seed_numbers) = seeds_str.split_once(": ").expect("Invalid input: seeds line");
    let seeds: Vec<_> = seed_numbers
        .split_whitespace()
        .map(|s| s.parse::<i64>().expect("Invalid input: seed numbers"))
        .collect();

    lines.next();

    let mut maps = Vec::new();
    while let Some(_header) = lines.next() {
        let mut ranges = Vec::new();
        for map_line in lines.by_ref().take_while(|line| !line.is_empty()) {
            let numbers: Vec<_> = map_line
                .split_whitespace()
                .map(|s| s.parse::<i64>().expect("Invalid input: map line"))
                .collect();
            ranges.push(MapRange {
                dest_start: numbers[0],
                source_start: numbers[1],
                length: numbers[2],
            });
        }

        maps.push(ranges);
    }

    Input { seeds, maps }
}

fn solve_part_1(input: &str) -> i64 {
    let input = parse_input(input);

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
        .filter_map(|range| {
            (range.source_start..range.source_start + range.length).contains(&value).then(|| {
                find_min_location_part_1(
                    input,
                    i + 1,
                    value + range.dest_start - range.source_start,
                )
            })
        })
        .min()
        .unwrap_or_else(|| {
            // Seed did not match any ranges; value maps to the next fertilizer directly
            find_min_location_part_1(input, i + 1, value)
        })
}

fn solve_part_2(input: &str) -> i64 {
    let mut input = parse_input(input);

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
