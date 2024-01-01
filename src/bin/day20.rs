//! Day 20: Pulse Propagation
//!
//! <https://adventofcode.com/2023/day/20>
//!
//! Assumptions made:
//! - The "rx" module has a single input which is a conjunction module
//! - Each input to said conjunction module is essentially a counter that outputs a high pulse once every N low pulses
//!   sent to the broadcaster (for different values of N) and a low pulse on every other broadcaster low pulse
//! - Said N values are pairwise coprime
//!
//! Part 1: This is running a simulation. The simulation sends 1000 low pulses to the broadcaster in sequence, and each
//! time it counts how many low pulses and high pulses are sent in total (including the initial low pulse to the
//! broadcaster).
//!
//! Part 2: The answer is way too high to solve through simulation. Using the assumptions noted above (which were
//! discovered by investigating the input), this first finds how many broadcaster low pulses it takes to make each
//! counter module output a high pulse. "rx" will receive a low pulse when the conjunction module receives a high pulse
//! from every counter input on the same broadcaster low pulse, which will first happen at the LCM of all of the
//! counter N values (equivalent to the product since the N values are assumed to be pairwise coprime).

use advent_of_code_2023::impl_main;
use rustc_hash::FxHashMap;
use std::collections::VecDeque;
use std::iter;
use std::ops::{Add, AddAssign};
use winnow::ascii::{alpha1, newline};
use winnow::combinator::{alt, fail, opt, preceded, separated, separated_pair};

use winnow::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Pulse {
    Low,
    High,
}

#[derive(Debug, Clone)]
struct FlipFlop<'a> {
    name: &'a str,
    on: bool,
    outputs: Vec<&'a str>,
}

impl<'a> FlipFlop<'a> {
    fn new(name: &'a str, outputs: Vec<&'a str>) -> Self {
        Self { name, on: false, outputs }
    }
}

#[derive(Debug, Clone)]
struct InputConjunction<'a> {
    name: &'a str,
    outputs: Vec<&'a str>,
}

impl<'a> InputConjunction<'a> {
    fn new(name: &'a str, outputs: Vec<&'a str>) -> Self {
        Self { name, outputs }
    }
}

#[derive(Debug, Clone)]
struct Broadcaster<'a> {
    outputs: Vec<&'a str>,
}

#[derive(Debug, Clone)]
enum InputNode<'a> {
    FlipFlop(FlipFlop<'a>),
    Conjunction(InputConjunction<'a>),
    Broadcaster(Broadcaster<'a>),
}

#[derive(Debug, Clone)]
struct Input<'a> {
    flip_flops: Vec<FlipFlop<'a>>,
    conjunctions: Vec<InputConjunction<'a>>,
    broadcaster: Broadcaster<'a>,
}

fn parse_outputs<'a>(input: &mut &'a str) -> PResult<Vec<&'a str>> {
    separated(1.., alpha1, ", ").parse_next(input)
}

fn parse_flip_flop<'a>(input: &mut &'a str) -> PResult<InputNode<'a>> {
    '%'.parse_next(input)?;

    let (name, outputs) = separated_pair(alpha1, " -> ", parse_outputs).parse_next(input)?;

    Ok(InputNode::FlipFlop(FlipFlop::new(name, outputs)))
}

fn parse_conjunction<'a>(input: &mut &'a str) -> PResult<InputNode<'a>> {
    '&'.parse_next(input)?;

    let (name, outputs) = separated_pair(alpha1, " -> ", parse_outputs).parse_next(input)?;

    Ok(InputNode::Conjunction(InputConjunction::new(name, outputs)))
}

fn parse_broadcaster<'a>(input: &mut &'a str) -> PResult<InputNode<'a>> {
    let outputs = preceded("broadcaster -> ", parse_outputs).parse_next(input)?;
    Ok(InputNode::Broadcaster(Broadcaster { outputs }))
}

fn parse_node<'a>(input: &mut &'a str) -> PResult<InputNode<'a>> {
    alt((parse_flip_flop, parse_conjunction, parse_broadcaster)).parse_next(input)
}

fn parse_input<'a>(input: &mut &'a str) -> PResult<Input<'a>> {
    let nodes: Vec<_> = separated(1.., parse_node, newline).parse_next(input)?;
    opt(newline).parse_next(input)?;

    let mut flip_flops = Vec::new();
    let mut conjunctions = Vec::new();
    let mut broadcasters = Vec::new();
    for node in nodes {
        match node {
            InputNode::FlipFlop(flip_flop) => flip_flops.push(flip_flop),
            InputNode::Conjunction(conjunction) => conjunctions.push(conjunction),
            InputNode::Broadcaster(broadcaster) => broadcasters.push(broadcaster),
        }
    }

    if broadcasters.len() != 1 {
        return fail(input);
    }

    Ok(Input { flip_flops, conjunctions, broadcaster: broadcasters.into_iter().next().unwrap() })
}

#[derive(Debug, Clone)]
struct Conjunction<'a> {
    inputs: FxHashMap<&'a str, Pulse>,
    outputs: Vec<&'a str>,
}

#[derive(Debug, Clone)]
enum Node<'a> {
    FlipFlop(FlipFlop<'a>),
    Conjunction(Conjunction<'a>),
}

impl<'a> Node<'a> {
    fn outputs(&self) -> &[&'a str] {
        match self {
            Self::FlipFlop(flip_flop) => &flip_flop.outputs,
            Self::Conjunction(conjunction) => &conjunction.outputs,
        }
    }
}

fn build_node_map(input: Input<'_>) -> (FxHashMap<&str, Node<'_>>, Broadcaster<'_>) {
    let node_outputs: Vec<_> = input
        .flip_flops
        .iter()
        .map(|flip_flop| (flip_flop.name, &flip_flop.outputs))
        .chain(
            input.conjunctions.iter().map(|conjunction| (conjunction.name, &conjunction.outputs)),
        )
        .chain(iter::once(("broadcaster", &input.broadcaster.outputs)))
        .collect();

    let mut name_to_inputs: FxHashMap<&str, Vec<&str>> = FxHashMap::default();
    for (input_name, output_names) in node_outputs {
        for &output_name in output_names {
            name_to_inputs.entry(output_name).or_default().push(input_name);
        }
    }

    let map: FxHashMap<_, _> = input
        .flip_flops
        .into_iter()
        .map(|flip_flop| (flip_flop.name, Node::FlipFlop(flip_flop)))
        .chain(input.conjunctions.into_iter().map(|conjunction| {
            let inputs: FxHashMap<_, _> = name_to_inputs
                .get(conjunction.name)
                .map(|inputs| inputs.iter().map(|&input| (input, Pulse::Low)).collect())
                .unwrap_or_default();

            (
                conjunction.name,
                Node::Conjunction(Conjunction { inputs, outputs: conjunction.outputs }),
            )
        }))
        .collect();

    (map, input.broadcaster)
}

#[derive(Debug, Clone, Copy)]
struct PulseCount {
    low: u64,
    high: u64,
}

impl PulseCount {
    fn new() -> Self {
        Self { low: 0, high: 0 }
    }

    fn from_vec(pulses: &[(&str, &str, Pulse)]) -> Self {
        let mut pulse_count = Self::new();

        for &(_, _, pulse) in pulses {
            match pulse {
                Pulse::Low => pulse_count.low += 1,
                Pulse::High => pulse_count.high += 1,
            }
        }

        pulse_count
    }
}

impl Add for PulseCount {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self { low: self.low + rhs.low, high: self.high + rhs.high }
    }
}

impl AddAssign for PulseCount {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

fn send_pulse<'a>(
    node_map: &mut FxHashMap<&str, Node<'a>>,
    broadcaster: &Broadcaster<'a>,
) -> Vec<(&'a str, &'a str, Pulse)> {
    let mut pulse_queue = VecDeque::new();
    for &broadcaster_output in &broadcaster.outputs {
        pulse_queue.push_back(("broadcaster", broadcaster_output, Pulse::Low));
    }

    // Count the initial low pulse to the broadcaster
    let mut all_pulses = Vec::new();
    all_pulses.push(("", "broadcaster", Pulse::Low));

    while let Some((input_name, output_name, pulse)) = pulse_queue.pop_front() {
        all_pulses.push((input_name, output_name, pulse));

        // If the node is not in the map, assume it has no outputs
        let Some(output_node) = node_map.get_mut(output_name) else { continue };

        match output_node {
            Node::FlipFlop(flip_flop) => {
                if pulse == Pulse::Low {
                    let out_pulse = if flip_flop.on { Pulse::Low } else { Pulse::High };
                    flip_flop.on = !flip_flop.on;

                    for &flip_out_name in &flip_flop.outputs {
                        pulse_queue.push_back((output_name, flip_out_name, out_pulse));
                    }
                }
            }
            Node::Conjunction(conjunction) => {
                *conjunction.inputs.get_mut(input_name).expect("Invalid node in input") = pulse;

                let out_pulse = if conjunction.inputs.values().all(|&pulse| pulse == Pulse::High) {
                    Pulse::Low
                } else {
                    Pulse::High
                };

                for &conj_out_name in &conjunction.outputs {
                    pulse_queue.push_back((output_name, conj_out_name, out_pulse));
                }
            }
        }
    }

    all_pulses
}

fn solve_part_1(input: &str) -> u64 {
    let input = parse_input.parse(input).expect("Invalid input");
    let (mut node_map, broadcaster) = build_node_map(input);

    let mut pulse_count = PulseCount::new();
    for _ in 0..1000 {
        let all_pulses = send_pulse(&mut node_map, &broadcaster);
        pulse_count += PulseCount::from_vec(&all_pulses);
    }

    pulse_count.low * pulse_count.high
}

fn find_node_inputs<'a>(
    target_name: &str,
    node_map: &FxHashMap<&'a str, Node<'_>>,
) -> Vec<&'a str> {
    node_map
        .iter()
        .filter_map(|(&node_name, node)| node.outputs().contains(&target_name).then_some(node_name))
        .collect()
}

// Part 2 solution assumes that 'rx' has a single conjunction input, and that the inputs to the conjunction each operate
// on a fixed cycle where they output a high pulse every N button presses (for different values of N)
// Under these assumptions, the solution is the least common multiple of all of the cycle lengths (assumed to be
// pairwise coprime here)
fn solve_part_2(input: &str) -> u64 {
    let input = parse_input.parse(input).expect("Invalid input");
    let (mut node_map, broadcaster) = build_node_map(input);

    let rx_inputs = find_node_inputs("rx", &node_map);
    assert_eq!(
        rx_inputs.len(),
        1,
        "expected there to be exactly 1 input to 'rx', found {}",
        rx_inputs.len()
    );

    assert!(
        matches!(node_map.get(rx_inputs[0]), Some(Node::Conjunction(_))),
        "expected 'rx' input to be a conjunction node, was {:?}",
        node_map.get(rx_inputs[0])
    );

    let rx_input_inputs = find_node_inputs(rx_inputs[0], &node_map);
    let mut high_button_counts: FxHashMap<&str, u64> = FxHashMap::default();
    for button_count in 1.. {
        let all_pulses = send_pulse(&mut node_map, &broadcaster);
        for (input, output, pulse) in all_pulses {
            if output == rx_inputs[0] && pulse == Pulse::High {
                high_button_counts.entry(input).or_insert(button_count);
            }
        }

        if high_button_counts.len() == rx_input_inputs.len() {
            return high_button_counts.values().copied().product();
        }
    }

    unreachable!("loop over 1_u64.. will never terminate naturally")
}

impl_main!(p1: solve_part_1, p2: solve_part_2);

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("../../sample_input/day20.txt");
    const SAMPLE_INPUT_2: &str = include_str!("../../sample_input/day20-2.txt");

    #[test]
    fn sample_input_part_1() {
        assert_eq!(solve_part_1(SAMPLE_INPUT), 32000000);
        assert_eq!(solve_part_1(SAMPLE_INPUT_2), 11687500);
    }
}
