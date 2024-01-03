//! Day 24: Never Tell Me The Odds
//!
//! <https://adventofcode.com/2023/day/24>
//!
//! Part 1: For each pair of hailstones, create a 2x3 matrix representing a system of linear equations where the two variables
//! are the times that the respective hailstones reach the intersection point (considering X/Y only). Use Gaussian
//! elimination to solve for the two times (or to determine that there is no solution) and count the intersection if both
//! times are positive and the X/Y intersection coordinates are within range.
//!
//! Part 2: Use some clever linear algebra to create a system of linear equations where the variables are the X/Y/Z
//! coordinates of the initial rock position and the rock velocity, then use Gaussian elimination to solve the equations.
//! Only the first 3 hailstones are considered because 2 pairs of hailstones are enough to provide the 6 equations
//! necessary to solve for 6 unknowns.

use advent_of_code_2023::impl_main;
use fixed::types::I64F64;
use fixed_macro::fixed;
use std::ops::{Add, Index, IndexMut, Mul, Neg, Sub};
use winnow::ascii::{newline, space1};
use winnow::combinator::{opt, separated, separated_pair};

use fixed::prelude::*;
use winnow::prelude::*;
use winnow::token::take_while;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Matrix<T, const M: usize, const N: usize>([[T; N]; M]);

impl<T, const M: usize, const N: usize> Matrix<T, M, N> {
    fn swap(&mut self, i: usize, j: usize) {
        self.0.swap(i, j);
    }
}

impl<T, const M: usize, const N: usize> Index<usize> for Matrix<T, M, N> {
    type Output = [T; N];

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<T, const M: usize, const N: usize> IndexMut<usize> for Matrix<T, M, N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl<T: Copy + Default + Add<Output = T>, const M: usize, const N: usize> Add for Matrix<T, M, N> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut result = [[T::default(); N]; M];

        for (i, result_row) in result.iter_mut().enumerate() {
            for (j, result_value) in result_row.iter_mut().enumerate() {
                *result_value = self[i][j] + rhs[i][j];
            }
        }

        Self(result)
    }
}

impl<T: Copy + Default + Sub<Output = T>, const M: usize, const N: usize> Sub for Matrix<T, M, N> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut result = [[T::default(); N]; M];

        for (i, result_row) in result.iter_mut().enumerate() {
            for (j, result_value) in result_row.iter_mut().enumerate() {
                *result_value = self[i][j] - rhs[i][j];
            }
        }

        Self(result)
    }
}

impl<T: Copy + Default + Mul<Output = T>, const M: usize, const N: usize> Mul<T>
    for Matrix<T, M, N>
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        let mut result = self.0;

        for (i, result_row) in result.iter_mut().enumerate() {
            for (j, result_value) in result_row.iter_mut().enumerate() {
                *result_value = rhs * self[i][j];
            }
        }

        Self(result)
    }
}

impl<
    T: Copy + Default + Add<Output = T> + Mul<Output = T>,
    const M: usize,
    const N: usize,
    const M2: usize,
> Mul<Matrix<T, N, M2>> for Matrix<T, M, N>
{
    type Output = Matrix<T, M, M2>;

    fn mul(self, rhs: Matrix<T, N, M2>) -> Self::Output {
        let mut result = [[T::default(); M2]; M];

        for (i, result_row) in result.iter_mut().enumerate() {
            for (j, result_value) in result_row.iter_mut().enumerate() {
                for k in 0..N {
                    *result_value = *result_value + self[i][k] * rhs[k][j];
                }
            }
        }

        Matrix(result)
    }
}

type Vector<T, const N: usize> = Matrix<T, N, 1>;
type Vector3<T> = Vector<T, 3>;

impl<T: Copy + Default, const N: usize> Vector<T, N> {
    fn new(arr: [T; N]) -> Self {
        let mut transposed = [[T::default(); 1]; N];
        for (&value, transposed_value) in arr.iter().zip(&mut transposed) {
            transposed_value[0] = value;
        }
        Self(transposed)
    }
}

impl<T: Copy> Vector3<T> {
    fn x(&self) -> T {
        self.0[0][0]
    }

    fn y(&self) -> T {
        self.0[1][0]
    }

    fn z(&self) -> T {
        self.0[2][0]
    }
}

impl<T: Copy + Default + Neg<Output = T>> Vector3<T> {
    fn skew_symmetric_matrix(&self) -> Matrix<T, 3, 3> {
        Matrix([
            [T::default(), -self.z(), self.y()],
            [self.z(), T::default(), -self.x()],
            [-self.y(), self.x(), T::default()],
        ])
    }
}

impl Vector3<I64F64> {
    fn round_to_i64(&self) -> Vector3<i64> {
        Vector3::new([
            i64::lossy_from(self.x().round()),
            i64::lossy_from(self.y().round()),
            i64::lossy_from(self.z().round()),
        ])
    }
}

#[derive(Debug, Clone)]
struct Hailstone {
    position: Vector3<i64>,
    velocity: Vector3<i64>,
}

fn parse_i64(input: &mut &str) -> PResult<i64> {
    take_while(1.., |c: char| c == '-' || c.is_ascii_digit()).parse_to().parse_next(input)
}

fn parse_coords(input: &mut &str) -> PResult<Vector3<i64>> {
    let coords: Vec<_> = separated(3, parse_i64, (',', space1)).parse_next(input)?;
    let &[x, y, z] = coords.as_slice() else { unreachable!("separated(3)") };
    Ok(Vector3::new([x, y, z]))
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

fn gauss_jordan<const M: usize, const N: usize>(matrix: &mut Matrix<I64F64, M, N>) {
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
    let mut matrix = Matrix([
        [a.velocity.x().into(), (-b.velocity.x()).into(), (b.position.x() - a.position.x()).into()],
        [a.velocity.y().into(), (-b.velocity.y()).into(), (b.position.y() - a.position.y()).into()],
    ]);

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

    let x = I64F64::from(b.position.x()) + n1 * I64F64::from(b.velocity.x());
    let y = I64F64::from(b.position.y()) + n1 * I64F64::from(b.velocity.y());

    Some((x, y))
}

fn solve_part_2(input: &str) -> i64 {
    let hailstones = parse_input.parse(input).expect("Invalid input");

    let rock_position = find_rock_position(&hailstones);

    rock_position.x() + rock_position.y() + rock_position.z()
}

fn generate_linear_equations(a: &Hailstone, b: &Hailstone) -> [[I64F64; 7]; 3] {
    // These 3 equations are of the form:
    //   a * px + b * py + c * pz + d * vx + e * vy + f * vz = g
    // Where px/py/pz are the coordinates of the initial rock position, vz/vy/vz are the coordinates of the rock
    // velocity, and a/b/c/d/e/f/g are the coefficients.
    //
    // There are a few tricks used to derive these equations. We start with the following vector equation where pr
    // is the initial rock position, vr is the rock velocity, p0 is hailstone 0's initial position, v0 is hailstone 0's
    // velocity, and t is the parameter:
    //   pr + vr * t = p0 + v0 * t
    //
    // This equation represents the fact that there must exist some time t where the rock is at the exact same position
    // as hailstone 0. It can be rewritten as:
    //   pr - p0 = -t * (vr - v0)
    //
    // Any vector multiplied by a scalar is another vector in either the same direction or the opposite direction with
    // a different magnitude, so this implies that (pr - p0) and (vr - v0) are parallel. The cross product of any two
    // parallel vectors is the zero vector:
    //   (pr - p0) x (vr - v0) = 0
    //
    // The cross product operation is distributive, so this can be rewritten as:
    //   pr x vr - pr x v0 - p0 x vr + p0 x v0 = 0
    //
    // Now imagine we did the same thing with hailstone 1 to derive a similar equation for hailstone 1's position and
    // velocity p1 and v1. We can set these equal to each other:
    //   pr x vr - pr x v0 - p0 x vr + p0 x v0 = pr x vr - pr x v1 - p1 x vr + p1 x v1
    //
    // The non-linear (pr x vr) term is common to both sides, so it cancels out and we're left with:
    //   p0 x v0 - pr x v0 - p0 x vr = p1 x v1 - pr x v1 - p1 x vr
    //
    // This can be rewritten as:
    //   p0 x v0 - p1 x v1 = pr x (v0 - v1) + (p0 - p1) x vr
    //
    // Cross product is anti-commutative, so this is equivalent to:
    //   p0 x v0 - p1 x v1 = (p0 - p1) x vr - (v0 - v1) x pr
    //
    // The cross products can be converted into matrix multiplications by converting the first vector in each expression
    // into its 3x3 skew-symmetric matrix form, which when multiplied out produces the following 3 equations:
    //   py * (v0 - v1).z + pz * -(v0 - v1).y + vy * -(p0 - p1).z + vz * (p0 - p1).y = (p0 x v0 - p1 x v1).x
    //   px * -(v0 - v1).z + pz * (v0 - v1).x + vx * (p0 - p1).z + vz * -(p0 - p1).x = (p0 x v0 - p1 x v1).y
    //   px * (v0 - v1).y + py * -(v0 - v1).x + vx * -(p0 - p1).y + vy * (p0 - p1).x = (p0 x v0 - p1 x v1).z

    let position_diff = a.position - b.position;
    let velocity_diff = a.velocity - b.velocity;

    let constant_vector = a.position.skew_symmetric_matrix() * a.velocity
        - b.position.skew_symmetric_matrix() * b.velocity;

    [
        [
            0,
            velocity_diff.z(),
            -velocity_diff.y(),
            0,
            -position_diff.z(),
            position_diff.y(),
            constant_vector.x(),
        ]
        .map(I64F64::from),
        [
            -velocity_diff.z(),
            0,
            velocity_diff.x(),
            position_diff.z(),
            0,
            -position_diff.x(),
            constant_vector.y(),
        ]
        .map(I64F64::from),
        [
            velocity_diff.y(),
            -velocity_diff.x(),
            0,
            -position_diff.y(),
            position_diff.x(),
            0,
            constant_vector.z(),
        ]
        .map(I64F64::from),
    ]
}

fn find_rock_position(hailstones: &[Hailstone]) -> Vector3<i64> {
    let h0 = &hailstones[0];
    let h1 = &hailstones[1];
    let h2 = &hailstones[2];
    let mut matrix = Matrix([[i64f64!(0); 7]; 6]);
    matrix.0[..3].copy_from_slice(&generate_linear_equations(h0, h1));
    matrix.0[3..].copy_from_slice(&generate_linear_equations(h0, h2));

    gauss_jordan(&mut matrix);

    assert_slice_is_zero(&matrix[5][..5]);
    assert_slice_is_zero(&matrix[4][..4]);
    assert_slice_is_zero(&matrix[3][..3]);
    assert_slice_is_zero(&matrix[2][..2]);
    assert_slice_is_zero(&matrix[1][..1]);

    let vz = matrix[5][6] / matrix[5][5];
    let vy = (matrix[4][6] - vz * matrix[4][5]) / matrix[4][4];
    let vx = (matrix[3][6] - vz * matrix[3][5] - vy * matrix[3][4]) / matrix[3][3];
    let pz =
        (matrix[2][6] - vz * matrix[2][5] - vy * matrix[2][4] - vx * matrix[2][3]) / matrix[2][2];
    let py = (matrix[1][6]
        - vz * matrix[1][5]
        - vy * matrix[1][4]
        - vx * matrix[1][3]
        - pz * matrix[1][2])
        / matrix[1][1];
    let px = (matrix[0][6]
        - vz * matrix[0][5]
        - vy * matrix[0][4]
        - vx * matrix[0][3]
        - pz * matrix[0][2]
        - py * matrix[0][1])
        / matrix[0][0];

    Vector3::new([px, py, pz]).round_to_i64()
}

fn assert_slice_is_zero(values: &[I64F64]) {
    assert!(values.iter().all(|&n| n.abs() < i64f64!(1.0e-3)));
}

impl_main!(p1: solve_part_1, p2: solve_part_2);

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("../../sample_input/day24.txt");

    #[test]
    fn sample_input_part_1() {
        assert_eq!(solve_part_1_inner(SAMPLE_INPUT, 7, 27), 2);
    }

    #[test]
    fn sample_input_part_2() {
        assert_eq!(solve_part_2(SAMPLE_INPUT), 47);
    }
}
