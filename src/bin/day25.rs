//! Day 25: Snowverload
//!
//! <https://adventofcode.com/2023/day/25>

use rustc_hash::{FxHashMap, FxHashSet};
use std::collections::VecDeque;
use std::error::Error;
use winnow::ascii::{alpha1, newline};
use winnow::combinator::{opt, separated, separated_pair};

use winnow::prelude::*;

#[derive(Debug, Clone)]
struct InputLine<'a> {
    name: &'a str,
    edges: Vec<&'a str>,
}

fn parse_edges<'a>(input: &mut &'a str) -> PResult<Vec<&'a str>> {
    separated(1.., alpha1, ' ').parse_next(input)
}

fn parse_line<'a>(input: &mut &'a str) -> PResult<InputLine<'a>> {
    let (name, edges) = separated_pair(alpha1, ": ", parse_edges).parse_next(input)?;
    Ok(InputLine { name, edges })
}

fn parse_input<'a>(input: &mut &'a str) -> PResult<Vec<InputLine<'a>>> {
    let lines = separated(1.., parse_line, newline).parse_next(input)?;
    opt(newline).parse_next(input)?;
    Ok(lines)
}

#[derive(Debug, Clone, Default)]
struct Node<'a> {
    edges: Vec<&'a str>,
}

#[derive(Debug, Clone)]
struct Graph<'a> {
    nodes: FxHashMap<&'a str, Node<'a>>,
}

impl<'a> Graph<'a> {
    fn new(input: &[InputLine<'a>]) -> Self {
        let mut nodes: FxHashMap<&str, Node<'_>> = FxHashMap::default();

        for line in input {
            for &edge in &line.edges {
                nodes.entry(line.name).or_default().edges.push(edge);
                nodes.entry(edge).or_default().edges.push(line.name);
            }
        }

        Self { nodes }
    }
}

// Guaranteed by problem description
const MIN_CUT: u32 = 3;

// Modified version of Edmonds-Karp that simply treats every edge as having a capacity of 1
fn edmonds_karp(graph: &Graph<'_>, source: &str, sink: &str) -> u32 {
    let mut flow_edges: FxHashSet<(&str, &str)> = FxHashSet::default();
    let mut flow = 0;
    loop {
        let mut queue = VecDeque::new();
        queue.push_back(source);

        let mut path_to_node = FxHashMap::default();

        let mut path_found = false;
        'outer: while let Some(node_name) = queue.pop_front() {
            let node = graph.nodes.get(&node_name).unwrap();
            for &edge in &node.edges {
                if edge != source
                    && !flow_edges.contains(&(node_name, edge))
                    && !path_to_node.contains_key(&edge)
                {
                    path_to_node.insert(edge, node_name);

                    if edge == sink {
                        path_found = true;
                        break 'outer;
                    }

                    queue.push_back(edge);
                }
            }
        }

        if !path_found {
            break;
        }

        flow += 1;
        if flow > MIN_CUT {
            break;
        }

        let mut current_node_name = sink;
        while current_node_name != source {
            let prev_node_name = *path_to_node.get(&current_node_name).unwrap();
            flow_edges.insert((prev_node_name, current_node_name));

            current_node_name = prev_node_name;
        }
    }

    flow
}

fn solve(input: &str) -> u32 {
    let input = parse_input.parse(input).expect("Invalid input");
    let graph = Graph::new(&input);

    let mut partition_size = 0_u32;
    let source = *graph.nodes.keys().next().unwrap();
    for &sink in graph.nodes.keys().filter(|&&sink| sink != source) {
        let flow = edmonds_karp(&graph, source, sink);
        if flow == MIN_CUT {
            partition_size += 1;
        }
    }

    partition_size * (graph.nodes.len() as u32 - partition_size)
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = advent_of_code_2023::read_input()?;

    let solution = solve(&input);
    println!("{solution}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("../sample/day25.txt");

    #[test]
    fn sample_input() {
        assert_eq!(solve(SAMPLE_INPUT), 54);
    }
}
