use std::collections::{ HashMap, HashSet, VecDeque };

use crate::util::{ grid::*, point::* };

pub fn run_day12(input: &str) {
    let grid = Grid::parse(input);

    let connected = HashMap::<u8, Vec<(Point, bool)>>::new();
    let mut visited = HashSet::<Point>::new();
    let mut regions = HashMap::new();
    for (i, b) in grid.bytes.iter().enumerate() {
        let p = grid.to_point(i).unwrap();
        if !visited.contains(&p) {
            let mut region_points = HashSet::from([p]);
            dfs(&p, *b, &grid, &mut visited, &mut region_points);
            regions.insert((i, *b), region_points.clone());
            region_points.clear();
        }
    }

    let mut total_cost = 0usize;
    for ((i, b), region) in &regions {
        let area = region.len();
        let perimeters: usize = region
            .iter()
            .map(|p| count_perimeter(p, *b, &grid) as usize)
            .sum();

        // println!("{} : {} * {} = {}", b as char, area, perimeters, area * perimeters);
        total_cost += area * perimeters;
    }
    println!("part one: {total_cost}");

    // had to look up answer below. i got close but failed on the sides calcuations

    let mut todo = Vec::new();
    let mut edge = Vec::new();
    let mut seen = grid.same_size_with(false);

    let mut part_one = 0;
    let mut part_two = 0;

    for y in 0..grid.height {
        for x in 0..grid.width {
            // Skip already filled points.
            let point = Point::new(x, y);
            if seen[point] {
                continue;
            }

            // Flood fill, using area as an index.
            let kind = grid[point];
            let check = |point| grid.contains(point) && grid[point] == kind;

            let mut area = 0;
            let mut perimeter = 0;
            let mut sides = 0;

            todo.push(point);
            seen[point] = true;

            while area < todo.len() {
                let point = todo[area];
                area += 1;

                for direction in ORTHOGONAL {
                    let next = point + direction;

                    if check(next) {
                        if !seen[next] {
                            todo.push(next);
                            seen[next] = true;
                        }
                    } else {
                        edge.push((point, direction));
                        perimeter += 1;
                    }
                }
            }

            // Sum sides for all plots in the region.
            for &(p, d) in &edge {
                let r = d.clockwise();
                let l = d.counter_clockwise();

                sides += (!check(p + l) || check(p + l + d)) as usize;
                sides += (!check(p + r) || check(p + r + d)) as usize;
            }

            todo.clear();
            edge.clear();

            part_one += area * perimeter;
            part_two += area * (sides / 2);
        }
    }

    let answer = (part_one, part_two);
    println!("{:?}", answer);
}

fn dfs(
    p: &Point,
    region: u8,
    grid: &Grid<u8>,
    visited: &mut HashSet<Point>,
    region_points: &mut HashSet<Point>
) {
    if !visited.contains(&p) {
        visited.insert(p.clone());
        for next in ORTHOGONAL.map(|o| *p + o) {
            if grid.contains(next) && !visited.contains(&next) && grid[next] == region {
                region_points.insert(next.clone());
                dfs(&next, region, grid, visited, region_points);
            }
        }
    }
}

fn dfs_edges_only(
    p: &Point,
    b: u8,
    grid: &Grid<u8>,
    visited: &mut HashSet<Point>,
    sides: &mut HashSet<Point>
) {
    if !visited.contains(&p) {
        visited.insert(p.clone());
        for next in ORTHOGONAL.map(|o| *p + o) {
            if
                grid.contains(next) &&
                !visited.contains(&next) &&
                grid[next] == b &&
                is_edge(&next, b, grid)
            {
                dfs(&next, b, grid, visited, sides);
            }
        }
    }
}

fn count_perimeter(p: &Point, b: u8, grid: &Grid<u8>) -> u8 {
    let mut perimeters = 0u8;
    for next in ORTHOGONAL.map(|o| *p + o) {
        if grid.contains(next) {
            if b != grid[next] {
                perimeters += 1;
            }
        } else {
            perimeters += 1;
        }
    }
    perimeters
}

fn is_edge(p: &Point, b: u8, grid: &Grid<u8>) -> bool {
    for next in ORTHOGONAL.map(|o| *p + o) {
        if grid.contains(next) {
            if b != grid[next] {
                return true;
            }
        } else {
            return true;
        }
    }
    false
}

fn is_vertex(p: &Point, b: u8, grid: &Grid<u8>) -> bool {
    let mut perimeters = 0u8;
    for next in ORTHOGONAL.map(|o| *p + o) {
        if grid.contains(next) {
            if b != grid[next] {
                perimeters += 1;
            }
        } else {
            perimeters += 1;
        }
    }
    perimeters > 1
}

pub const TESTD12: &str = "AAAA
BBCD
BBCC
EEEC";

pub const TESTD12P2: &str =
    "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";
