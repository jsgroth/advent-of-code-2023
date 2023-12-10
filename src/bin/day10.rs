//! Day 10: Pipe Maze
//!
//! <https://adventofcode.com/2023/day/10>

use advent_of_code_2023::impl_standard_main;
use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    const ALL: [Self; 4] = [Self::North, Self::South, Self::East, Self::West];

    const fn x_diff(self) -> i32 {
        match self {
            Self::West => -1,
            Self::East => 1,
            Self::North | Self::South => 0,
        }
    }

    const fn y_diff(self) -> i32 {
        match self {
            Self::North => -1,
            Self::South => 1,
            Self::West | Self::East => 0,
        }
    }

    const fn inverse(self) -> Self {
        match self {
            Self::North => Self::South,
            Self::South => Self::North,
            Self::West => Self::East,
            Self::East => Self::West,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Space {
    Empty,
    Start,
    Pipe([Direction; 2]),
}

const EMPTY_DIRECTIONS: &[Direction] = &[];

impl Space {
    fn adjacent_directions(&self) -> &[Direction] {
        match self {
            Self::Empty => EMPTY_DIRECTIONS,
            Self::Start => &Direction::ALL,
            Self::Pipe(directions) => directions,
        }
    }
}

impl From<char> for Space {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Empty,
            'S' => Self::Start,
            '|' => Self::Pipe([Direction::North, Direction::South]),
            '-' => Self::Pipe([Direction::West, Direction::East]),
            'L' => Self::Pipe([Direction::North, Direction::East]),
            'J' => Self::Pipe([Direction::North, Direction::West]),
            '7' => Self::Pipe([Direction::South, Direction::West]),
            'F' => Self::Pipe([Direction::South, Direction::East]),
            _ => panic!("Invalid input char: {value}"),
        }
    }
}

fn parse_input(input: &str) -> Vec<Vec<Space>> {
    input.lines().map(|line| line.chars().map(Space::from).collect()).collect()
}

fn solve_part_1(input: &str) -> u32 {
    let map = parse_input(input);

    let (start_i, start_j) = find_start(&map);

    let (_, loop_len) = traverse_map(&map, start_i, start_j);
    (loop_len + 1) / 2
}

fn find_start(map: &[Vec<Space>]) -> (usize, usize) {
    map.iter()
        .enumerate()
        .find_map(|(i, row)| {
            row.iter().enumerate().find_map(|(j, &space)| (space == Space::Start).then_some((i, j)))
        })
        .expect("No start position in map")
}

fn traverse_map(map: &[Vec<Space>], start_i: usize, start_j: usize) -> (HashSet<(i32, i32)>, u32) {
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    visited.insert((start_i as i32, start_j as i32));

    let mut current_i = start_i as i32;
    let mut current_j = start_j as i32;
    let mut steps = 0;
    loop {
        let mut found_path = false;
        for &direction in map[current_i as usize][current_j as usize].adjacent_directions() {
            let i = current_i + direction.y_diff();
            let j = current_j + direction.x_diff();
            if !(0..map.len() as i32).contains(&i) || !(0..map[0].len() as i32).contains(&j) {
                continue;
            }

            let adjacent_space = map[i as usize][j as usize];
            if let Space::Pipe(pipe_dirs) = adjacent_space {
                if !visited.contains(&(i, j)) && pipe_dirs.contains(&direction.inverse()) {
                    found_path = true;
                    current_i = i;
                    current_j = j;
                    visited.insert((current_i, current_j));
                    break;
                }
            }
        }

        if !found_path {
            // Reached the end of the loop
            break;
        }

        steps += 1;
    }

    (visited, steps)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FloodSpace {
    Unknown,
    Pipe,
    Outside,
}

fn solve_part_2(input: &str) -> u32 {
    let mut map = parse_input(input);

    let (start_i, start_j) = find_start(&map);

    let (loop_spaces, _) = traverse_map(&map, start_i, start_j);

    let start_directions = determine_start_directions(&map, start_i, start_j);
    map[start_i][start_j] = Space::Pipe([start_directions[0], start_directions[1]]);

    let mut flood_map = vec![vec![FloodSpace::Unknown; 2 * map[0].len() - 1]; 2 * map.len() - 1];
    fill_in_pipes(&map, &mut flood_map, &loop_spaces);

    for i in 0..flood_map.len() {
        floodfill(&mut flood_map, i, 0);
        let last_col = flood_map[0].len() - 1;
        floodfill(&mut flood_map, i, last_col);
    }

    for j in 0..flood_map[0].len() {
        floodfill(&mut flood_map, 0, j);
        let last_row = flood_map.len() - 1;
        floodfill(&mut flood_map, last_row, j);
    }

    let mut inside_count = 0;
    for i in (0..flood_map.len()).step_by(2) {
        for j in (0..flood_map[0].len()).step_by(2) {
            if flood_map[i][j] == FloodSpace::Unknown {
                inside_count += 1;
            }
        }
    }

    inside_count
}

fn determine_start_directions(
    map: &[Vec<Space>],
    start_i: usize,
    start_j: usize,
) -> Vec<Direction> {
    Direction::ALL
        .into_iter()
        .filter(|direction| {
            let i = start_i as i32 + direction.y_diff();
            let j = start_j as i32 + direction.x_diff();

            if !(0..map.len() as i32).contains(&i) || !(0..map[0].len() as i32).contains(&j) {
                return false;
            }

            let Space::Pipe(pipe_dirs) = map[i as usize][j as usize] else { return false };
            pipe_dirs.contains(&direction.inverse())
        })
        .collect()
}

fn fill_in_pipes(
    map: &[Vec<Space>],
    flood_map: &mut Vec<Vec<FloodSpace>>,
    loop_spaces: &HashSet<(i32, i32)>,
) {
    for &(i, j) in loop_spaces {
        flood_map[(2 * i) as usize][(2 * j) as usize] = FloodSpace::Pipe;
    }

    for i in 0..flood_map.len() {
        for j in 0..flood_map[i].len() {
            if i % 2 != 0 && j % 2 == 0 {
                let Space::Pipe(north_dirs) = map[(i - 1) / 2][j / 2] else {
                    continue;
                };
                let Space::Pipe(south_dirs) = map[(i + 1) / 2][j / 2] else {
                    continue;
                };

                if north_dirs.contains(&Direction::South) && south_dirs.contains(&Direction::North)
                {
                    flood_map[i][j] = FloodSpace::Pipe;
                }
            }

            if i % 2 == 0 && j % 2 != 0 {
                let Space::Pipe(west_dirs) = map[i / 2][(j - 1) / 2] else { continue };
                let Space::Pipe(east_dirs) = map[i / 2][(j + 1) / 2] else { continue };

                if west_dirs.contains(&Direction::East) && east_dirs.contains(&Direction::West) {
                    flood_map[i][j] = FloodSpace::Pipe;
                }
            }
        }
    }
}

fn floodfill(flood_map: &mut Vec<Vec<FloodSpace>>, i: usize, j: usize) {
    if flood_map[i][j] != FloodSpace::Unknown {
        return;
    }

    flood_map[i][j] = FloodSpace::Outside;

    for (di, dj) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
        let ii = i as i32 + di;
        let jj = j as i32 + dj;
        if !(0..flood_map.len() as i32).contains(&ii)
            || !(0..flood_map[0].len() as i32).contains(&jj)
        {
            continue;
        }

        floodfill(flood_map, ii as usize, jj as usize);
    }
}

impl_standard_main!(p1: solve_part_1, p2: solve_part_2);

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("../sample/day10.txt");
    const SAMPLE_INPUT_2: &str = include_str!("../sample/day10-2.txt");
    const SAMPLE_INPUT_3: &str = include_str!("../sample/day10-3.txt");
    const SAMPLE_INPUT_4: &str = include_str!("../sample/day10-4.txt");
    const SAMPLE_INPUT_5: &str = include_str!("../sample/day10-5.txt");

    #[test]
    fn sample_input_part_1() {
        assert_eq!(solve_part_1(SAMPLE_INPUT), 4);
        assert_eq!(solve_part_1(SAMPLE_INPUT_2), 8);
    }

    #[test]
    fn sample_input_part_2() {
        assert_eq!(solve_part_2(SAMPLE_INPUT_3), 4);
        assert_eq!(solve_part_2(SAMPLE_INPUT_4), 8);
        assert_eq!(solve_part_2(SAMPLE_INPUT_5), 10);
    }
}
