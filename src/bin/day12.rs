//! Day 12: Hot Springs
//!
//! <https://adventofcode.com/2023/day/12>

use advent_of_code_2023::impl_standard_main;
use std::collections::HashMap;
use winnow::ascii::digit1;
use winnow::combinator::{fail, repeat, separated, separated_pair, success};
use winnow::dispatch;
use winnow::prelude::*;
use winnow::token::any;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Spring {
    Operational,
    Damaged,
    Unknown,
}

#[derive(Debug, Clone)]
struct Record {
    springs: Vec<Spring>,
    damage_groups: Vec<u32>,
}

fn parse_spring(input: &mut &str) -> PResult<Spring> {
    dispatch! { any;
        '.' => success(Spring::Operational),
        '#' => success(Spring::Damaged),
        '?' => success(Spring::Unknown),
        _ => fail,
    }
    .parse_next(input)
}

fn parse_springs(input: &mut &str) -> PResult<Vec<Spring>> {
    repeat(1.., parse_spring).parse_next(input)
}

fn parse_u32(input: &mut &str) -> PResult<u32> {
    digit1.parse_to().parse_next(input)
}

fn parse_damage_groups(input: &mut &str) -> PResult<Vec<u32>> {
    separated(1.., parse_u32, ',').parse_next(input)
}

fn parse_line(input: &mut &str) -> PResult<Record> {
    let (springs, damage_groups) =
        separated_pair(parse_springs, ' ', parse_damage_groups).parse_next(input)?;
    Ok(Record { springs, damage_groups })
}

fn solve_part_1(input: &str) -> u64 {
    input
        .lines()
        .map(|line| {
            let record = parse_line.parse(line).expect("Invalid line");

            count_unique_arrangements(
                &record.springs,
                &record.damage_groups,
                initial_remaining_required(&record.damage_groups),
                &mut HashMap::new(),
            )
        })
        .sum()
}

fn solve_part_2(input: &str) -> u64 {
    input
        .lines()
        .map(|line| {
            let record = parse_line.parse(line).expect("Invalid input");

            let mut new_springs = Vec::new();
            for i in 0..5 {
                new_springs.extend(&record.springs);
                if i != 4 {
                    new_springs.push(Spring::Unknown);
                }
            }

            let new_damage_groups = record.damage_groups.repeat(5);

            count_unique_arrangements(
                &new_springs,
                &new_damage_groups,
                initial_remaining_required(&new_damage_groups),
                &mut HashMap::new(),
            )
        })
        .sum()
}

fn initial_remaining_required(groups: &[u32]) -> usize {
    groups.iter().copied().sum::<u32>() as usize + groups.len() - 1
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct CacheKey {
    springs_len: u32,
    damage_groups_len: u32,
}

impl CacheKey {
    fn new(springs: &[Spring], damage_groups: &[u32]) -> Self {
        Self { springs_len: springs.len() as u32, damage_groups_len: damage_groups.len() as u32 }
    }
}

fn count_unique_arrangements(
    springs: &[Spring],
    damage_groups: &[u32],
    remaining_required: usize,
    cache: &mut HashMap<CacheKey, u64>,
) -> u64 {
    if remaining_required == 0 {
        let damage_remaining = springs.iter().any(|&status| status == Spring::Damaged);
        return if damage_remaining { 0 } else { 1 };
    }

    if springs.len() < remaining_required {
        return 0;
    }

    let cache_key = CacheKey::new(springs, damage_groups);
    if let Some(&count) = cache.get(&cache_key) {
        return count;
    }

    let mut count = 0;
    for i in 0..=springs.len() - remaining_required {
        if damage_group_fits(springs, i, damage_groups[0]) {
            if damage_groups.len() == 1 && i == springs.len() - remaining_required {
                count += 1;
            } else {
                count += count_unique_arrangements(
                    &springs[i + damage_groups[0] as usize + 1..],
                    &damage_groups[1..],
                    remaining_required.saturating_sub(damage_groups[0] as usize + 1),
                    cache,
                );
            }
        }

        if springs[i] == Spring::Damaged {
            break;
        }
    }

    cache.insert(cache_key, count);

    count
}

fn damage_group_fits(springs: &[Spring], i: usize, group: u32) -> bool {
    let group = group as usize;

    let no_damage_after = i + group == springs.len() || springs[i + group] != Spring::Damaged;
    no_damage_after && springs[i..i + group].iter().all(|&status| status != Spring::Operational)
}

impl_standard_main!(p1: solve_part_1, p2: solve_part_2);

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("../sample/day12.txt");

    #[test]
    fn sample_input_part_1() {
        assert_eq!(solve_part_1(SAMPLE_INPUT), 21);
    }

    #[test]
    fn sample_input_part_2() {
        assert_eq!(solve_part_2(SAMPLE_INPUT), 525152);
    }
}
