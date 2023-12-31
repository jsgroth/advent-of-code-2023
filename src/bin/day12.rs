//! Day 12: Hot Springs
//!
//! <https://adventofcode.com/2023/day/12>
//!
//! Part 1: Uses dynamic programming (not necessary for part 1 but is necessary for part 2). The cache key is the pair
//! of (# springs remaining, # damage groups remaining).
//!
//! Each step processes the first damage group. If there are no damage groups remaining, there is exactly 1 solution
//! if there are no damaged springs remaining, and 0 solutions if there are damaged springs remaining.
//!
//! Otherwise, the algorithm determines all possible positions where the first damage group can be placed without
//! skipping any damaged springs and while leaving enough room for the remaining damage groups, and it recursively
//! computes the number of solutions for each of those positions and sums them.
//!
//! Part 2: Uses the same algorithm as part 1, simply pre-processing the input to apply the transformation specified
//! in the problem description (which massively expands the search space to the point that a brute force solution won't
//! work).

use advent_of_code_2023::impl_main;
use rustc_hash::FxHashMap;
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
            count_unique_arrangements(&record.springs, &record.damage_groups)
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

            count_unique_arrangements(&new_springs, &new_damage_groups)
        })
        .sum()
}

fn count_unique_arrangements(springs: &[Spring], damage_groups: &[u32]) -> u64 {
    let remaining_required = initial_remaining_required(damage_groups);
    assert!(
        remaining_required <= springs.len(),
        "Damage groups require {remaining_required} springs, cannot possibly fit in input of len {}",
        springs.len()
    );

    count_inner(springs, damage_groups, remaining_required, &mut FxHashMap::default())
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

fn count_inner(
    springs: &[Spring],
    damage_groups: &[u32],
    remaining_required: usize,
    cache: &mut FxHashMap<CacheKey, u64>,
) -> u64 {
    if remaining_required == 0 {
        let damage_remaining = springs.iter().any(|&status| status == Spring::Damaged);
        return if damage_remaining { 0 } else { 1 };
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
                count += count_inner(
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

impl_main!(p1: solve_part_1, p2: solve_part_2);

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("../../sample_input/day12.txt");

    #[test]
    fn sample_input_part_1() {
        assert_eq!(solve_part_1(SAMPLE_INPUT), 21);
    }

    #[test]
    fn sample_input_part_2() {
        assert_eq!(solve_part_2(SAMPLE_INPUT), 525152);
    }
}
