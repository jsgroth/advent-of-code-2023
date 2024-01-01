//! Day 21: Step Counter
//!
//! <https://adventofcode.com/2023/day/21>
//!
//! Assumptions made:
//! - The map is square
//! - The topmost row, the bottommost row, the leftmost column, and the rightmost column are all completely empty
//!
//! Part 1: Do a breadth-first search to find all spaces that can be reached in N or fewer steps from the starting
//! position. Each space can be reached in only an odd number of steps or an even number steps - 64 is even, so return
//! the number of spaces touched that the elf can reach with an even number of steps from the starting position.
//!
//! Part 2: This is a doozy. It's worth noting that all of the example inputs use an even number of steps but the actual
//! problem specifies an odd number of steps.
//!
//! First, do a BFS to determine how many steps it takes to reach each position in the map from the starting position.
//! All steps on odd/even spaces (depending on input parity) are counted towards the answer.
//!
//! Next, count spaces touched in each direction moving horizontally or vertically from the center map:
//! - Start by determining how many spaces it takes to reach each space on the appropriate border (e.g. the right border
//!   for the maps to the left)
//! - Continue moving across maps in the same direction until a repeat is detected in how many steps it takes to reach
//!   each border space (normalized to the minimum to reach any space in the map). Count all odd/even spaces as
//!   appropriate along the way
//! - At this point, it is assumed that the first space entered is one of the corners, which means the elf will reach
//!   a new map in this direction every map_len steps. Compute how many maps in this direction the elf will reach
//! - Determine how many steps it takes to reach every space in the map from the repeated starting positions. For the
//!   last maps the elf will reach, count how many odd/even spaces the elf can reach given the number of steps
//!   remaining upon first entering that map, and continue moving backwards until there are enough remaining steps to
//!   reach every space in the map
//! - For the in-between maps where the elf can reach every space, simply count how many maps there are where the elf
//!   touches every odd space and how many there are where the elf touches every even space
//!
//! Oof. Finally, count spaces that the elf can reach moving across maps diagonally:
//! - Count how many steps it takes the elf to first reach the appropriate corner (e.g. the bottom-right corner for the
//!   first map to the top-left), which will always be 2 plus the number of steps to reach the opposite corner in the
//!   center map from the starting position
//! - Since the outermost rows and columns are empty, after every map_len steps, the elf will reach a new N+1 maps
//!   where N is the current number of maps on the diagonal. The elf starts in 1 map, then reaches 2 new maps after
//!   map_len steps, then reaches 3 new maps after another map_len steps, etc.
//! - Calculate how many maps out the elf will reach given the number of steps remaining upon first reaching the corner,
//!   and then perform a computation similar to what was done in the horizontal/vertical directions. For the last
//!   diagonals, count how many odd/even spaces the elf can reach with the number of steps remaining, and then multiply
//!   that by the number of maps along that diagonal
//! - Again, for the in-between maps where the elf can reach every space, count how many there are where the elf touches
//!   every odd space and how many where the elf touches every even space (only multiplying by the number of maps along
//!   the diagonal at each step)
//!
//! Very complicated, but it works and is fast enough (6-7 milliseconds on my computer).

use advent_of_code_2023::impl_main;
use std::cmp;
use std::collections::VecDeque;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    i: u32,
    j: u32,
}

impl Point {
    fn new(i: u32, j: u32) -> Self {
        Self { i, j }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Space {
    Garden,
    Rock,
}

#[derive(Debug, Clone)]
struct Input {
    map: Vec<Vec<Space>>,
    start: Point,
}

fn parse_input(input: &str) -> Input {
    let mut map: Vec<Vec<Space>> = Vec::new();
    let mut start: Option<Point> = None;
    for (i, line) in input.lines().enumerate() {
        let mut row = Vec::new();
        for (j, c) in line.chars().enumerate() {
            match c {
                '.' => row.push(Space::Garden),
                '#' => row.push(Space::Rock),
                'S' => {
                    row.push(Space::Garden);

                    assert!(start.is_none(), "Multiple start positions in input");
                    start = Some(Point::new(i as u32, j as u32));
                }
                _ => panic!("Invalid input char: {c}"),
            }
        }
        map.push(row);
    }

    Input { map, start: start.expect("No start position in map") }
}

const PART_1_STEPS: u32 = 64;

fn solve_part_1(input: &str) -> u32 {
    solve_part_1_inner(input, PART_1_STEPS)
}

fn solve_part_1_inner(input: &str, target_steps: u32) -> u32 {
    let Input { map, start } = parse_input(input);

    let (step_map, _) = build_step_map(
        &map,
        &[StartPosition { i: start.i as usize, j: start.j as usize, step: 0 }],
    );
    count_positions(&step_map, target_steps.into(), (target_steps % 2).into()) as u32
}

const PART_2_STEPS: u64 = 26_501_365;

fn solve_part_2(input: &str) -> u64 {
    solve_part_2_inner(input, PART_2_STEPS)
}

fn solve_part_2_inner(input: &str, target_steps: u64) -> u64 {
    let Input { map, start } = parse_input(input);

    let (center_step_map, _) = build_step_map(
        &map,
        &[StartPosition { i: start.i as usize, j: start.j as usize, step: 0 }],
    );

    // Center
    let mut count = count_positions(&center_step_map, target_steps, target_steps % 2);

    // Left
    count += count_edge(&map, &center_step_map, target_steps, |step_map| {
        (0..map.len())
            .map(|i| StartPosition { i, j: map.len() - 1, step: step_map[i][0] + 1 })
            .collect()
    });

    // Right
    count += count_edge(&map, &center_step_map, target_steps, |step_map| {
        (0..map.len())
            .map(|i| StartPosition { i, j: 0, step: step_map[i][map.len() - 1] + 1 })
            .collect()
    });

    // Up
    count += count_edge(&map, &center_step_map, target_steps, |step_map| {
        (0..map.len())
            .map(|j| StartPosition { i: map.len() - 1, j, step: step_map[0][j] + 1 })
            .collect()
    });

    // Down
    count += count_edge(&map, &center_step_map, target_steps, |step_map| {
        (0..map.len())
            .map(|j| StartPosition { i: 0, j, step: step_map[map.len() - 1][j] + 1 })
            .collect()
    });

    // Top left
    count += count_corner(&map, &center_step_map, target_steps, map.len() - 1, map.len() - 1);

    // Top right
    count += count_corner(&map, &center_step_map, target_steps, map.len() - 1, 0);

    // Bottom left
    count += count_corner(&map, &center_step_map, target_steps, 0, map.len() - 1);

    // Bottom right
    count += count_corner(&map, &center_step_map, target_steps, 0, 0);

    count
}

fn count_positions(step_map: &[Vec<u64>], step_limit: u64, step_modulo: u64) -> u64 {
    step_map
        .iter()
        .map(|row| {
            row.iter().filter(|&&steps| steps <= step_limit && steps % 2 == step_modulo).count()
                as u64
        })
        .sum()
}

fn count_edge(
    map: &[Vec<Space>],
    center_step_map: &[Vec<u64>],
    mut remaining_steps: u64,
    start_position_fn: impl Fn(&[Vec<u64>]) -> Vec<StartPosition>,
) -> u64 {
    let mut start_positions = start_position_fn(center_step_map);
    let initial_min_steps = find_min_step(&start_positions);
    if initial_min_steps > remaining_steps {
        return 0;
    }

    normalize_to_min_step(&mut start_positions, initial_min_steps);
    remaining_steps -= initial_min_steps;

    let mut count = 0;
    let mut step_modulo = remaining_steps % 2;
    loop {
        let (next_step_map, _) = build_step_map(map, &start_positions);
        count += count_positions(&next_step_map, remaining_steps, step_modulo);

        let mut next_start_positions = start_position_fn(&next_step_map);
        let min_steps = find_min_step(&next_start_positions);
        if min_steps > remaining_steps {
            return count;
        }

        normalize_to_min_step(&mut next_start_positions, min_steps);
        remaining_steps -= min_steps;
        step_modulo = step_modulo.wrapping_sub(min_steps) % 2;

        if next_start_positions == start_positions {
            // Loop detected; short circuit and only explicitly the last few where not the entire block is filled
            return count
                + count_edge_loop(map, &next_start_positions, remaining_steps, step_modulo);
        }

        start_positions = next_start_positions;
    }
}

fn find_min_step(start_positions: &[StartPosition]) -> u64 {
    start_positions.iter().map(|position| position.step).min().unwrap()
}

fn normalize_to_min_step(start_positions: &mut [StartPosition], min_step: u64) {
    for position in start_positions {
        position.step -= min_step;
    }
}

fn count_edge_loop(
    map: &[Vec<Space>],
    start_positions: &[StartPosition],
    remaining_steps: u64,
    step_modulo: u64,
) -> u64 {
    let (step_map, steps_to_fill) = build_step_map(map, start_positions);

    let even_full_count = count_positions(&step_map, (map.len() * map.len()) as u64, 0);
    let odd_full_count = count_positions(&step_map, (map.len() * map.len()) as u64, 1);

    let mut out_distance = remaining_steps / map.len() as u64;
    let mut count = 0_u64;
    let mut step_modulo = (step_modulo + out_distance * map.len() as u64) % 2;
    loop {
        let block_steps = remaining_steps - out_distance * map.len() as u64;
        if steps_to_fill <= block_steps {
            loop {
                count += if step_modulo == 0 { even_full_count } else { odd_full_count };
                step_modulo = (step_modulo + map.len() as u64) % 2;

                if out_distance == 0 {
                    return count;
                }
                out_distance -= 1;
            }
        }

        count += count_positions(&step_map, block_steps, step_modulo);

        if out_distance == 0 {
            return count;
        }
        out_distance -= 1;
        step_modulo = (step_modulo + map.len() as u64) % 2;
    }
}

fn count_corner(
    map: &[Vec<Space>],
    center_step_map: &[Vec<u64>],
    target_steps: u64,
    start_i: usize,
    start_j: usize,
) -> u64 {
    let distance_to_corner = center_step_map[map.len() - 1 - start_i][map.len() - 1 - start_j] + 2;
    if distance_to_corner > target_steps {
        return 0;
    }

    let corner_steps = target_steps - distance_to_corner;

    let full_even_count = count_positions(center_step_map, (map.len() * map.len()) as u64, 0);
    let full_odd_count = count_positions(center_step_map, (map.len() * map.len()) as u64, 1);

    let mut out_distance = 1 + corner_steps / map.len() as u64;
    let mut count = 0_u64;
    let mut step_modulo = (target_steps - (out_distance - 1) * map.len() as u64) % 2;
    while out_distance > 0 {
        let (step_map, steps_to_fill) =
            build_step_map(map, &[StartPosition { i: start_i, j: start_j, step: 0 }]);

        let remaining_steps = corner_steps - (out_distance - 1) * map.len() as u64;

        if steps_to_fill <= remaining_steps {
            while out_distance > 0 {
                count += out_distance
                    * (if step_modulo == 0 { full_even_count } else { full_odd_count });
                out_distance -= 1;
                step_modulo = (step_modulo + map.len() as u64) % 2;
            }

            return count;
        }

        count += out_distance * count_positions(&step_map, remaining_steps, step_modulo);

        step_modulo = (step_modulo + map.len() as u64) % 2;
        out_distance -= 1;
    }

    count
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct QueueEntry {
    i: usize,
    j: usize,
    steps: u64,
}

fn build_step_map(map: &[Vec<Space>], start_positions: &[StartPosition]) -> (Vec<Vec<u64>>, u64) {
    let mut step_map = vec![vec![u64::MAX; map.len()]; map.len()];

    let mut queue = VecDeque::new();
    for position in start_positions {
        queue.push_back(QueueEntry { i: position.i, j: position.j, steps: position.step });
    }

    let mut max_steps = u64::MIN;
    while let Some(QueueEntry { i, j, steps }) = queue.pop_front() {
        if step_map[i][j] < steps {
            continue;
        }

        step_map[i][j] = steps;
        max_steps = cmp::max(max_steps, steps);

        for (di, dj) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let new_i = i as i32 + di;
            let new_j = j as i32 + dj;

            if (0..map.len() as i32).contains(&new_i)
                && (0..map.len() as i32).contains(&new_j)
                && map[new_i as usize][new_j as usize] == Space::Garden
            {
                let new_i = new_i as usize;
                let new_j = new_j as usize;
                let new_steps = steps + 1;
                if step_map[new_i][new_j] > new_steps {
                    step_map[new_i][new_j] = new_steps;
                    queue.push_back(QueueEntry { i: new_i, j: new_j, steps: new_steps });
                }
            }
        }
    }

    (step_map, max_steps)
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct StartPosition {
    i: usize,
    j: usize,
    step: u64,
}

impl_main!(p1: solve_part_1, p2: solve_part_2);

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("../../sample_input/day21.txt");

    #[test]
    fn sample_input_part_1() {
        assert_eq!(solve_part_1_inner(SAMPLE_INPUT, 6), 16);
    }

    #[test]
    fn sample_input_part_2() {
        assert_eq!(solve_part_2_inner(SAMPLE_INPUT, 1), 2);
        assert_eq!(solve_part_2_inner(SAMPLE_INPUT, 3), 6);
        assert_eq!(solve_part_2_inner(SAMPLE_INPUT, 6), 16);
        assert_eq!(solve_part_2_inner(SAMPLE_INPUT, 7), 22);
        assert_eq!(solve_part_2_inner(SAMPLE_INPUT, 10), 50);
        assert_eq!(solve_part_2_inner(SAMPLE_INPUT, 50), 1594);
        assert_eq!(solve_part_2_inner(SAMPLE_INPUT, 100), 6536);
        assert_eq!(solve_part_2_inner(SAMPLE_INPUT, 500), 167004);
        assert_eq!(solve_part_2_inner(SAMPLE_INPUT, 1000), 668697);
        assert_eq!(solve_part_2_inner(SAMPLE_INPUT, 5000), 16733044);
    }
}
