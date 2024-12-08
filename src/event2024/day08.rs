use std::collections::HashSet;

use crate::util::{
    grid::Grid,
    hash::{FastSet, FastSetBuilder},
    point::{Point, ORIGIN},
};

pub fn run_day08(input: &str) {
    let grid = Grid::parse(input);
    let mut ant_found = FastSet::new();
    let mut antinodes = FastSet::new();

    for i in 0..grid.bytes.len() {
        if grid.bytes[i] != b'.' {
            if ant_found.contains(&grid.bytes[i]) {
                continue;
            }
            ant_found.insert(grid.bytes[i]);

            let mut temp_grid = grid.clone();
            let mut ants = FastSet::new();
            for ti in 0..temp_grid.bytes.len() {
                if temp_grid.bytes[ti] != grid.bytes[i] {
                    temp_grid.bytes[ti] = b'.';
                }
            }

            for ti in 0..temp_grid.bytes.len() {
                if temp_grid.bytes[ti] != b'.' {
                    if let Some(p) = temp_grid.to_point(ti) {
                        ants.insert(p);
                    }
                }
            }
            antinodes.extend(&ants);

            let unique_pairs = get_unique_pairs_from_set(ants);
            let mut mirrored_points = FastSet::new();
            for (one, two) in &unique_pairs {
                let mut mir = mirror_point(one, two);
                let mut prev_one = one.clone();

                // PART I only needs an if
                // if temp_grid.contains(mir) {
                //     mirrored_points.insert(mir);
                // }

                // PART II needs to continue until off the map
                while temp_grid.contains(mir) {
                    mirrored_points.insert(mir);
                    let prev_mir = mir.clone();
                    mir = mirror_point(&mir, &prev_one);
                    prev_one = prev_mir;
                }
            }
            // println!("{:?}", unique_pairs);
            // print_grid(&temp_grid, &mirrored_points);
            antinodes.extend(mirrored_points);
        }
    }
    // print_grid(&grid, &antinodes);
    println!("{:?}", antinodes.len());
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
                print!("#");
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

fn mirror_point(one: &Point, two: &Point) -> Point {
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

pub const TESTD8: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";
