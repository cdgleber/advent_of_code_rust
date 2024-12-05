use std::ops::Index;

use crate::util::grid::*;
use crate::util::point::*;

pub fn run_day04(input: &str) {
    let grid = Grid::parse(input);
    let mut number_of_xmas = 0;
    let needle = [Some(&b'M'), Some(&b'A'), Some(&b'S')];
    for spot in 0..grid.bytes.len() {
        if grid.bytes[spot] == b'X' {
            let start = grid.to_point(spot).unwrap();

            for movement in DIAGONAL {
                let first = match grid.contains(start + movement) {
                    true => Some(grid.index(start + movement)),
                    false => None,
                };
                let second = match grid.contains(start + movement * 2) {
                    true => Some(grid.index(start + movement * 2)),
                    false => None,
                };
                let third = match grid.contains(start + movement * 3) {
                    true => Some(grid.index(start + movement * 3)),
                    false => None,
                };

                let haystack = [first, second, third];

                // println!("{:?}, {:?}", haystack, needle);

                if haystack == needle {
                    number_of_xmas += 1;
                }
            }
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

// fn depth_first_search(start: Point, forward: bool, grid: &Grid<u8>) -> i32 {
//     //search XMAS forward
//     let to_find = if forward {
//         [b'M', b'A', b'S']
//     } else {
//         [b'A', b'M', b'X']
//     };

//     let mut found = 0;
//     for movement in DIAGONAL {
//         let mut found: bool = false;

//         let index_movement = grid.width * movement.y + movement.x;

//
//         let next_letters = [];

//         if found {
//             println!("{:?}, {:?}", start, movement);
//             found += 1;
//         }
//     }

//     found
// }

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
