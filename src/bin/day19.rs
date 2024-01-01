//! Day 19: Aplenty
//!
//! <https://adventofcode.com/2023/day/19>
//!
//! Assumptions made:
//! - There are no infinite loops in the workflows
//!
//! Part 1: This is simply parsing the input and then simulating each part through the workflows to determine whether
//! it is ultimately accepted or rejected, starting at the "in" workflow.
//!
//! Part 2: This is running ranges through the simulation rather than parts. At the start, each field is allowed to have
//! a value ranging from 1 to 4000. At each workflow step, the range for that field gets split into two: the part of the
//! range that meets the condition and branches to the other workflow, and the part of the range that fails to meet
//! the condition and continues in the current workflow. If either sub-range is empty then there are no possible Accepts
//! down that path and the simulation does not proceed that way.
//!
//! When an Accept is reached, the number of valid part values down that path is equal to the product of the range
//! length for each of the 4 fields.

use advent_of_code_2023::impl_main;
use rustc_hash::FxHashMap;
use winnow::ascii::{alpha1, digit1, newline};
use winnow::combinator::{
    delimited, fail, opt, repeat, separated, separated_pair, success, terminated,
};
use winnow::dispatch;

use winnow::prelude::*;
use winnow::token::any;

#[derive(Debug, Clone, Default)]
struct Part {
    x: u32,
    m: u32,
    a: u32,
    s: u32,
}

impl Part {
    fn value(&self) -> u32 {
        self.x + self.m + self.a + self.s
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PartField {
    X,
    M,
    A,
    S,
}

impl Part {
    fn field(&self, field: PartField) -> u32 {
        match field {
            PartField::X => self.x,
            PartField::M => self.m,
            PartField::A => self.a,
            PartField::S => self.s,
        }
    }

    fn field_mut(&mut self, field: PartField) -> &mut u32 {
        match field {
            PartField::X => &mut self.x,
            PartField::M => &mut self.m,
            PartField::A => &mut self.a,
            PartField::S => &mut self.s,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Comparison {
    Greater,
    Less,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Condition(PartField, Comparison, u32);

impl Condition {
    fn check(self, part: &Part) -> bool {
        match self.1 {
            Comparison::Greater => part.field(self.0) > self.2,
            Comparison::Less => part.field(self.0) < self.2,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Destination<'a> {
    Workflow(&'a str),
    Accept,
    Reject,
}

#[derive(Debug, Clone, Copy)]
struct Rule<'a>(Condition, Destination<'a>);

#[derive(Debug, Clone, Copy)]
struct FlexibleRule<'a>(Option<Condition>, Destination<'a>);

#[derive(Debug, Clone)]
struct Workflow<'a> {
    name: &'a str,
    rules: Vec<Rule<'a>>,
    final_condition: Destination<'a>,
}

#[derive(Debug, Clone)]
struct Input<'a> {
    workflows: Vec<Workflow<'a>>,
    parts: Vec<Part>,
}

fn parse_part_field_name(input: &mut &str) -> PResult<PartField> {
    dispatch! { any;
        'x' => success(PartField::X),
        'm' => success(PartField::M),
        'a' => success(PartField::A),
        's' => success(PartField::S),
        _ => fail,
    }
    .parse_next(input)
}

fn parse_condition(input: &mut &str) -> PResult<Condition> {
    let field = parse_part_field_name.parse_next(input)?;

    let comparison = dispatch! { any;
        '>' => success(Comparison::Greater),
        '<' => success(Comparison::Less),
        _ => fail,
    }
    .parse_next(input)?;

    let value = digit1.parse_to().parse_next(input)?;

    Ok(Condition(field, comparison, value))
}

fn parse_rule<'a>(input: &mut &'a str) -> PResult<FlexibleRule<'a>> {
    let condition = opt(terminated(parse_condition, ':')).parse_next(input)?;
    let destination = dispatch! { alpha1;
        "A" => success(Destination::Accept),
        "R" => success(Destination::Reject),
        workflow_name => success(Destination::Workflow(workflow_name))
    }
    .parse_next(input)?;

    Ok(FlexibleRule(condition, destination))
}

fn parse_workflow<'a>(input: &mut &'a str) -> PResult<Workflow<'a>> {
    let name = alpha1.parse_next(input)?;

    let flex_rules: Vec<_> =
        delimited('{', separated(1.., parse_rule, ','), '}').parse_next(input)?;

    if flex_rules.is_empty()
        || flex_rules.last().unwrap().0.is_some()
        || flex_rules[..flex_rules.len() - 1].iter().any(|rule| rule.0.is_none())
    {
        return fail(input);
    }

    let rules: Vec<_> = flex_rules[..flex_rules.len() - 1]
        .iter()
        .map(|rule| Rule(rule.0.unwrap(), rule.1))
        .collect();
    let final_condition = flex_rules.last().unwrap().1;
    Ok(Workflow { name, rules, final_condition })
}

fn parse_part_field(input: &mut &str) -> PResult<(PartField, u32)> {
    separated_pair(parse_part_field_name, '=', digit1.parse_to()).parse_next(input)
}

fn parse_part(input: &mut &str) -> PResult<Part> {
    '{'.parse_next(input)?;

    let fields: Vec<_> = separated(4, parse_part_field, ',').parse_next(input)?;

    let mut part = Part::default();
    for (field, value) in fields {
        *part.field_mut(field) = value;
    }

    '}'.parse_next(input)?;

    Ok(part)
}

fn parse_input<'a>(input: &mut &'a str) -> PResult<Input<'a>> {
    let workflows = separated(1.., parse_workflow, newline).parse_next(input)?;

    repeat(2, newline).parse_next(input)?;

    let parts = separated(1.., parse_part, newline).parse_next(input)?;

    opt(newline).parse_next(input)?;

    Ok(Input { workflows, parts })
}

fn build_workflow_map<'a>(workflows: &[Workflow<'a>]) -> FxHashMap<&'a str, Workflow<'a>> {
    workflows.iter().map(|workflow| (workflow.name, workflow.clone())).collect()
}

fn check_part(part: &Part, workflow_map: &FxHashMap<&str, Workflow<'_>>) -> bool {
    let mut current_workflow = workflow_map.get("in").expect("No 'in' workflow in input");
    loop {
        let mut destination = None;
        for &Rule(condition, rule_destination) in &current_workflow.rules {
            if condition.check(part) {
                destination = Some(rule_destination);
                break;
            }
        }

        let destination = destination.unwrap_or(current_workflow.final_condition);
        match destination {
            Destination::Accept => return true,
            Destination::Reject => return false,
            Destination::Workflow(workflow_name) => {
                current_workflow =
                    workflow_map.get(workflow_name).expect("Invalid workflow name in input")
            }
        }
    }
}

fn solve_part_1(input: &str) -> u32 {
    let input = parse_input.parse(input).expect("Invalid input");
    let workflow_map = build_workflow_map(&input.workflows);

    input
        .parts
        .into_iter()
        .filter_map(|part| check_part(&part, &workflow_map).then(|| part.value()))
        .sum()
}

#[derive(Debug, Clone)]
struct FieldRange {
    min: u32,
    max: u32,
}

impl FieldRange {
    fn new() -> Self {
        Self { min: 1, max: 4000 }
    }

    fn range(&self) -> u64 {
        (self.max - self.min + 1).into()
    }
}

#[derive(Debug, Clone)]
struct PartRanges {
    x: FieldRange,
    m: FieldRange,
    a: FieldRange,
    s: FieldRange,
}

impl PartRanges {
    fn new() -> Self {
        Self {
            x: FieldRange::new(),
            m: FieldRange::new(),
            a: FieldRange::new(),
            s: FieldRange::new(),
        }
    }

    fn field_mut(&mut self, field: PartField) -> &mut FieldRange {
        match field {
            PartField::X => &mut self.x,
            PartField::M => &mut self.m,
            PartField::A => &mut self.a,
            PartField::S => &mut self.s,
        }
    }

    fn possible_combinations(&self) -> u64 {
        self.x.range() * self.m.range() * self.a.range() * self.s.range()
    }
}

fn find_possible_combinations(
    mut range: PartRanges,
    workflow: &Workflow<'_>,
    workflow_map: &FxHashMap<&str, Workflow<'_>>,
) -> u64 {
    let mut count = 0;

    for &Rule(Condition(field, comparison, value), destination) in &workflow.rules {
        match comparison {
            Comparison::Greater => {
                if range.field_mut(field).max > value {
                    let mut range = range.clone();
                    range.field_mut(field).min = value + 1;

                    count += check_next_destination(range, destination, workflow_map);
                }

                if range.field_mut(field).min > value {
                    // Impossible to reach any future rules in this workflow
                    return count;
                }

                range.field_mut(field).max = value;
            }
            Comparison::Less => {
                if range.field_mut(field).min < value {
                    let mut range = range.clone();
                    range.field_mut(field).max = value - 1;

                    count += check_next_destination(range, destination, workflow_map);
                }

                if range.field_mut(field).max < value {
                    // Impossible to reach any future rules in this workflow
                    return count;
                }

                range.field_mut(field).min = value;
            }
        }
    }

    count += check_next_destination(range, workflow.final_condition, workflow_map);

    count
}

fn check_next_destination(
    range: PartRanges,
    destination: Destination<'_>,
    workflow_map: &FxHashMap<&str, Workflow<'_>>,
) -> u64 {
    match destination {
        Destination::Accept => range.possible_combinations(),
        Destination::Reject => 0,
        Destination::Workflow(workflow_name) => {
            let next_workflow =
                workflow_map.get(workflow_name).expect("Invalid workflow name in input");
            find_possible_combinations(range, next_workflow, workflow_map)
        }
    }
}

fn solve_part_2(input: &str) -> u64 {
    let input = parse_input.parse(input).expect("Invalid input");
    let workflow_map = build_workflow_map(&input.workflows);

    let start_workflow = workflow_map.get("in").expect("No 'in' workflow in input");
    find_possible_combinations(PartRanges::new(), start_workflow, &workflow_map)
}

impl_main!(p1: solve_part_1, p2: solve_part_2);

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("../../sample_input/day19.txt");

    #[test]
    fn sample_input_part_1() {
        assert_eq!(solve_part_1(SAMPLE_INPUT), 19114);
    }

    #[test]
    fn sample_input_part_2() {
        assert_eq!(solve_part_2(SAMPLE_INPUT), 167409079868000);
    }
}
