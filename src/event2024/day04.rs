use std::ops::Index;

use crate::util::grid::*;
use crate::util::point::*;

pub fn run_day04(input: &str) {
    let grid = Grid::parse(input);
    let mut number_of_xmas = 0;
    for spot in 0..grid.bytes.len() {
        if grid.bytes[spot] == b'X' {
            let start = grid.to_point(spot);
            let number_found = depth_first_search(start, true, &grid);
            println!("{}", number_found);
            number_of_xmas += number_found;
        }

        // if grid.bytes[spot] == b'S' {
        //     let start = grid.to_point(spot);
        //     println!("{:?}", start);
        //     if depth_first_search(start, false, &grid) {
        //         // println!("FOUND");
        //         number_of_xmas += 1;
        //     }
        // }
    }

    println!("{}", number_of_xmas);
}

fn depth_first_search(start: Point, forward: bool, grid: &Grid<u8>) -> i32 {
    //search XMAS forward
    let to_find = if forward {
        [b'M', b'A', b'S']
    } else {
        [b'A', b'M', b'X']
    };

    let mut found = 0;
    for movement in DIAGONAL {
        let mut found_next_letter: bool = false;

        for next_letter in &to_find {
            found_next_letter = false;
            let next_point = start + movement;

            if grid.contains(next_point) {
                let grid_letter = grid.index(next_point);

                print!("{}", *grid_letter as char);

                if next_letter == grid_letter {
                    found_next_letter = true;
                }
            }

            println!("{}", found_next_letter);

            if !found_next_letter {
                break;
            }
        }

        if found_next_letter {
            println!("{:?}, {:?}", start, movement);
            found += 1;
        }
    }

    found
}

pub const TESTD4E: &str = "SOOSOOS
OAOAOAO
OOMMMOO
SAMXMAS
OOMMMOO
OAOAOAO
SOOSOOS";

pub const TESTD4: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
