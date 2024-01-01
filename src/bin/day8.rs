//! Day 8: Haunted Wasteland
//!
//! <https://adventofcode.com/2023/day/8>
//!
//! Assumptions made:
//! - For each node name that ends in 'A', following the directions starting at that node will eventually result in a
//!   cycle of length C that begins at step N. Further, the cycle contains exactly one node with a name ending in 'Z'
//!   at step (C - N) within the cycle, which means that the path lands on that node at global steps C, 2C, 3C, etc.
//!   The C and N values are not assumed to be identical between starting nodes.
//!
//! Part 1: Simple directed graph traversal where each node has exactly 2 outgoing edges and the L/R direction specifies
//! which edge to take at each step. Start at "AAA" and follow the input directions in a cycle until you reach "ZZZ",
//! counting how many steps it takes to get there.
//!
//! Part 2: The answer is too large for a naive solution. Given the assumption noted above, search for the cycle in each
//! path by counting how many steps it takes for each path to first reach a node name ending in 'Z'. Given that each
//! path lands on a node ending in 'Z' at steps C, 2C, 3C, etc., the first step where _all_ paths land on a node ending
//! in 'Z' is the the least common multiple of all of the cycle lengths.
//!
//! LCM is associative, so the LCM across all cycle lengths is computed by reducing over the list of cycle lengths and
//! computing pairwise LCMs (using Euclid's algorithm to calculate greatest common divisor).

use advent_of_code_2023::impl_main;
use rustc_hash::FxHashMap;
use winnow::ascii::{alphanumeric1, newline};
use winnow::combinator::{
    delimited, fail, opt, repeat, separated, separated_pair, success, terminated,
};
use winnow::dispatch;
use winnow::prelude::*;
use winnow::token::any;

#[derive(Debug, Clone)]
struct Node<'a> {
    name: &'a str,
    left: &'a str,
    right: &'a str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug, Clone)]
struct Input<'a> {
    directions: Vec<Direction>,
    nodes: Vec<Node<'a>>,
}

fn parse_direction(input: &mut &str) -> PResult<Direction> {
    dispatch! { any;
        'L' => success(Direction::Left),
        'R' => success(Direction::Right),
        _ => fail,
    }
    .parse_next(input)
}

fn parse_node<'a>(input: &mut &'a str) -> PResult<Node<'a>> {
    let (name, (left, right)) = separated_pair(
        alphanumeric1,
        " = ",
        delimited('(', separated_pair(alphanumeric1, ", ", alphanumeric1), ')'),
    )
    .parse_next(input)?;
    Ok(Node { name, left, right })
}

fn parse_input<'a>(input: &mut &'a str) -> PResult<Input<'a>> {
    let directions: Vec<_> = terminated(repeat(1.., parse_direction), newline).parse_next(input)?;

    newline.parse_next(input)?;

    let nodes: Vec<_> = separated(1.., parse_node, newline).parse_next(input)?;

    opt(newline).parse_next(input)?;

    Ok(Input { directions, nodes })
}

fn solve_part_1(input: &str) -> u32 {
    let input = parse_input.parse(input).expect("Invalid input");

    let node_map = nodes_to_map(&input.nodes);

    let mut current = node_map.get("AAA").expect("No AAA node in input");
    let mut steps = 0;
    for &direction in input.directions.iter().cycle() {
        match direction {
            Direction::Left => {
                current = node_map.get(current.left).expect("Invalid left in input");
            }
            Direction::Right => {
                current = node_map.get(current.right).expect("Invalid right in input");
            }
        }

        steps += 1;
        if current.name == "ZZZ" {
            break;
        }
    }

    steps
}

fn nodes_to_map<'a>(nodes: &[Node<'a>]) -> FxHashMap<&'a str, Node<'a>> {
    nodes.iter().map(|node| (node.name, node.clone())).collect()
}

fn solve_part_2(input: &str) -> u64 {
    let input = parse_input.parse(input).expect("Invalid input");

    let node_map = nodes_to_map(&input.nodes);

    let mut current: Vec<_> = input.nodes.iter().filter(|node| node.name.ends_with('A')).collect();
    let mut first_z_step: FxHashMap<&str, u64> = FxHashMap::default();

    let mut steps = 0;
    for &direction in input.directions.iter().cycle() {
        for node in &mut current {
            match direction {
                Direction::Left => {
                    *node = node_map.get(node.left).expect("Invalid left in input");
                }
                Direction::Right => {
                    *node = node_map.get(node.right).expect("Invalid right in input");
                }
            }
        }

        steps += 1;
        if current.iter().all(|node| node.name.ends_with('Z')) {
            return steps;
        }

        for node in &current {
            if node.name.ends_with('Z') && !first_z_step.contains_key(node.name) {
                first_z_step.insert(node.name, steps);
            }
        }

        if first_z_step.len() == current.len() {
            return lcm(first_z_step.values().copied());
        }
    }

    unreachable!("Above loop is iterating over an infinite iterator and never breaks, only returns")
}

fn lcm(nums: impl Iterator<Item = u64>) -> u64 {
    nums.reduce(|a, b| a * b / gcd(a, b)).expect("No cycle lengths in LCM input")
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 { a } else { gcd(b, a % b) }
}

impl_main!(p1: solve_part_1, p2: solve_part_2);

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("../../sample_input/day8.txt");
    const SAMPLE_INPUT_2: &str = include_str!("../../sample_input/day8-2.txt");
    const SAMPLE_INPUT_3: &str = include_str!("../../sample_input/day8-3.txt");

    #[test]
    fn sample_input_part_1() {
        assert_eq!(solve_part_1(SAMPLE_INPUT), 2);
        assert_eq!(solve_part_1(SAMPLE_INPUT_2), 6);
    }

    #[test]
    fn sample_input_part_2() {
        assert_eq!(solve_part_2(SAMPLE_INPUT_3), 6);
    }
}
