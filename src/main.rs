use advent_of_code_rust::event2024::*;
// use day01::run_day01;
// use day02::run_day02;
use day03::run_day03;

fn main() {
    // let day_01_notes = include_str!("../input/aoc_2024_d1.txt");
    // day01(&day_01_notes);

    // let day_02_notes = include_str!("../input/aoc_2024_d2.txt");
    // run_day02(&day_02_notes);

    let day_03_notes = include_str!("../input/aoc_2024_d3.txt");
    run_day03(&day_03_notes);
}

const TESTD3: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
const TESTD3p2: &str = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
