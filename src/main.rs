use advent_of_code_rust::event2024::*;
// use day01::run_day01;
use day02::run_day02;

fn main() {
    // let day_01_notes = include_str!("../input/aoc_2024_d1.txt");
    // day01(&day_01_notes);

    let day_02_notes = TEST;

    let day02 = run_day02(TEST);
}

const TEST: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
