use std::{ collections::HashSet, fmt::Display, ops::Index };

use crate::util::{ grid::Grid, point::{ Point, DOWN, LEFT, RIGHT, UP } };

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
                _ => {
                    current_pos = next_pos;
                }
            }
        } else {
            break;
        }
    }

    println!("{}", visited_locations.len());
}

pub fn run_day06_p2(input: &str) {
    let grid = Grid::parse(input);
    let mut visited_locations = HashSet::<Point>::new();
    let mut potential_blocks: HashSet<Point> = HashSet::<Point>::new();
    let mut all_blocks: HashSet<Point> = HashSet::new();

    for (index, byte) in grid.bytes.iter().enumerate() {
        if *byte == b'#' {
            all_blocks.insert(grid.to_point(index).unwrap());
        }
    }

    // println!("{:?}", all_blocks);
    // println!("{:?}", collect_all_x(&all_blocks));

    let starting_pos = grid.find(b'^').unwrap();
    let mut current_pos = starting_pos;
    let mut movement = UP;
    let mut turn_count = 0u8;
    let mut last_turn_point: Option<Point> = None;

    loop {
        visited_locations.insert(current_pos);
        let next_pos = current_pos + movement;
        if grid.contains(next_pos) {
            let next_char = grid.index(next_pos);
            match next_char {
                b'#' => {
                    // print_grid(&grid, &visited_locations);

                    println!("{} {:?} {:?}", turn_count, movement, last_turn_point);

                    if turn_count > 2 && last_turn_point.is_some() {
                        match movement {
                            LEFT => {
                                let all_block_x = collect_all_x(
                                    &all_blocks,
                                    current_pos.y,
                                    movement
                                );

                                println!(
                                    "{:?} ; {} to {}",
                                    all_block_x,
                                    last_turn_point.unwrap().x,
                                    current_pos.x
                                );

                                for x in current_pos.x..last_turn_point.unwrap().x {
                                    let mut potential_x = Point::new(x, current_pos.y);
                                    println!("{:?} {:?}", current_pos, potential_x);
                                    if all_block_x.contains(&potential_x.x) {
                                        potential_x += movement;
                                        if !all_blocks.contains(&potential_x) {
                                            potential_blocks.insert(potential_x + movement);
                                        }
                                    }
                                }

                                println!("{:?}", potential_blocks);
                            }
                            // RIGHT => {
                            //     let all_block_x = collect_all_x(
                            //         &all_blocks,
                            //         current_pos.y,
                            //         movement
                            //     );

                            //     println!(
                            //         "{:?} ; {} to {}",
                            //         all_block_x,
                            //         last_corner.x,
                            //         current_pos.x
                            //     );

                            //     for x in last_corner.x..current_pos.x {
                            //         if all_block_x.contains(&x) {
                            //             potential_blocks.insert(Point::new(x, current_pos.y));
                            //         }
                            //     }
                            // }
                            // UP => {
                            //     let all_block_y = collect_all_y(
                            //         &all_blocks,
                            //         current_pos.x,
                            //         movement
                            //     );

                            //     println!(
                            //         "{:?} ; {} to {}",
                            //         all_block_y,
                            //         last_corner.y,
                            //         current_pos.y
                            //     );

                            //     for y in last_corner.y..current_pos.y {
                            //         if all_block_y.contains(&y) {
                            //             println!("{:?}", Point::new(current_pos.x, y));
                            //             potential_blocks.insert(Point::new(current_pos.x, y));
                            //         }
                            //     }
                            // }
                            // DOWN => {
                            //     let all_block_y = collect_all_y(
                            //         &all_blocks,
                            //         current_pos.x,
                            //         movement
                            //     );

                            //     println!(
                            //         "{:?} ; {} to {}",
                            //         all_block_y,
                            //         last_corner.y,
                            //         current_pos.y
                            //     );

                            //     for y in last_corner.y..current_pos.y {
                            //         if all_block_y.contains(&y) {
                            //             potential_blocks.insert(Point::new(current_pos.x, y));
                            //         }
                            //     }
                            // }
                            _ => (),
                        }
                    }
                    movement = movement.clockwise();
                    turn_count += 1;
                    last_turn_point = Some(current_pos);
                }
                _ => {
                    current_pos = next_pos;
                }
            }
        } else {
            potential_blocks.insert(current_pos);
            break;
        }
    }

    print_grid(&grid, &potential_blocks);

    println!("{}", potential_blocks.len());
}

fn collect_all_x(s: &HashSet<Point>, y: i32, dir: Point) -> HashSet<i32> {
    s.iter()
        .filter(|p| if dir == LEFT { p.y < y } else { p.y > y })
        .map(|p| p.x)
        .collect()
}

fn collect_all_y(s: &HashSet<Point>, x: i32, dir: Point) -> HashSet<i32> {
    s.iter()
        .filter(|p| if dir == DOWN { p.x < x } else { p.x > x })
        .map(|p| p.y)
        .collect()
}

fn print_grid(g: &Grid<u8>, s: &HashSet<Point>) {
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
