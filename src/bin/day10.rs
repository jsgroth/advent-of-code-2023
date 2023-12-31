//! Day 10: Pipe Maze
//!
//! <https://adventofcode.com/2023/day/10>
//!
//! Part 1: Starting from the `S`, traverse the grid to find all spaces that are part of the pipe loop. Use the pipe
//! orientations to determine which directions are valid to move at each step. For the starting position, look at which
//! of the 4 adjacent spaces contain pipes that are oriented towards the starting position.
//!
//! The loop length must be an even number of steps because it must take an even number of steps to get back to the
//! starting position, so the distance to the farthest position is always half of the loop length.
//!
//! Part 2: Expand the map to "double resolution" by copying each pipe at `M[i,j]` in the original map into `M[2i,2j]`
//! in the expanded map, and then filling in the odd-numbered spaces for pipes that are part of the loop. Pipes that are
//! not part of the loop are not copied into the expanded map.
//!
//! Next, perform a floodfill in the expanded map to determine all spaces that are reachable from the map borders.
//!
//! The answer is the number of spaces that were not reached by the floodfill, are not part of the loop, and are present
//! in the original-resolution map (i.e. i % 2 == 0 and j % 2 == 0).

use advent_of_code_2023::impl_main;
use rustc_hash::FxHashSet;

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
    let loop_spaces = find_loop_spaces(&map, start_i, start_j);

    loop_spaces.len() as u32 / 2
}

fn find_start(map: &[Vec<Space>]) -> (usize, usize) {
    map.iter()
        .enumerate()
        .find_map(|(i, row)| {
            row.iter().enumerate().find_map(|(j, &space)| (space == Space::Start).then_some((i, j)))
        })
        .expect("No start position in map")
}

// Find all positions that are part of the loop
fn find_loop_spaces(map: &[Vec<Space>], start_i: usize, start_j: usize) -> FxHashSet<(i32, i32)> {
    let mut visited: FxHashSet<(i32, i32)> = FxHashSet::default();
    visited.insert((start_i as i32, start_j as i32));

    let mut current_i = start_i as i32;
    let mut current_j = start_j as i32;
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
            return visited;
        }
    }
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

    let loop_spaces = find_loop_spaces(&map, start_i, start_j);

    // Replace the start pipe with a regular pipe
    let start_directions = determine_start_directions(&map, start_i, start_j);
    map[start_i][start_j] = Space::Pipe([start_directions[0], start_directions[1]]);

    // Generate a new map that is ~double the size/resolution
    let mut flood_map = vec![vec![FloodSpace::Unknown; 2 * map[0].len() - 1]; 2 * map.len() - 1];
    fill_in_pipes(&map, &mut flood_map, &loop_spaces);

    // Floodfill starting from left and right columns
    for i in 0..flood_map.len() {
        floodfill(&mut flood_map, i, 0);
        let last_col = flood_map[0].len() - 1;
        floodfill(&mut flood_map, i, last_col);
    }

    // Floodfill starting from top and bottom rows
    for j in 0..flood_map[0].len() {
        floodfill(&mut flood_map, 0, j);
        let last_row = flood_map.len() - 1;
        floodfill(&mut flood_map, last_row, j);
    }

    // Any space that has not been filled must be inside the loop
    // Only count spaces that are present at original resolution (i % 2 == 0 && j % 2 == 0)
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

#[allow(clippy::needless_range_loop)]
fn fill_in_pipes(
    map: &[Vec<Space>],
    flood_map: &mut Vec<Vec<FloodSpace>>,
    loop_spaces: &FxHashSet<(i32, i32)>,
) {
    for &(i, j) in loop_spaces {
        flood_map[(2 * i) as usize][(2 * j) as usize] = FloodSpace::Pipe;
    }

    for i in 0..flood_map.len() {
        for j in 0..flood_map[i].len() {
            if i % 2 != 0 && j % 2 == 0 {
                // Odd row, even column; check if spaces above and below are connected pipes
                let north_row = (i - 1) / 2;
                let south_row = (i + 1) / 2;
                let col = j / 2;
                if !loop_spaces.contains(&(north_row as i32, col as i32))
                    || !loop_spaces.contains(&(south_row as i32, col as i32))
                {
                    continue;
                }

                let Space::Pipe(north_dirs) = map[north_row][col] else { continue };
                let Space::Pipe(south_dirs) = map[south_row][col] else { continue };

                if north_dirs.contains(&Direction::South) && south_dirs.contains(&Direction::North)
                {
                    flood_map[i][j] = FloodSpace::Pipe;
                }
            }

            if i % 2 == 0 && j % 2 != 0 {
                // Even row, odd column; check if spaces left and right are connected pipes
                let row = i / 2;
                let west_col = (j - 1) / 2;
                let east_col = (j + 1) / 2;
                if !loop_spaces.contains(&(row as i32, west_col as i32))
                    || !loop_spaces.contains(&(row as i32, east_col as i32))
                {
                    continue;
                }

                let Space::Pipe(west_dirs) = map[row][west_col] else { continue };
                let Space::Pipe(east_dirs) = map[row][east_col] else { continue };

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

impl_main!(p1: solve_part_1, p2: solve_part_2);

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("../../sample_input/day10.txt");
    const SAMPLE_INPUT_2: &str = include_str!("../../sample_input/day10-2.txt");
    const SAMPLE_INPUT_3: &str = include_str!("../../sample_input/day10-3.txt");
    const SAMPLE_INPUT_4: &str = include_str!("../../sample_input/day10-4.txt");
    const SAMPLE_INPUT_5: &str = include_str!("../../sample_input/day10-5.txt");

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
