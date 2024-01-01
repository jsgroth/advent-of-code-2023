//! Day 22: Sand Slabs
//!
//! <https://adventofcode.com/2023/day/22>
//!
//! Part 1: A brick can be dropped if it is oriented along the X or Y axis and every space below one of its blocks is
//! empty, or if it is oriented along the Z axis and the space below its lowest block is empty. All spaces at Z=0 are
//! treated as non-empty.
//!
//! This solution checks each brick individually after dropping the bricks as far as possible: Clone the map, delete
//! the blocks for the given brick, and then see if any other bricks can be dropped.
//!
//! Open/full spaces are tracked using a simple 3D grid.
//!
//! Part 2: Basically the same as part 1, but sum up the number of bricks that drop after each disintegration instead of
//! counting the number of bricks that cause no other bricks to drop.
//!
use advent_of_code_2023::impl_main;
use rustc_hash::FxHashSet;
use std::cmp;
use std::ops::{Add, Sub};
use winnow::ascii::{digit1, newline};
use winnow::combinator::{opt, separated, separated_pair};

use winnow::prelude::*;

#[derive(Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

impl Point {
    fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self { x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z }
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self { x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z }
    }
}

#[derive(Debug, Clone)]
struct Map(Vec<Vec<Vec<bool>>>);

impl Map {
    fn create(bricks: &[Brick]) -> Self {
        let mut max_x = i32::MIN;
        let mut max_y = i32::MIN;
        let mut max_z = i32::MIN;
        for brick in bricks {
            max_x = cmp::max(max_x, cmp::max(brick.0.x, brick.1.x));
            max_y = cmp::max(max_y, cmp::max(brick.0.y, brick.1.y));
            max_z = cmp::max(max_z, cmp::max(brick.0.z, brick.1.z));
        }

        let mut map = Self(vec![
            vec![vec![false; (max_z + 1) as usize]; (max_y + 1) as usize];
            (max_x + 1) as usize
        ]);

        for brick in bricks {
            brick.for_each_point(|point| map.set(point, true));
        }

        map
    }

    fn get(&self, point: Point) -> bool {
        self.0[point.x as usize][point.y as usize][point.z as usize]
    }

    fn set(&mut self, point: Point, value: bool) {
        self.0[point.x as usize][point.y as usize][point.z as usize] = value;
    }
}

#[derive(Debug, Clone)]
struct Brick(Point, Point);

impl Brick {
    fn for_each_point<T, F>(&self, mut f: F)
    where
        F: FnMut(Point) -> T,
    {
        if self.0.x != self.1.x {
            let min_x = cmp::min(self.0.x, self.1.x);
            let max_x = cmp::max(self.0.x, self.1.x);
            for x in min_x..=max_x {
                f(Point::new(x, self.0.y, self.0.z));
            }
        } else if self.0.y != self.1.y {
            let min_y = cmp::min(self.0.y, self.1.y);
            let max_y = cmp::max(self.0.y, self.1.y);
            for y in min_y..=max_y {
                f(Point::new(self.0.x, y, self.0.z));
            }
        } else {
            let min_z = cmp::min(self.0.z, self.1.z);
            let max_z = cmp::max(self.0.z, self.1.z);
            for z in min_z..=max_z {
                f(Point::new(self.0.x, self.0.y, z));
            }
        }
    }

    fn can_drop(&self, map: &Map) -> bool {
        if self.0.z == 1 || self.1.z == 1 {
            // Brick is on the ground
            return false;
        }

        if self.0.x == self.1.x && self.0.y == self.1.y {
            // Brick is along z-axis
            let min_z = cmp::min(self.0.z, self.1.z);
            !map.get(Point::new(self.0.x, self.0.y, min_z) - Point::new(0, 0, 1))
        } else {
            // Brick is along x-axis or y-axis; check all points below
            let mut space_below = true;
            self.for_each_point(|point| {
                space_below &= !map.get(point - Point::new(0, 0, 1));
            });
            space_below
        }
    }

    fn drop(&mut self) {
        self.0.z -= 1;
        self.1.z -= 1;
    }
}

fn parse_i32(input: &mut &str) -> PResult<i32> {
    digit1.parse_to().parse_next(input)
}

fn parse_point(input: &mut &str) -> PResult<Point> {
    let coordinates: Vec<_> = separated(3, parse_i32, ',').parse_next(input)?;
    let &[x, y, z] = coordinates.as_slice() else { unreachable!("separated(3)") };
    Ok(Point { x, y, z })
}

fn parse_brick(input: &mut &str) -> PResult<Brick> {
    let (p1, p2) = separated_pair(parse_point, '~', parse_point).parse_next(input)?;
    Ok(Brick(p1, p2))
}

fn parse_input(input: &mut &str) -> PResult<Vec<Brick>> {
    let bricks = separated(1.., parse_brick, newline).parse_next(input)?;
    opt(newline).parse_next(input)?;
    Ok(bricks)
}

fn solve_part_1(input: &str) -> u32 {
    let mut bricks = parse_input.parse(input).expect("Invalid input");

    let mut map = Map::create(&bricks);

    drop_bricks(&mut bricks, &mut map);

    let mut count = 0;
    for brick in &bricks {
        let mut disintegrated_map = map.clone();
        brick.for_each_point(|point| disintegrated_map.set(point, false));

        if drop_bricks(&mut bricks.clone(), &mut disintegrated_map) == 0 {
            count += 1;
        }
    }

    count
}

fn drop_bricks(bricks: &mut [Brick], map: &mut Map) -> u32 {
    let mut dropped_bricks = FxHashSet::default();

    loop {
        let mut dropped_any = false;
        for (i, brick) in bricks.iter_mut().enumerate() {
            if !brick.can_drop(map) {
                continue;
            }

            dropped_bricks.insert(i);
            dropped_any = true;

            brick.for_each_point(|point| map.set(point, false));

            while brick.can_drop(map) {
                brick.drop();
            }

            brick.for_each_point(|point| map.set(point, true));
        }

        if !dropped_any {
            break;
        }
    }

    dropped_bricks.len() as u32
}

fn solve_part_2(input: &str) -> u32 {
    let mut bricks = parse_input.parse(input).expect("Invalid input");

    let mut map = Map::create(&bricks);

    drop_bricks(&mut bricks, &mut map);

    let mut count = 0;
    for brick in &bricks {
        let mut disintegrated_map = map.clone();
        brick.for_each_point(|point| disintegrated_map.set(point, false));

        count += drop_bricks(&mut bricks.clone(), &mut disintegrated_map);
    }

    count
}

impl_main!(p1: solve_part_1, p2: solve_part_2);

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("../../sample_input/day22.txt");

    #[test]
    fn sample_input_part_1() {
        assert_eq!(solve_part_1(SAMPLE_INPUT), 5);
    }

    #[test]
    fn sample_input_part_2() {
        assert_eq!(solve_part_2(SAMPLE_INPUT), 7);
    }
}
