//! Day 24: Never Tell Me The Odds
//!
//! <https://adventofcode.com/2023/day/24>

use advent_of_code_2023::impl_main;
use fixed::types::I64F64;
use fixed_macro::fixed;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use winnow::ascii::{newline, space1};
use winnow::combinator::{opt, separated, separated_pair};

use fixed::prelude::*;
use winnow::prelude::*;
use winnow::token::take_while;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Coords {
    x: i64,
    y: i64,
    z: i64,
}

#[derive(Debug, Clone)]
struct Hailstone {
    position: Coords,
    velocity: Coords,
}

fn parse_i64(input: &mut &str) -> PResult<i64> {
    take_while(1.., |c: char| c == '-' || c.is_ascii_digit()).parse_to().parse_next(input)
}

fn parse_coords(input: &mut &str) -> PResult<Coords> {
    let coords: Vec<_> = separated(3, parse_i64, (',', space1)).parse_next(input)?;
    let &[x, y, z] = coords.as_slice() else { unreachable!("separated(3)") };
    Ok(Coords { x, y, z })
}

fn parse_hailstone(input: &mut &str) -> PResult<Hailstone> {
    let (position, velocity) =
        separated_pair(parse_coords, (space1, '@', space1), parse_coords).parse_next(input)?;
    Ok(Hailstone { position, velocity })
}

fn parse_input(input: &mut &str) -> PResult<Vec<Hailstone>> {
    let hailstones = separated(1.., parse_hailstone, newline).parse_next(input)?;
    opt(newline).parse_next(input)?;
    Ok(hailstones)
}

macro_rules! i64f64 {
    ($n:expr) => {
        fixed!($n: I64F64)
    }
}

fn gauss_jordan<const M: usize, const N: usize>(matrix: &mut [[I64F64; N]; M]) {
    let mut pivot_row = 0;
    let mut pivot_col = 0;
    while pivot_row < M && pivot_col < N {
        let i_max = (pivot_row..M).max_by_key(|&i| matrix[i][pivot_col].abs()).unwrap();
        if matrix[i_max][pivot_col].abs() < i64f64!(1.0e-3) {
            pivot_col += 1;
            continue;
        }

        matrix.swap(pivot_row, i_max);

        for i in pivot_row + 1..M {
            let scale = matrix[i][pivot_col] / matrix[pivot_row][pivot_col];
            matrix[i][pivot_col] = i64f64!(0);

            for j in pivot_col + 1..N {
                let pivot_value = matrix[pivot_row][j];
                matrix[i][j] -= pivot_value * scale;
            }
        }

        pivot_row += 1;
        pivot_col += 1;
    }
}

const PART_1_AREA_MIN: i64 = 200_000_000_000_000;
const PART_1_AREA_MAX: i64 = 400_000_000_000_000;

fn solve_part_1(input: &str) -> u32 {
    solve_part_1_inner(input, PART_1_AREA_MIN, PART_1_AREA_MAX)
}

fn solve_part_1_inner(input: &str, min_position: i64, max_position: i64) -> u32 {
    let hailstones = parse_input.parse(input).expect("Invalid input");

    let valid_range = I64F64::from(min_position)..=I64F64::from(max_position);
    let mut intersection_count = 0;
    for (i, hailstone_a) in hailstones.iter().enumerate() {
        for hailstone_b in hailstones.iter().skip(i + 1) {
            if find_2d_intersection(hailstone_a, hailstone_b)
                .is_some_and(|(x, y)| valid_range.contains(&x) && valid_range.contains(&y))
            {
                intersection_count += 1;
            }
        }
    }

    intersection_count
}

fn find_2d_intersection(a: &Hailstone, b: &Hailstone) -> Option<(I64F64, I64F64)> {
    // Given two lines defined using parametric equations:
    //   x = an + b
    //   y = cn + d
    // Set x and y equal to each other and solve for the two n values (a/b/c/d are known):
    //   a0 * n0 + b0 = a1 * n1 + b1
    //   c0 * n0 + d0 = c1 * n1 + d1
    // This can be rewritten as:
    //   a0 * n0 - a1 * n1 = b1 - b0
    //   c0 * n0 - c1 * n1 = d1 - d0
    let mut matrix = [
        [a.velocity.x.into(), (-b.velocity.x).into(), (b.position.x - a.position.x).into()],
        [a.velocity.y.into(), (-b.velocity.y).into(), (b.position.y - a.position.y).into()],
    ];

    gauss_jordan(&mut matrix);

    if matrix[1][0].abs() >= i64f64!(1.0e-3) || matrix[1][1].abs() < i64f64!(1.0e-3) {
        // No unique solution found (e.g. because lines are parallel)
        return None;
    }

    let n1 = matrix[1][2] / matrix[1][1];
    let n0 = (matrix[0][2] - n1 * matrix[0][1]) / matrix[0][0];
    if n0 <= i64f64!(0) || n1 <= i64f64!(0) {
        // Intersection is in the past of one of the hailstones
        return None;
    }

    let x = I64F64::from(b.position.x) + n1 * I64F64::from(b.velocity.x);
    let y = I64F64::from(b.position.y) + n1 * I64F64::from(b.velocity.y);

    Some((x, y))
}

fn solve_part_2(input: &str) -> i64 {
    const THREADS: usize = 16;

    let hailstones = parse_input.parse(input).expect("Invalid input");

    let done = Arc::new(AtomicBool::new(false));
    let mut handles = Vec::new();
    for initial_step in 0..THREADS as i64 {
        let hailstones = hailstones.clone();
        let done = Arc::clone(&done);
        handles.push(thread::spawn(move || {
            for max_velocity in (1 + initial_step..).step_by(THREADS) {
                if done.load(Ordering::Relaxed) {
                    return None;
                }

                for vx in -max_velocity..=max_velocity {
                    for vy in -max_velocity..=max_velocity {
                        for vz in -max_velocity..=max_velocity {
                            if vx.abs() < max_velocity
                                && vy.abs() < max_velocity
                                && vz.abs() < max_velocity
                            {
                                continue;
                            }

                            let rock_velocity = Coords { x: vx, y: vy, z: vz };
                            if let Some((x, y, z)) =
                                find_connecting_line(&hailstones, rock_velocity)
                            {
                                done.store(true, Ordering::Relaxed);
                                return Some(
                                    i64::lossy_from(x.round().int())
                                        + i64::lossy_from(y.round().int())
                                        + i64::lossy_from(z.round().int()),
                                );
                            }
                        }
                    }
                }
            }

            unreachable!("above loops will never run to completion")
        }));
    }

    for handle in handles {
        if let Some(solution) = handle.join().unwrap() {
            return solution;
        }
    }

    panic!("no solution found")
}

fn find_connecting_line(
    hailstones: &[Hailstone],
    rock_velocity: Coords,
) -> Option<(I64F64, I64F64, I64F64)> {
    // Create a matrix representing a system of equations of the following form:
    //   a*x + b*y + c*z + d*n0 + e*n1 + f*n2 = g
    // Fill with 9 equations (x/y/z for 3 hailstones) because using only 2 hailstones will give false positives
    let h0 = &hailstones[0];
    let h1 = &hailstones[1];
    let h2 = &hailstones[2];
    let mut matrix = [
        [
            i64f64!(1),
            i64f64!(0),
            i64f64!(0),
            (rock_velocity.x - h0.velocity.x).into(),
            i64f64!(0),
            i64f64!(0),
            h0.position.x.into(),
        ],
        [
            i64f64!(0),
            i64f64!(1),
            i64f64!(0),
            (rock_velocity.y - h0.velocity.y).into(),
            i64f64!(0),
            i64f64!(0),
            h0.position.y.into(),
        ],
        [
            i64f64!(0),
            i64f64!(0),
            i64f64!(1),
            (rock_velocity.z - h0.velocity.z).into(),
            i64f64!(0),
            i64f64!(0),
            h0.position.z.into(),
        ],
        [
            i64f64!(1),
            i64f64!(0),
            i64f64!(0),
            i64f64!(0),
            (rock_velocity.x - h1.velocity.x).into(),
            i64f64!(0),
            h1.position.x.into(),
        ],
        [
            i64f64!(0),
            i64f64!(1),
            i64f64!(0),
            i64f64!(0),
            (rock_velocity.y - h1.velocity.y).into(),
            i64f64!(0),
            h1.position.y.into(),
        ],
        [
            i64f64!(0),
            i64f64!(0),
            i64f64!(1),
            i64f64!(0),
            (rock_velocity.z - h1.velocity.z).into(),
            i64f64!(0),
            h1.position.z.into(),
        ],
        [
            i64f64!(1),
            i64f64!(0),
            i64f64!(0),
            i64f64!(0),
            i64f64!(0),
            (rock_velocity.x - h2.velocity.x).into(),
            h2.position.x.into(),
        ],
        [
            i64f64!(0),
            i64f64!(1),
            i64f64!(0),
            i64f64!(0),
            i64f64!(0),
            (rock_velocity.y - h2.velocity.y).into(),
            h2.position.y.into(),
        ],
        [
            i64f64!(0),
            i64f64!(0),
            i64f64!(1),
            i64f64!(0),
            i64f64!(0),
            (rock_velocity.z - h2.velocity.z).into(),
            h2.position.z.into(),
        ],
    ];

    gauss_jordan(&mut matrix);

    if !matrix[6].iter().all(|&n| n.abs() < i64f64!(1.0e-3)) {
        return None;
    }

    if !matrix[5][..5].iter().all(|&n| n.abs() < i64f64!(1.0e-3))
        || matrix[5][5].abs() < i64f64!(1.0e-3)
    {
        return None;
    }

    let n2 = matrix[5][6] / matrix[5][5];
    let n1 = (matrix[4][6] - n2 * matrix[4][5]) / matrix[4][4];
    let n0 = (matrix[3][6] - n2 * matrix[3][5] - n1 * matrix[3][4]) / matrix[3][3];
    if n0 <= i64f64!(0) || n1 <= i64f64!(0) || n2 <= i64f64!(0) {
        return None;
    }

    let rock_x = I64F64::from(h0.position.x) - n0 * I64F64::from(rock_velocity.x - h0.velocity.x);
    let rock_y = I64F64::from(h0.position.y) - n0 * I64F64::from(rock_velocity.y - h0.velocity.y);
    let rock_z = I64F64::from(h0.position.z) - n0 * I64F64::from(rock_velocity.z - h0.velocity.z);

    Some((rock_x, rock_y, rock_z))
}

impl_main!(p1: solve_part_1, p2: solve_part_2);

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("../sample/day24.txt");

    #[test]
    fn sample_input_part_1() {
        assert_eq!(solve_part_1_inner(SAMPLE_INPUT, 7, 27), 2);
    }

    #[test]
    fn sample_input_part_2() {
        assert_eq!(solve_part_2(SAMPLE_INPUT), 47);
    }
}
