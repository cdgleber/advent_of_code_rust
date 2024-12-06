use advent_of_code_rust::event2024::*;
use day05::{ run_day05_p1, run_day05_p2, TESTD5 };
use day06::{ run_day06_p2, TESTD6 };
// use day01::run_day01;
// use day02::run_day02;
// use day03::run_day03;
// use day04::{run_day04_p1, run_day04_p2};

fn main() {
    // let day_01_notes = include_str!("../input/aoc_2024_d1.txt");
    // day01(&day_01_notes);

    // let day_02_notes = include_str!("../input/aoc_2024_d2.txt");
    // run_day02(&day_02_notes);

    // let day_03_notes = include_str!("../input/aoc_2024_d3.txt");
    // run_day03(&day_03_notes);

    // let day_04_notes = include_str!("../input/aoc_2024_d4.txt");
    // run_day04_p2(&day_04_notes);

    // let day_05_notes = include_str!("../input/aoc_2024_d5.txt");
    // run_day05_p2(&day_05_notes);

    let day_06_notes = include_str!("../input/aoc_2024_d6.txt");
    // run_day06_p2(&day_06_notes);
    println!("{}", run_day06_p2(day_06_notes));
}
