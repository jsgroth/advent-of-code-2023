//! Day 25: Snowverload
//!
//! <https://adventofcode.com/2023/day/25>
//!
//! This is treated as a variant of the minimum cut graph problem. The value of the min cut is known (3), so instead
//! of finding the min cut, repeatedly use the Edmonds-Karp algorithm to find a pair of nodes where the max flow / min cut
//! between the nodes is equal to 3, and then do a basic BFS to count the number of nodes that are reachable from the source
//! after the network is saturated with max flow between the two nodes.

use rustc_hash::{FxHashMap, FxHashSet};
use std::cmp;
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

#[derive(Debug, Clone)]
struct Edge {
    flow: i32,
    capacity: i32,
}

impl Edge {
    fn new() -> Self {
        Self { flow: 0, capacity: 1 }
    }
}

#[derive(Debug, Clone, Default)]
struct Node<'a> {
    edges: FxHashMap<&'a str, Edge>,
}

impl<'a> Node<'a> {
    fn edge(&self, edge_name: &str) -> &Edge {
        self.edges.get(edge_name).unwrap()
    }

    fn edge_mut(&mut self, edge_name: &str) -> &mut Edge {
        self.edges.get_mut(edge_name).unwrap()
    }
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
                nodes.entry(line.name).or_default().edges.insert(edge, Edge::new());
                nodes.entry(edge).or_default().edges.insert(line.name, Edge::new());
            }
        }

        Self { nodes }
    }

    fn node(&self, node_name: &str) -> &Node<'a> {
        self.nodes.get(node_name).unwrap()
    }

    fn node_mut(&mut self, node_name: &str) -> &mut Node<'a> {
        self.nodes.get_mut(node_name).unwrap()
    }
}

// Guaranteed by problem description
const MIN_CUT: u32 = 3;

fn edmonds_karp(graph: &mut Graph<'_>, source: &str, sink: &str) -> u32 {
    let mut flow = 0;
    loop {
        let mut queue = VecDeque::new();
        queue.push_back(source);

        let mut path_to_node = FxHashMap::default();

        let mut path_found = false;
        'outer: while let Some(node_name) = queue.pop_front() {
            let node = graph.nodes.get(&node_name).unwrap();
            for (&edge_name, edge) in &node.edges {
                if edge_name != source
                    && edge.flow < edge.capacity
                    && !path_to_node.contains_key(edge_name)
                {
                    path_to_node.insert(edge_name, node_name);

                    if edge_name == sink {
                        path_found = true;
                        break 'outer;
                    }

                    queue.push_back(edge_name);
                }
            }
        }

        if !path_found {
            break;
        }

        let mut added_flow = i32::MAX;
        let mut current_node_name = sink;
        while current_node_name != source {
            let prev_node_name = *path_to_node.get(current_node_name).unwrap();
            let edge = graph.node(prev_node_name).edge(current_node_name);
            added_flow = cmp::min(added_flow, edge.capacity - edge.flow);

            current_node_name = prev_node_name;
        }

        let mut current_node_name = sink;
        while current_node_name != source {
            let prev_node_name = *path_to_node.get(current_node_name).unwrap();
            let edge = graph.node_mut(prev_node_name).edge_mut(current_node_name);
            edge.flow += added_flow;

            let residual_edge = graph.node_mut(current_node_name).edge_mut(prev_node_name);
            residual_edge.flow -= added_flow;

            current_node_name = prev_node_name;
        }

        flow += added_flow;
        if flow > MIN_CUT as i32 {
            break;
        }
    }

    assert!(flow >= 0);
    flow as u32
}

fn determine_partition_size(graph: &Graph<'_>, source: &str) -> u32 {
    let mut queue = VecDeque::new();
    queue.push_back(source);

    let mut visited = FxHashSet::default();
    visited.insert(source);

    // Initialize to 1 to include the source
    let mut partition_size = 1;
    while let Some(node_name) = queue.pop_front() {
        for (&edge_name, edge) in &graph.node(node_name).edges {
            if edge.flow < edge.capacity && visited.insert(edge_name) {
                queue.push_back(edge_name);
                partition_size += 1;
            }
        }
    }

    partition_size
}

fn solve(input: &str) -> u32 {
    let input = parse_input.parse(input).expect("Invalid input");
    let graph = Graph::new(&input);

    let source = *graph.nodes.keys().next().unwrap();
    for &sink in graph.nodes.keys().filter(|&&sink| sink != source) {
        let mut graph = graph.clone();
        let flow = edmonds_karp(&mut graph, source, sink);
        if flow == MIN_CUT {
            let partition_size = determine_partition_size(&graph, source);
            return partition_size * (graph.nodes.len() as u32 - partition_size);
        }
    }

    panic!("no solution found")
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

    const SAMPLE_INPUT: &str = include_str!("../../sample_input/day25.txt");

    #[test]
    fn sample_input() {
        assert_eq!(solve(SAMPLE_INPUT), 54);
    }
}
