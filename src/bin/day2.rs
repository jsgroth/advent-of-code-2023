//! Day 2: Cube Conundrum
//!
//! <https://adventofcode.com/2023/day/2>

use std::error::Error;
use winnow::ascii::{alpha1, digit1};
use winnow::combinator::{delimited, fail, separated, separated_pair};
use winnow::prelude::*;

#[derive(Debug, Clone, Default)]
struct Reveal {
    red: Option<u32>,
    green: Option<u32>,
    blue: Option<u32>,
}

#[derive(Debug, Clone)]
struct Game {
    id: u32,
    reveals: Vec<Reveal>,
}

fn parse_u32(input: &mut &str) -> PResult<u32> {
    digit1.parse_to().parse_next(input)
}

fn parse_reveal_field<'a>(input: &mut &'a str) -> PResult<(u32, &'a str)> {
    separated_pair(parse_u32, ' ', alpha1).parse_next(input)
}

fn parse_reveal(input: &mut &str) -> PResult<Reveal> {
    let fields: Vec<_> = separated(1.., parse_reveal_field, ", ").parse_next(input)?;

    let mut reveal = Reveal::default();
    for (number, color) in fields {
        match color {
            "red" => reveal.red = Some(number),
            "green" => reveal.green = Some(number),
            "blue" => reveal.blue = Some(number),
            _ => return fail(input),
        }
    }

    Ok(reveal)
}

fn parse_game(input: &mut &str) -> PResult<Game> {
    let game_id = delimited("Game ", parse_u32, ": ").parse_next(input)?;

    let reveals = separated(1.., parse_reveal, "; ").parse_next(input)?;

    Ok(Game { id: game_id, reveals })
}

fn solve_part_1(input: &str) -> u32 {
    input
        .lines()
        .filter_map(|line| {
            let game = parse_game.parse(line).expect("Failed to parse game");

            game.reveals
                .iter()
                .all(|game| {
                    game.red.unwrap_or(0) <= 12
                        && game.green.unwrap_or(0) <= 13
                        && game.blue.unwrap_or(0) <= 14
                })
                .then_some(game.id)
        })
        .sum()
}

fn solve_part_2(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let game = parse_game.parse(line).expect("Failed to parse game");

            let red = game.reveals.iter().filter_map(|reveal| reveal.red).max().unwrap_or(0);
            let green = game.reveals.iter().filter_map(|reveal| reveal.green).max().unwrap_or(0);
            let blue = game.reveals.iter().filter_map(|reveal| reveal.blue).max().unwrap_or(0);

            red * green * blue
        })
        .sum()
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

    const SAMPLE_INPUT: &str = include_str!("../sample/day2.txt");

    #[test]
    fn sample_input_part_1() {
        assert_eq!(solve_part_1(SAMPLE_INPUT), 8);
    }

    #[test]
    fn sample_input_part_2() {
        assert_eq!(solve_part_2(SAMPLE_INPUT), 2286);
    }
}
