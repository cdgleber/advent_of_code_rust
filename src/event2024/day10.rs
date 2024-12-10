use std::collections::{ HashMap, HashSet };
use crate::util::{ grid::Grid, parse::ParseByte, point::* };

pub fn run_day10(input: &str) {
    let grid = Grid::parse(input);
    let trailheads: Vec<Point> = grid.find_all(b'0').unwrap();
    let mut trailends: HashSet<Point> = HashSet::new();
    let valid_movements = ORTHOGONAL;
    let mut adj_list: HashMap<Point, Vec<Point>> = HashMap::new();

    for (i, byte) in grid.bytes.iter().enumerate() {
        let start = grid.to_point(i).unwrap();
        let mut neighbors = Vec::<Point>::new();
        let valid_height: u8 = byte + 1;
        for mv in valid_movements {
            let next = start + mv;
            if grid.contains(next) {
                let next_height = grid[next];
                if next_height == valid_height {
                    neighbors.push(next);
                }
                if next_height == b'9' {
                    trailends.insert(next);
                }
            }
        }
        if neighbors.len() > 0 {
            adj_list.insert(start, neighbors);
        }
    }

    let answer: usize = trailheads
        .iter()
        .map(|trailhead| {
            let mut visited = HashSet::<Point>::new();
            let mut trailends_visited = HashSet::<Point>::new();
            let trailends_found = dfs(
                &trailhead,
                &adj_list,
                &mut visited,
                &trailends,
                &mut trailends_visited,
                0u8
            );
            trailends_found as usize
        })
        .sum();

    println!("part 1: {}", answer);

    let answer: usize = trailheads
        .iter()
        .map(|trailhead| {
            let mut visited = HashSet::<Point>::new();
            let mut trailends_visited = HashSet::<Point>::new();
            let trailends_found = dfs_p2(
                &trailhead,
                &adj_list,
                &mut visited,
                &trailends,
                &mut trailends_visited,
                0u8
            );
            trailends_found as usize
        })
        .sum();

    println!("part 2: {}", answer);
}

fn dfs(
    node: &Point,
    adj_list: &HashMap<Point, Vec<Point>>,
    visited: &mut HashSet<Point>,
    trailends: &HashSet<Point>,
    trailends_visited: &mut HashSet<Point>,
    mut trailends_found: u8
) -> u8 {
    if !visited.contains(&node) {
        visited.insert(node.clone());
        if let Some(neighbors) = adj_list.get(node) {
            for next in neighbors {
                if trailends.contains(next) && !trailends_visited.contains(next) {
                    trailends_found += 1;
                    trailends_visited.insert(next.clone()); // PART ONE
                }
                trailends_found = dfs(
                    next,
                    adj_list,
                    visited,
                    trailends,
                    trailends_visited,
                    trailends_found
                );
            }
        }
    }
    trailends_found
}

fn dfs_p2(
    node: &Point,
    adj_list: &HashMap<Point, Vec<Point>>,
    visited: &mut HashSet<Point>,
    trailends: &HashSet<Point>,
    trailends_visited: &mut HashSet<Point>,
    mut trailends_found: u8
) -> u8 {
    if !visited.contains(&node) {
        if trailends.contains(node) {
            //clear the visited set so that overlaps can occur after backtracking
            visited.clear();
        } else {
            visited.insert(node.clone());
        }
        if let Some(neighbors) = adj_list.get(node) {
            for next in neighbors {
                if trailends.contains(next) && !trailends_visited.contains(next) {
                    trailends_found += 1;
                }
                trailends_found = dfs_p2(
                    next,
                    adj_list,
                    visited,
                    trailends,
                    trailends_visited,
                    trailends_found
                );
            }
        }
    }
    trailends_found
}

pub const TESTD10: &str = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

pub const TESTD10E: &str = "..90..9
...1.98
...2..7
6543456
765.987
876....
987....";

pub const TESTD10R: &str = "..90..9
...1.98
...2..7
6543456
765.987
876....
987....";
