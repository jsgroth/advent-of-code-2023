//! Day 2: Cube Conundrum
//!
//! <https://adventofcode.com/2023/day/2>

use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, char, digit1};
use nom::combinator::map_res;
use nom::multi::separated_list1;
use nom::sequence::{delimited, separated_pair};
use nom::IResult;
use std::error::Error;

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

fn parse_u32(input: &str) -> IResult<&str, u32> {
    map_res(digit1, str::parse)(input)
}

fn parse_reveal_field(input: &str) -> IResult<&str, (u32, &str)> {
    separated_pair(parse_u32, char(' '), alpha1)(input)
}

fn parse_reveal(input: &str) -> IResult<&str, Reveal> {
    let (input, fields) = separated_list1(tag(", "), parse_reveal_field)(input)?;

    let reveal = fields.into_iter().fold(Reveal::default(), |mut reveal, (number, color)| {
        match color {
            "red" => reveal.red = Some(number),
            "green" => reveal.green = Some(number),
            "blue" => reveal.blue = Some(number),
            _ => panic!("Invalid color string: {color}"),
        }
        reveal
    });

    Ok((input, reveal))
}

fn parse_game(input: &str) -> IResult<&str, Game> {
    let (rest, game_id) = delimited(tag("Game "), parse_u32, tag(": "))(input)?;

    let (rest, reveals) = separated_list1(tag("; "), parse_reveal)(rest)?;

    Ok((rest, Game { id: game_id, reveals }))
}

fn solve_part_1(input: &str) -> u32 {
    input
        .lines()
        .filter_map(|line| {
            let (_, game) = parse_game(line).expect("Failed to parse game");

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
            let (_, game) = parse_game(line).expect("Failed to parse game");

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
