use std::{collections::HashSet, fmt::Display, ops::Index};

use crate::util::{
    grid::Grid,
    point::{Point, DOWN, LEFT, RIGHT, UP},
};

pub fn run_day06_p1(input: &str) {
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
                _ => current_pos = next_pos,
            }
        } else {
            break;
        }
    }

    println!("{}", visited_locations.len());
}

pub fn run_day06_p2(input: &str) {
    let grid = Grid::parse(input);
    let mut potential_blocks = HashSet::<Point>::new();
    let starting_pos = grid.find(b'^').unwrap();
    let mut current_pos = starting_pos;
    let mut movement = UP;
    let mut turn_count = 0u8;
    let mut first_turn_pos = starting_pos;

    loop {
        let next_pos = current_pos + movement;
        if grid.contains(next_pos) {
            let next_char = grid.index(next_pos);
            match next_char {
                b'#' => {
                    if turn_count == 0 {
                        first_turn_pos = current_pos;
                    }

                    movement = movement.clockwise();
                    turn_count += 1;

                    if turn_count == 2 {
                        let last_corner = match movement {
                            LEFT | RIGHT => Point::new(first_turn_pos.x, current_pos.y),
                            UP | DOWN => Point::new(current_pos.x, first_turn_pos.y),
                            _ => unreachable!(),
                        };
                        let potential_block = last_corner + movement;
                        potential_blocks.insert(potential_block);
                        turn_count = 0;
                    }
                }
                _ => current_pos = next_pos,
            }
        } else {
            potential_blocks.insert(current_pos);
            break;
        }
    }

    print_grid(grid);

    println!("{}", potential_blocks.len());
}

fn print_grid(g: Grid<u8>) {
    for i in 0..g.bytes.len() {
        print!("{}", g.bytes[i] as char);
        if i as i32 % g.width == 0 && i > 0 {
            print!("\n");
        }
    }
    print!("\n");
}

pub const TESTD6: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
