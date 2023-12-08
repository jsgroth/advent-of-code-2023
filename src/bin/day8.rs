//! Day 8: Haunted Wasteland
//!
//! <https://adventofcode.com/2023/day/8>

use std::collections::HashMap;
use std::error::Error;
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

fn nodes_to_map<'a>(nodes: &[Node<'a>]) -> HashMap<&'a str, Node<'a>> {
    nodes.iter().map(|node| (node.name, node.clone())).collect()
}

fn solve_part_2(input: &str) -> u64 {
    let input = parse_input.parse(input).expect("Invalid input");

    let node_map = nodes_to_map(&input.nodes);

    let mut current: Vec<_> = input.nodes.iter().filter(|node| node.name.ends_with('A')).collect();
    let mut visited_to_step: Vec<HashMap<(u32, &str), u64>> = vec![HashMap::new(); current.len()];
    let mut cycle_len: Vec<Option<u64>> = vec![None; current.len()];

    for (node, visited_map) in current.iter().zip(&mut visited_to_step) {
        visited_map.insert((0, node.name), 0);
    }

    let mut steps = 0;
    for (direction_idx, &direction) in input.directions.iter().enumerate().cycle() {
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

        for (node_idx, node) in current.iter().enumerate() {
            if cycle_len[node_idx].is_none() {
                if let Some(&prev_steps) =
                    visited_to_step[node_idx].get(&(direction_idx as u32, node.name))
                {
                    cycle_len[node_idx] = Some(steps - prev_steps);
                } else {
                    visited_to_step[node_idx].insert((direction_idx as u32, node.name), steps);
                }
            }
        }

        // This takes advantage of a property of the (actual) input.
        // For each node i that ends in 'A', following the directions will eventually result in a cycle of length C[i]
        // that begins after N[i] steps.
        // It just so happens that for each of these cycles, the cycle lands on a node that ends in 'Z' at
        // step C[i] - N[i], which causes the math to work out such that the minimum step where every node ends in
        // 'Z' is the least common multiple of all of the cycle lengths C[i].
        if cycle_len.iter().all(Option::is_some) {
            let nums: Vec<_> = cycle_len.iter().copied().map(Option::unwrap).collect();
            return lcm(&nums);
        }
    }

    unreachable!("Above loop is iterating over an infinite iterator and never breaks, only returns")
}

fn lcm(nums: &[u64]) -> u64 {
    nums.iter().copied().reduce(|a, b| a * b / gcd(a, b)).expect("No cycle lengths in LCM input")
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 { a } else { gcd(b, a % b) }
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

    const SAMPLE_INPUT: &str = include_str!("../sample/day8.txt");
    const SAMPLE_INPUT_2: &str = include_str!("../sample/day8-2.txt");
    const SAMPLE_INPUT_3: &str = include_str!("../sample/day8-3.txt");

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
