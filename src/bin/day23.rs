//! Day 23: A Long Walk
//!
//! <https://adventofcode.com/2023/day/23>

use advent_of_code_2023::impl_main;
use rustc_hash::{FxHashMap, FxHashSet};
use std::cmp;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    North,
    South,
    West,
    East,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Space {
    Path,
    Forest,
    Slope(Direction),
}

fn parse_input(input: &str) -> Vec<Vec<Space>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => Space::Path,
                    '#' => Space::Forest,
                    '^' => Space::Slope(Direction::North),
                    'v' => Space::Slope(Direction::South),
                    '<' => Space::Slope(Direction::West),
                    '>' => Space::Slope(Direction::East),
                    _ => panic!("Invalid input char: {c}"),
                })
                .collect()
        })
        .collect()
}

#[derive(Debug, Clone, Copy)]
struct Edge {
    node: usize,
    weight: u32,
}

#[derive(Debug, Clone)]
struct Graph {
    nodes: Vec<Vec<Edge>>,
}

fn create_graph(map: &[Vec<Space>]) -> Graph {
    let mut coordinates_to_node = FxHashMap::default();

    for (i, row) in map.iter().enumerate() {
        for (j, &space) in row.iter().enumerate() {
            if space == Space::Forest {
                continue;
            }

            let adjacent_count = [(-1, 0), (1, 0), (0, -1), (0, 1)]
                .into_iter()
                .filter(|&(di, dj)| {
                    let new_i = i as i32 + di;
                    let new_j = j as i32 + dj;
                    (0..map.len() as i32).contains(&new_i)
                        && (0..map[0].len() as i32).contains(&new_j)
                        && map[new_i as usize][new_j as usize] != Space::Forest
                })
                .count();
            if i == 0 || i == map.len() - 1 || adjacent_count > 2 {
                let node_id = coordinates_to_node.len();
                coordinates_to_node.insert((i, j), node_id);
            }
        }
    }

    let mut nodes: Vec<Vec<Edge>> = vec![vec![]; coordinates_to_node.len()];
    for (&(i, j), &node_id) in &coordinates_to_node {
        for (di, dj) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let mut new_i = i as i32 + di;
            let mut new_j = j as i32 + dj;
            if (0..map.len() as i32).contains(&new_i)
                && (0..map[0].len() as i32).contains(&new_j)
                && map[new_i as usize][new_j as usize] != Space::Forest
            {
                let mut visited = FxHashSet::default();
                visited.insert((i, j));

                let mut path_len = 1;
                while !coordinates_to_node.contains_key(&(new_i as usize, new_j as usize)) {
                    visited.insert((new_i as usize, new_j as usize));

                    for (di, dj) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                        let path_i = (new_i + di) as usize;
                        let path_j = (new_j + dj) as usize;
                        if !visited.contains(&(path_i, path_j))
                            && map[path_i][path_j] != Space::Forest
                        {
                            new_i = path_i as i32;
                            new_j = path_j as i32;
                            path_len += 1;
                            break;
                        }
                    }
                }

                let path_node_id =
                    *coordinates_to_node.get(&(new_i as usize, new_j as usize)).unwrap();
                nodes[node_id].push(Edge { node: path_node_id, weight: path_len });
            }
        }
    }

    Graph { nodes }
}

fn search(
    map: &[Vec<Space>],
    visited: &mut Vec<Vec<bool>>,
    i: usize,
    j: usize,
    end_col: usize,
    current_path_len: u32,
    max_path_len: &mut u32,
) {
    if visited[i][j] || map[i][j] == Space::Forest {
        return;
    }

    if i == map.len() - 1 && j == end_col {
        *max_path_len = cmp::max(*max_path_len, current_path_len);
        return;
    }

    visited[i][j] = true;

    match map[i][j] {
        Space::Path => {
            for (di, dj) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let new_i = i as i32 + di;
                let new_j = j as i32 + dj;

                if (0..map.len() as i32).contains(&new_i)
                    && (0..map[0].len() as i32).contains(&new_j)
                    && !visited[new_i as usize][new_j as usize]
                    && map[new_i as usize][new_j as usize] != Space::Forest
                {
                    // valid_count += 1;
                    search(
                        map,
                        visited,
                        new_i as usize,
                        new_j as usize,
                        end_col,
                        current_path_len + 1,
                        max_path_len,
                    );
                }
            }
        }
        Space::Slope(direction) => {
            let (di, dj) = match direction {
                Direction::North => (-1, 0),
                Direction::South => (1, 0),
                Direction::West => (0, -1),
                Direction::East => (0, 1),
            };

            // Assume a slope will never point towards a forest
            let new_i = (i as i32 + di) as usize;
            let new_j = (j as i32 + dj) as usize;
            search(map, visited, new_i, new_j, end_col, current_path_len + 1, max_path_len);
        }
        Space::Forest => unreachable!("moving onto a forest space early returns"),
    }

    visited[i][j] = false;
}

fn solve_part_1(input: &str) -> u32 {
    let map = parse_input(input);

    let start_col =
        map[0].iter().position(|&space| space == Space::Path).expect("No path in top row");
    let end_col = map[map.len() - 1]
        .iter()
        .position(|&space| space == Space::Path)
        .expect("No path in bottom row");

    let mut visited = vec![vec![false; map[0].len()]; map.len()];
    let mut max_path_len = u32::MIN;
    search(&map, &mut visited, 0, start_col, end_col, 0, &mut max_path_len);

    max_path_len
}

fn solve_part_2(input: &str) -> u32 {
    let map = parse_input(input);

    let graph = create_graph(&map);
    let mut max_path_len = 0;
    search_part_2(&graph, &mut vec![false; graph.nodes.len()], 0, 0, &mut max_path_len);

    max_path_len
}

fn search_part_2(
    graph: &Graph,
    visited: &mut [bool],
    node: usize,
    path_len: u32,
    max_path_len: &mut u32,
) {
    if node == graph.nodes.len() - 1 {
        *max_path_len = cmp::max(*max_path_len, path_len);
        return;
    }

    visited[node] = true;

    for &edge in &graph.nodes[node] {
        if !visited[edge.node] {
            search_part_2(graph, visited, edge.node, path_len + edge.weight, max_path_len);
        }
    }

    visited[node] = false;
}

impl_main!(p1: solve_part_1, p2: solve_part_2);

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("../sample/day23.txt");

    #[test]
    fn sample_input_part_1() {
        assert_eq!(solve_part_1(SAMPLE_INPUT), 94);
    }

    #[test]
    fn sample_input_part_2() {
        assert_eq!(solve_part_2(SAMPLE_INPUT), 154);
    }
}
