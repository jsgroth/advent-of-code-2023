//! Day 2: Cube Conundrum
//!
//! <https://adventofcode.com/2023/day/2>

use std::error::Error;
use std::str::FromStr;

#[derive(Debug, Clone)]
struct Reveal {
    red: Option<u32>,
    green: Option<u32>,
    blue: Option<u32>,
}

impl FromStr for Reveal {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let err_fn_0 = || format!("Invalid reveal string: '{s}'");
        let err_fn_1 = |err| format!("Invalid reveal string: '{s}': {err}");

        let mut red = None;
        let mut green = None;
        let mut blue = None;

        for cubes_str in s.split(", ") {
            let (num_str, color) = cubes_str.split_once(' ').ok_or_else(err_fn_0)?;
            let num = num_str.parse().map_err(err_fn_1)?;

            match color {
                "red" => red = Some(num),
                "green" => green = Some(num),
                "blue" => blue = Some(num),
                _ => return Err(err_fn_0()),
            }
        }

        Ok(Self { red, green, blue })
    }
}

#[derive(Debug, Clone)]
struct Game {
    id: u32,
    reveals: Vec<Reveal>,
}

impl FromStr for Game {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let err_fn_0 = || format!("Invalid game string: '{s}'");
        let err_fn_1 = |err| format!("Invalid game string: '{s}': {err}");

        let (game_str, reveals_str) = s.split_once(": ").ok_or_else(err_fn_0)?;
        let (_, game_id_str) = game_str.split_once(' ').ok_or_else(err_fn_0)?;
        let game_id = game_id_str.parse().map_err(err_fn_1)?;

        let reveals = reveals_str
            .split("; ")
            .map(|reveal_str| reveal_str.parse::<Reveal>())
            .collect::<Result<_, _>>()?;

        Ok(Self { id: game_id, reveals })
    }
}

fn solve_part_1(input: &str) -> u32 {
    input
        .lines()
        .filter_map(|line| {
            let game = line.parse::<Game>().expect("Failed to parse game line");

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
            let game = line.parse::<Game>().expect("Failed to parse game line");

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
