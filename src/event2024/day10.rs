use std::collections::{HashMap, HashSet};

use crate::util::{grid::Grid, parse::ParseByte, point::*};

pub fn run_day10(input: &str) {
    let grid = Grid::parse(input);
    let trailheads: Vec<Point> = grid.find_all(b'0').unwrap();
    let valid_movements = ORTHOGONAL;
    let adj_list: HashMap<Point, Vec<Point>> = make_adj_list(&grid, &valid_movements);

    // println!("{:?}", adj_list);
    let mut visited = HashSet::<Point>::new();
    println!("{:?}", dfs(&trailheads[0], &adj_list, &mut visited));
}

fn make_adj_list(grid: &Grid<u8>, valid_movements: &[Point]) -> HashMap<Point, Vec<Point>> {
    let mut adj_list: HashMap<Point, Vec<Point>> = HashMap::new();
    for (i, byte) in grid.bytes.iter().enumerate() {
        let start = grid.to_point(i).unwrap();
        let mut neighbors = Vec::<Point>::new();
        let valid_height: u8 = byte + 1;
        for mv in valid_movements {
            let next = start + *mv;
            if grid.contains(next) {
                let next_height = grid[next];
                if next_height == valid_height {
                    neighbors.push(next);
                }
            }
        }
        if neighbors.len() > 0 {
            adj_list.insert(start, neighbors);
        }
    }

    adj_list
}

fn dfs(
    node: &Point,
    adj_list: &HashMap<Point, Vec<Point>>,
    visited: &mut HashSet<Point>,
) -> Option<Point> {
    let mut furthest_point: Option<Point> = None; //FIND FURTHEST FAILING
    if !visited.contains(&node) {
        visited.insert(node.clone());
        furthest_point = Some(node.clone());
        if let Some(neighbors) = adj_list.get(node) {
            for next in neighbors {
                dfs(next, adj_list, visited);
            }
        }
    }
    furthest_point
}

pub const TESTD10: &str = "...0...
...1...
...2...
6543456
7.....7
8.....8
9.....9";
