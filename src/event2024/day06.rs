use crate::util::grid::*;
use crate::util::hash::*;
use crate::util::point::*;
use crate::util::thread::*;
use std::collections::HashSet;
use std::ops::Index;
use std::sync::atomic::{ AtomicUsize, Ordering };

pub fn parse(input: &str) -> Grid<u8> {
    Grid::parse(input)
}

pub fn run_day06_p1(input: &str) -> usize {
    let grid = Grid::parse(input);
    let mut visited_locations = HashSet::<Point>::new();
    let starting_pos = grid.find(b'^').unwrap();
    let mut current_pos = starting_pos;
    let mut movement = UP;
    loop {
        visited_locations.insert(current_pos);
        let next_pos = current_pos + movement;
        if grid.contains(next_pos) {
            let next_char = grid.index(next_pos);
            match next_char {
                b'#' => {
                    movement = movement.clockwise();
                }
                _ => {
                    current_pos = next_pos;
                }
            }
        } else {
            break;
        }
    }

    visited_locations.len()
}

/// Follow the guard's path, checking every step for a potential cycle.
pub fn run_day06_p2(input: &str) -> usize {
    let mut grid = Grid::parse(input);
    let mut position = grid.find(b'^').unwrap();
    let mut direction = UP;
    let mut path = Vec::with_capacity(5_000);

    while grid.contains(position + direction) {
        if grid[position + direction] == b'#' {
            direction = direction.clockwise();
        }

        let next = position + direction;

        // Avoid double counting when the path crosses itself.
        if grid[next] == b'.' {
            path.push((position, direction));
            grid[next] = b'^';
        }

        position = next;
    }

    // Use as many cores as possible to parallelize the remaining search.
    let shortcut = Shortcut::from(&grid);
    let total = AtomicUsize::new(0);

    spawn_batches(path, |batch| worker(&shortcut, &total, &batch));
    total.into_inner()
}

fn worker(shortcut: &Shortcut, total: &AtomicUsize, batch: &[(Point, Point)]) {
    let mut seen = FastSet::new();
    let result = batch
        .iter()
        .filter(|(position, direction)| {
            seen.clear();
            is_cycle(shortcut, &mut seen, *position, *direction)
        })
        .count();

    total.fetch_add(result, Ordering::Relaxed);
}

fn is_cycle(
    shortcut: &Shortcut,
    seen: &mut FastSet<(Point, Point)>,
    mut position: Point,
    mut direction: Point
) -> bool {
    let obstacle = position + direction;

    while shortcut.up.contains(position) {
        // Reaching the same position in the same direction is a cycle.
        if !seen.insert((position, direction)) {
            return true;
        }

        // The tricky part is checking for the new time travelling instigated obstacle.
        position = match direction {
            UP => {
                let next = shortcut.up[position];
                if position.x == obstacle.x && position.y > obstacle.y && obstacle.y >= next.y {
                    obstacle - UP
                } else {
                    next
                }
            }
            DOWN => {
                let next = shortcut.down[position];
                if position.x == obstacle.x && position.y < obstacle.y && obstacle.y <= next.y {
                    obstacle - DOWN
                } else {
                    next
                }
            }
            LEFT => {
                let next = shortcut.left[position];
                if position.y == obstacle.y && position.x > obstacle.x && obstacle.x >= next.x {
                    obstacle - LEFT
                } else {
                    next
                }
            }
            RIGHT => {
                let next = shortcut.right[position];
                if position.y == obstacle.y && position.x < obstacle.x && obstacle.x <= next.x {
                    obstacle - RIGHT
                } else {
                    next
                }
            }
            _ => unreachable!(),
        };

        direction = direction.clockwise();
    }

    false
}

struct Shortcut {
    up: Grid<Point>,
    down: Grid<Point>,
    left: Grid<Point>,
    right: Grid<Point>,
}

impl Shortcut {
    fn from(grid: &Grid<u8>) -> Self {
        let mut up = copy(grid);
        let mut down = copy(grid);
        let mut left = copy(grid);
        let mut right = copy(grid);

        for x in 0..grid.width {
            let mut last = Point::new(x, -1);

            for y in 0..grid.height {
                let point = Point::new(x, y);
                if grid[point] == b'#' {
                    last = Point::new(x, y + 1);
                }
                up[point] = last;
            }
        }

        for x in 0..grid.width {
            let mut last = Point::new(x, grid.height);

            for y in (0..grid.height).rev() {
                let point = Point::new(x, y);
                if grid[point] == b'#' {
                    last = Point::new(x, y - 1);
                }
                down[point] = last;
            }
        }

        for y in 0..grid.height {
            let mut last = Point::new(-1, y);

            for x in 0..grid.width {
                let point = Point::new(x, y);
                if grid[point] == b'#' {
                    last = Point::new(x + 1, y);
                }
                left[point] = last;
            }
        }

        for y in 0..grid.height {
            let mut last = Point::new(grid.width, y);

            for x in (0..grid.width).rev() {
                let point = Point::new(x, y);
                if grid[point] == b'#' {
                    last = Point::new(x - 1, y);
                }
                right[point] = last;
            }
        }

        Shortcut { up, down, left, right }
    }
}

fn copy(grid: &Grid<u8>) -> Grid<Point> {
    Grid {
        width: grid.width,
        height: grid.height,
        bytes: vec![ORIGIN; (grid.width * grid.height) as usize],
    }
}

pub const TESTD6: &str =
    "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
