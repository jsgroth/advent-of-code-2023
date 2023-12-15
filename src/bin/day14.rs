//! Day 14: Parabolic Reflector Dish
//!
//! <https://adventofcode.com/2023/day/14>

use advent_of_code_2023::impl_main;
use rustc_hash::FxHashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Rock {
    None,
    Round,
    Cube,
}

fn parse_input(input: &str) -> Vec<Vec<Rock>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => Rock::None,
                    'O' => Rock::Round,
                    '#' => Rock::Cube,
                    _ => panic!("Invalid input char: {c}"),
                })
                .collect()
        })
        .collect()
}

fn solve_part_1(input: &str) -> u32 {
    let mut grid = parse_input(input);

    let rows = grid.len();
    let cols = grid[0].len();
    shift_north(&mut grid, rows, cols);

    count_north_weight(&grid)
}

fn solve_part_2(input: &str) -> u32 {
    let mut grid = parse_input(input);

    let mut recorded_grids: FxHashMap<Vec<Vec<Rock>>, u64> = FxHashMap::default();

    let rows = grid.len();
    let cols = grid[0].len();

    for cycle in 0.. {
        if let Some(&prev_cycle) = recorded_grids.get(&grid) {
            let target_cycle = prev_cycle + ((1_000_000_000 - prev_cycle) % (cycle - prev_cycle));
            return recorded_grids
                .iter()
                .find_map(|(grid, &cycle)| {
                    (cycle == target_cycle).then(|| count_north_weight(grid))
                })
                .expect("Should always find target cycle in map");
        }

        recorded_grids.insert(grid.clone(), cycle);

        shift_north(&mut grid, rows, cols);
        shift_west(&mut grid, rows, cols);
        shift_south(&mut grid, rows, cols);
        shift_east(&mut grid, rows, cols);
    }

    unreachable!("loop over 0_u64.. will never terminate organically")
}

fn shift_north(grid: &mut Vec<Vec<Rock>>, rows: usize, cols: usize) {
    let positions = (0..rows).flat_map(|i| (0..cols).map(move |j| (i, j)));
    shift(grid, positions, -1, 0);
}

fn shift_west(grid: &mut Vec<Vec<Rock>>, rows: usize, cols: usize) {
    let positions = (0..cols).flat_map(|j| (0..rows).map(move |i| (i, j)));
    shift(grid, positions, 0, -1);
}

fn shift_south(grid: &mut Vec<Vec<Rock>>, rows: usize, cols: usize) {
    let positions = (0..rows).rev().flat_map(|i| (0..cols).map(move |j| (i, j)));
    shift(grid, positions, 1, 0);
}

fn shift_east(grid: &mut Vec<Vec<Rock>>, rows: usize, cols: usize) {
    let positions = (0..cols).rev().flat_map(|j| (0..rows).map(move |i| (i, j)));
    shift(grid, positions, 0, 1);
}

fn shift(
    grid: &mut Vec<Vec<Rock>>,
    positions: impl Iterator<Item = (usize, usize)>,
    di: i32,
    dj: i32,
) {
    for (i, j) in positions {
        if grid[i][j] != Rock::Round {
            continue;
        }

        let mut ii = i as i32;
        let mut jj = j as i32;

        while (0..grid.len() as i32).contains(&(ii + di))
            && (0..grid[0].len() as i32).contains(&(jj + dj))
            && grid[(ii + di) as usize][(jj + dj) as usize] == Rock::None
        {
            ii += di;
            jj += dj;
        }

        grid[i][j] = Rock::None;
        grid[ii as usize][jj as usize] = Rock::Round;
    }
}

fn count_north_weight(grid: &[Vec<Rock>]) -> u32 {
    grid.iter()
        .enumerate()
        .map(|(i, row)| {
            row.iter()
                .filter_map(|&rock| (rock == Rock::Round).then_some(grid.len() - i))
                .sum::<usize>()
        })
        .sum::<usize>() as u32
}

impl_main!(p1: solve_part_1, p2: solve_part_2);

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("../sample/day14.txt");

    #[test]
    fn sample_input_part_1() {
        assert_eq!(solve_part_1(SAMPLE_INPUT), 136);
    }

    #[test]
    fn sample_input_part_2() {
        assert_eq!(solve_part_2(SAMPLE_INPUT), 64);
    }
}
