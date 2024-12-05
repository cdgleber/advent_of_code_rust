use std::ops::Index;

use crate::util::grid::*;
use crate::util::point::*;

pub fn run_day04_p1(input: &str) {
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

pub fn run_day04_p2(input: &str) {
    let grid = Grid::parse(input);
    let mut number_of_xmas = 0;
    let mmss = [Some(&b'M'), Some(&b'M'), Some(&b'S'), Some(&b'S')];
    let smms = [Some(&b'S'), Some(&b'M'), Some(&b'M'), Some(&b'S')];
    let ssmm = [Some(&b'S'), Some(&b'S'), Some(&b'M'), Some(&b'M')];
    let mssm = [Some(&b'M'), Some(&b'S'), Some(&b'S'), Some(&b'M')];
    let all_matches = vec![mmss, smms, ssmm, mssm];

    for spot in 0..grid.bytes.len() {
        if grid.bytes[spot] == b'A' {
            let start = grid.to_point(spot).unwrap();

            let mut temp = Vec::new();
            for movement in XGONAL {
                let to_add = match grid.contains(start + movement) {
                    true => Some(grid.index(start + movement)),
                    false => None,
                };
                temp.push(to_add);
            }

            for compare in &all_matches {
                if temp.as_slice() == compare {
                    number_of_xmas += 1;
                    break;
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
