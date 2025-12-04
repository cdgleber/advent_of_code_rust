use crate::util::{grid::Grid, point::DIAGONAL};

pub fn solve() {
    let input = include_str!("input/day04.txt");
    let mut grid = Grid::parse(input);
    let mut accessible = 0;

    loop {
        //added for part 2
        let mut at_least_one_removed = false;
        for p in grid.find_all(b'@').unwrap() {
            let mut count_rolls = 0;
            for m in DIAGONAL {
                let new_point = p + m;
                if grid.contains(new_point) {
                    if grid[new_point] == b'@' {
                        count_rolls += 1;
                    }
                }
            }
            if count_rolls < 4 {
                accessible += 1;
                grid[p] = b'.'; //added for part 2
                at_least_one_removed = true; //added for part 2
            }
        }
        if !at_least_one_removed {
            break;
        }
    }
    println!("{}", accessible);
}

const TEST: &str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";
