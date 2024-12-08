use std::collections::HashSet;

use crate::util::{
    grid::Grid,
    hash::{FastSet, FastSetBuilder},
    point::{Point, ORIGIN},
};

pub fn run_day08(input: &str) {
    let grid = Grid::parse(input);
    let mut ants = FastSet::new();

    for i in 0..grid.bytes.len() {
        if grid.bytes[i] == b'a' {
            if let Some(p) = grid.to_point(i) {
                ants.insert(p);
            }
        }
    }
}

fn get_unique_pairs_from_set(set: FastSet<Point>) -> FastSet<(Point, Point)> {
    let mut new_set = FastSet::new();
    for i in &set {
        for j in &set {
            if i == j {
                continue;
            }
            new_set.insert((i.clone(), j.clone()));
        }
    }
    new_set
}

fn print_grid(g: &Grid<u8>, s: &FastSet<Point>) {
    for i in 1..=g.bytes.len() {
        let x = i - 1;
        if let Some(point) = g.to_point(x) {
            if s.contains(&point) {
                print!("O");
            } else {
                print!("{}", g.bytes[x] as char);
            }
        } else {
            print!("{}", g.bytes[x] as char);
        }
        if (i as i32) % g.width == 0 && i > 0 {
            print!("\n");
        }
    }
    print!("\n");
}

fn mirror_point(one: Point, two: Point) -> Point {
    let mirror_x = one.x + (one.x - two.x);
    let mirror_y = one.y + (one.y - two.y);
    Point::new(mirror_x, mirror_y)
}

fn copy(grid: &Grid<u8>) -> Grid<Point> {
    Grid {
        width: grid.width,
        height: grid.height,
        bytes: vec![ORIGIN; (grid.width * grid.height) as usize],
    }
}

const TESTD8: &str = "..........
...#......
#.........
....a.....
........a.
.....a....
..#.......
......#...
..........
..........";
